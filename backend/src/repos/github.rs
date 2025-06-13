use std::io::{Cursor, Read};

use ::anyhow::anyhow;
use ::chrono::{Duration, DurationRound, Utc};
use ::futures::StreamExt;
use ::octocrab::Octocrab;
use ::octocrab::models::AssetId;
use ::octocrab::models::repos::{Asset, Release};
use ::octocrab::repos::RepoHandler;
use ::secrecy::SecretString;
use ::serde::Deserialize;
use ::tokio::task::JoinHandle;
use ::tokio::{fs, task, time};
use ::tracing::{Instrument, info_span};
use ::zip::ZipArchive;

use crate::CveRecord;
use crate::domains::vulnerabilities::{VulnerabilityEvent, VulnerabilityFeed};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GithubConfig {
    pub access_token: SecretString,
}

pub struct Github {
    poll_task: JoinHandle<()>,
}

impl Github {
    pub async fn new(config: GithubConfig) -> anyhow::Result<Self> {
        use secrecy::ExposeSecret;

        let GithubConfig { access_token } = config;

        let client = Octocrab::builder()
            .personal_token(access_token.expose_secret())
            .build()?;

        let poll_task = tokio::spawn(async move {
            loop {
                tracing::info!("started polling vulnerabilities");
                Self::poll(&client, false).await.unwrap();

                let now = Utc::now();

                let mut next = now.duration_round(Duration::hours(2)).unwrap();
                if next <= now {
                    next += Duration::hours(2);
                }

                // We offset the next poll time by 10 minutes to ensure that the new releaase has been published
                next += Duration::minutes(10);
                tracing::info!(next=?next, "finished polling vulnerabilities");
                time::sleep((next - now).to_std().unwrap()).await;
            }
        });

        Ok(Self { poll_task })
    }

    #[tracing::instrument(skip(client), name = "github::poll")]
    async fn poll(client: &Octocrab, all: bool) -> anyhow::Result<()> {
        tracing::info!("retrieving releases page");
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

            // Spawn a blocking task to not starve the executor, while decompressing the asset contents
            let decompress_task =
                task::spawn_blocking(move || Self::decompress_asset_contents(asset_contents, all));

            let decompressed_files = decompress_task.await??;

            for file in decompressed_files {
                let record: CveRecord = serde_json::from_slice(&file).unwrap();
                println!("{record:#?}");
            }

            Ok(())
        } else {
            return Err(anyhow!("releases page is empty"));
        }
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
    fn decompress_asset_contents(contents: Vec<u8>, all: bool) -> anyhow::Result<Vec<Vec<u8>>> {
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

        tracing::info!(
            n = decompressed_files.len(),
            "successfully decompressed files"
        );

        Ok(decompressed_files)
    }
}

// #[async_trait]
// impl VulnerabilityFeed for Github {
//     async fn listen(&self) -> Result<impl VulnerabilityQueue, ()> {
//         let (_, rx) = tokio::sync::broadcast::channel::<VulnerabilityEvent>(1024);
//         Ok(BroadcastStream::new(rx))
//     }
// }
