use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::sync::{Arc, Mutex};

use ::anyhow::anyhow;
use ::chrono::{DateTime, Duration, DurationRound, NaiveDateTime, Utc};
use ::futures::StreamExt;
use ::octocrab::Octocrab;
use ::octocrab::models::AssetId;
use ::octocrab::models::repos::{Asset, Release};
use ::octocrab::repos::RepoHandler;
use ::secrecy::SecretString;
use ::serde::Deserialize;
use ::serde_json::Value as JsonValue;
use ::serde_with::{TryFromInto, serde_as};
use ::tokio::sync::broadcast;
use ::tokio::task::{JoinHandle, JoinSet};
use ::tokio::{task, time};
use ::tracing::{Instrument, info, info_span};
use ::url::Url;
use ::zip::ZipArchive;

use crate::domains::vulnerabilities::{VulnerabilityEvent, VulnerabilityFeed};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GithubConfig {
    pub access_token: SecretString,
}

pub struct Github {
    client: Octocrab,
    task: Arc<Mutex<Option<JoinHandle<()>>>>,
    tx: broadcast::Sender<CveRecord>,
    rx: broadcast::Receiver<CveRecord>,
}

impl Github {
    pub async fn new(config: GithubConfig) -> anyhow::Result<Self> {
        use secrecy::ExposeSecret;

        let GithubConfig { access_token } = config;

        let client = Octocrab::builder()
            .personal_token(access_token.expose_secret())
            .build()?;

        let (tx, rx) = broadcast::channel(500 * 1024);
        let task = Arc::new(Mutex::new(None));

        Ok(Self { client, tx, rx, task })
    }

    /// Starts the underlying task
    pub fn start(&self) {
        let Self { client, task, tx, .. } = self;
        let client = Octocrab::clone(client);
        let tx = broadcast::Sender::clone(tx);

        let mut task = task.lock().unwrap();
        *task = Some(tokio::spawn(
            async move {
                loop {
                    tracing::info!("started to poll vulnerabilities");
                    match Self::poll(&client, true).await {
                        Ok(records) => {
                            tracing::info!(n = records.len(), "finished polling vulnerabilities");

                            for record in records {
                                tx.send(record).unwrap();
                            }
                        }
                        Err(err) => {
                            tracing::error!(?err, "failed polling vulnerabilities")
                        }
                    };

                    let (next, until) = Self::next_poll();
                    tracing::info!(ts = ?next, "scheduled next polling");
                    time::sleep(until.to_std().unwrap()).await;
                }
            }
            .instrument(info_span!("github::task::poll")),
        ));
    }

    pub fn listen(&self) -> broadcast::Receiver<CveRecord> {
        self.tx.subscribe()
    }

    #[tracing::instrument(skip(client))]
    async fn poll(client: &Octocrab, all: bool) -> anyhow::Result<Vec<CveRecord>> {
        tracing::info!("started to retrieve releases page");
        let repo = client.repos("cveproject", "cvelistv5");
        let releases = repo.releases().list().per_page(100).send().await.unwrap();
        tracing::info!("successfully retrieved releases page");

        if let Some(release) = releases.items.first() {
            let Release { assets, .. } = release;

            let asset_name = if all { "_all_" } else { "_delta_" };
            let asset = match assets.iter().find(|asset| asset.name.contains(asset_name)) {
                Some(asset) => asset,
                None => {
                    tracing::error!("failed to find asset");
                    return Err(anyhow!("failed to find asset"));
                }
            };

            let asset_contents = match Self::get_asset_contents(&repo, asset.id).await {
                Ok(contents) => contents,
                Err(err) => {
                    tracing::error!("failed to get asset contents");
                    return Err(err);
                }
            };

            tracing::info!("started to decompress asset archive");
            let asset_files = Self::decompress_asset_archive_task(asset_contents, all).await?;
            tracing::info!("successfully decompressed asset archive");

            tracing::info!("started to decode files");
            let records = Self::decode_asset_files(asset_files).await?;
            tracing::info!(n = records.len(), "successfully decoded files");

            Ok(records)
        } else {
            return Err(anyhow!("releases page is empty"));
        }
    }

    fn next_poll() -> (DateTime<Utc>, Duration) {
        let now = Utc::now();
        let mut next = now.duration_round(Duration::hours(2)).unwrap();
        if next <= now {
            next += Duration::hours(2);
        }

        // We offset the next poll time by 10 minutes to increase the likelyhood that the new release has actually been published
        next += Duration::minutes(10);

        (next, next - now)
    }

    /// Retrieves the contents of the asset
    async fn get_asset_contents(repo: &RepoHandler<'_>, asset: AssetId) -> anyhow::Result<Vec<u8>> {
        tracing::info!("streaming contents");
        let stream = repo.release_assets().stream(*asset).await.unwrap();
        let chunks = stream.collect::<Vec<_>>().await;
        let contents = chunks.into_iter().collect::<Result<Vec<_>, _>>()?.concat();
        tracing::info!("successfully streamed contents");

        Ok(contents)
    }

    /// Decompresses the asset contents into all the files contained
    fn decompress_asset_archive(contents: Vec<u8>, all: bool) -> anyhow::Result<Vec<Vec<u8>>> {
        let mut archive = ZipArchive::new(Cursor::new(contents))?;

        // We decompress twice, because the archive containing all vulnerabilities is double compressed
        let mut archive = if all {
            let mut file = archive.by_index(0)?;

            let mut buf = Vec::with_capacity(file.size() as _);
            file.read_to_end(&mut buf)?;

            ZipArchive::new(Cursor::new(buf))?
        } else {
            archive
        };

        let mut decompressed_files = Vec::new();
        let file_names = archive.file_names().map(String::from).collect::<Vec<_>>();
        for file_name in file_names {
            if let Ok(mut file) = archive.by_name(&file_name) {
                if file.is_file() && file_name.ends_with(".json") {
                    if ["cves/delta.json", "cves/deltaLog.json"].contains(&file_name.as_str()) {
                        continue;
                    }

                    let mut buf = Vec::new();
                    file.read_to_end(&mut buf)?;

                    decompressed_files.push(buf);
                }
            } else {
                tracing::error!(?file_name, "failed to get compressed file")
            }
        }

        Ok(decompressed_files)
    }

    async fn decompress_asset_archive_task(
        contents: Vec<u8>,
        all: bool,
    ) -> anyhow::Result<Vec<Vec<u8>>> {
        // Spawn a blocking task to not starve the executor, while decompressing the asset contents
        task::spawn_blocking(move || Self::decompress_asset_archive(contents, all)).await?
    }

    /// Decodes the asset files into records
    async fn decode_asset_files(files: Vec<Vec<u8>>) -> anyhow::Result<Vec<CveRecord>> {
        let mut records = Vec::with_capacity(files.len());

        let files: Arc<[Vec<u8>]> = Arc::from(files);

        const FILES_CHUNK_SIZE: usize = 8192;
        let mut file_indices = Vec::new();
        while file_indices.len() * FILES_CHUNK_SIZE < files.len() {
            let start = file_indices.len() * FILES_CHUNK_SIZE;
            file_indices.push(start..(start + FILES_CHUNK_SIZE).min(files.len()));
        }

        let mut tasks = JoinSet::new();
        for files_chunk in file_indices {
            let files = Arc::clone(&files);

            tasks.spawn_blocking(move || {
                files_chunk
                    .into_iter()
                    .map(|i| Self::decode_asset_file(&files[i]))
                    .collect::<Result<Vec<_>, _>>()
            });
        }

        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(records_chunk) => {
                    records.extend(records_chunk?);
                }
                Err(err) => {
                    tracing::error!(?err, "task panicked");
                    return Err(err.into());
                }
            }
        }

        Ok(records)
    }

    #[tracing::instrument(skip(file))]
    fn decode_asset_file(file: &[u8]) -> serde_json::Result<CveRecord> {
        match serde_json::from_slice(file) {
            Ok(record) => Ok(record),
            Err(err) => {
                tracing::warn!(
                    file = ?str::from_utf8(&file),
                    ?err,
                    "failed to decode asset file"
                );
                return Err(err);
            }
        }
    }
}

// #[async_trait]
// impl VulnerabilityFeed for Github {
//     async fn listen(&self) -> Result<impl VulnerabilityQueue, ()> {
//         let (_, rx) = tokio::sync::broadcast::channel::<VulnerabilityEvent>(1024);
//         Ok(BroadcastStream::new(rx))
//     }
// }

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CveTimestamp {
    With(DateTime<Utc>),
    Without(NaiveDateTime),
}

impl CveTimestamp {
    pub fn and_utc(&self) -> DateTime<Utc> {
        match self {
            Self::With(ts) => *ts,
            Self::Without(ts) => ts.and_utc(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CveId {
    pub year: u16,
    pub id: u64,
}

impl TryFrom<&str> for CveId {
    type Error = anyhow::Error;

    // See https://github.com/CVEProject/cve-schema/blob/main/schema/CVE_Record_Format.json
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("CVE-") {
            if (&value[4..8]).chars().all(|c| c.is_ascii_digit()) {
                let year: u16 = value[4..8].parse().unwrap();

                if &value[8..9] == "-" && (4..=19).contains(&value[9..].chars().count()) {
                    if value[9..].chars().all(|c| c.is_ascii_digit()) {
                        let id: u64 = value[9..].parse().unwrap();
                        return Ok(Self { year, id });
                    }
                }

                Err(anyhow!("invalid valid"))
            } else {
                Err(anyhow!("year must contain 4 digits"))
            }
        } else {
            return Err(anyhow!("value isn't prefixed with \"CVE-\""));
        }
    }
}

impl From<CveId> for String {
    fn from(value: CveId) -> Self {
        format!("CVE-{:04}-{:04}", value.year, value.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CveMeta {
    #[serde_as(as = "TryFromInto<&str>")]
    #[serde(rename = "cveId")]
    pub id: CveId,
    pub state: String,
    #[serde(rename = "dateReserved")]
    pub reserved_at: Option<CveTimestamp>,
    #[serde(rename = "datePublished")]
    pub published_at: Option<CveTimestamp>,
    #[serde(rename = "dateRejected")]
    pub rejected_at: Option<CveTimestamp>,
    #[serde(rename = "dateUpdated")]
    pub updated_at: Option<CveTimestamp>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CveReference {
    pub url: Url,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CveDescription {
    #[serde(rename = "lang")]
    pub language: String,
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CveCnaContainer {
    pub title: Option<String>,
    pub descriptions: Vec<CveDescription>,
    pub references: Vec<CveReference>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CveContainer {
    Cna(CveCnaContainer),
    Adp {},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CveDataType;

impl CveDataType {
    const VALUE: &str = "CVE_RECORD";
}

impl TryFrom<&str> for CveDataType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == Self::VALUE {
            Ok(CveDataType)
        } else {
            Err(anyhow!(
                "invalid value (got: {value:?}, expected: {:?})",
                Self::VALUE
            ))
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CveRecord {
    #[serde_as(as = "TryFromInto<&str>")]
    pub data_type: CveDataType,
    pub data_version: String,
    #[serde(rename = "cveMetadata")]
    pub metadata: CveMeta,
    pub containers: HashMap<String, JsonValue>,
}

#[cfg(test)]
mod tests {
    use crate::repos::github::CveId;

    #[test]
    fn test_try_from_cve_id() {
        let cve = CveId::try_from("CVE-0000-0000").unwrap();
        assert!(cve.year == 0 && cve.id == 0);

        let cve = CveId::try_from("CVE-2020-1234567891011121314").unwrap();
        println!("{cve:?}");
        assert!(cve.year == 2020 && cve.id == 1234567891011121314);
    }
}
