use std::io::Cursor;

use ::futures::StreamExt;
use ::octocrab::Octocrab;
use ::secrecy::SecretString;
use ::serde::Deserialize;
use ::tokio::fs;
use ::tokio::task::JoinHandle;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GithubConfig {
    pub access_token: SecretString,
}

pub struct Github {
    handle: JoinHandle<()>,
}

impl Github {
    pub async fn new(config: GithubConfig) -> anyhow::Result<Self> {
        use secrecy::ExposeSecret;

        let GithubConfig { access_token } = config;

        let client = Octocrab::builder()
            .personal_token(access_token.expose_secret())
            .build()
            .unwrap();

        let repo = client.repos("cveproject", "cvelistv5");
        let releases = repo.releases().list().per_page(100).send().await.unwrap();

        for release in &releases.items[0..1] {
            for asset in &release.assets {
                tracing::info!("Assets: {}", asset.name);
                if asset.name.contains("_all_") {
                    let mut contents = Vec::new();
                    tracing::info!("started streaming contents");
                    let mut stream = repo.release_assets().stream(*asset.id).await.unwrap();
                    while let Some(chunk) = stream.next().await {
                        contents.extend(chunk.unwrap().to_vec())
                    }

                    tracing::info!("successfully streamed contents");

                    fs::write("vulnerabilities.zip", &mut contents)
                        .await
                        .unwrap();

                    let archive = zip::ZipArchive::new(Cursor::new(&mut contents)).unwrap();

                    for file_name in archive.file_names() {
                        println!("{file_name}");
                    }
                }
            }
        }

        let handle = tokio::spawn(async move {});

        Ok(Self { handle })
    }
}

// #[async_trait]
// impl VulnerabilityFeed for Github {
//     async fn listen(&self) -> Result<impl VulnerabilityQueue, ()> {
//         let (_, rx) = tokio::sync::broadcast::channel::<VulnerabilityEvent>(1024);
//         Ok(BroadcastStream::new(rx))
//     }
// }
