use anyhow::Ok;
use axum::extract::{self, path};
use chrono::{DateTime, Utc};
use http::{header::ACCEPT, HeaderMap, StatusCode};
use reqwest::{header::USER_AGENT, Client, ClientBuilder};
use serde::Deserialize;
use tokio::fs;
use url::Url;
use zip::{read::ZipFile, unstable::stream::ZipStreamReader, *};

use std::{collections::HashMap, fs::File, num::NonZeroU8};

pub struct Language([Option<NonZeroU8>; 3]);

pub struct VulnerabilityMetric {
    pub vector: String,
}

pub struct Vulnerability {
    pub name: String,
    pub descriptions: HashMap<String, String>,
    pub metrics: Vec<VulnerabilityMetric>,
}

pub struct ListVulnerabilities {}

pub struct VulnerabilityRepo {
    base_url: Url,
    client: Client,
}
impl VulnerabilityRepo {
    const BASE_URL: &'static str = "https://api.github.com";

    pub async fn new() -> anyhow::Result<Self> {
        let base_url = Url::parse(Self::BASE_URL).unwrap();

        let headers = HeaderMap::from_iter([(USER_AGENT, "skrt".parse().unwrap())]);

        let client = ClientBuilder::new()
            .https_only(true)
            .default_headers(headers)
            .build().unwrap();

        Ok(Self { base_url, client })
    }

    pub async fn list_vulnerabilities(
        &self,
        r: ListVulnerabilities,
    ) -> anyhow::Result<Vec<Release>> {
        let releases = self.get_releases().await?;
        let release = &releases[0];
        let assets = self.get_assets(release.id).await?;
        
        let zip_output_dir = std::path::Path::new("./all.zip");
        let extracted_output_dir = std::path::Path::new("./unzipped");


        for asset in assets {
            println!("{}: {}", asset.name, asset.content_type);

            if asset.name.contains("_all_") {
                if let Some(contents) = self.get_asset_contents(asset.id).await? {

                    fs::write(&zip_output_dir, &contents).await?;

                }
            }

        }

        Self::extract(&zip_output_dir, &extracted_output_dir).await?;


        //println!("{assets:#?}");

        Ok(releases)
    }

    pub async fn extract(target_dir: &std::path::Path, output_dir: &std::path::Path) -> anyhow::Result<()>{
        let file = tokio::fs::File::open(target_dir).await?.try_into_std().unwrap();
        let mut archive = ZipArchive::new(&file).unwrap();

        if let Err(err) = archive.extract(output_dir) {
            println!("upsi wupsers, something has gone wrong here. Err: {:?}", err);
        };
        
        Ok(())
    }

    async fn get_releases(&self) -> anyhow::Result<Vec<Release>> {
        let Self { base_url, client } = &self;

        let path = "repos/cveproject/cvelistv5/releases";
        let url = base_url.join(path).unwrap();

        let request = client.get(url);
        let response = request.send().await?;

        println!("{}", response.status());

        let releases = response.json::<Vec<Release>>().await?;
        Ok(releases)
    }

    async fn get_assets(&self, release_id: u64) -> anyhow::Result<Vec<Asset>> {
        let Self { base_url, client } = self;

        let path = format!("repos/cveproject/cvelistv5/releases/{}/assets", release_id);
        let url = base_url.join(&path).unwrap();
        let request = client.get(url);
        let response = request.send().await?;

        let assets = response.json::<Vec<Asset>>().await?;

        Ok(assets)
    }

    async fn get_asset_contents(&self, asset_id: u64) -> anyhow::Result<Option<Vec<u8>>> {
        let Self { base_url, client, .. } = self;

        let path = format!("repos/cveproject/cvelistv5/releases/assets/{}", asset_id);
        let url = base_url.join(&path).unwrap();
        let request = client.get(url).header(ACCEPT, "application/octet-stream");
        let response = request.send().await?;

        match response.status() {
            StatusCode::OK => Ok(Some(response.bytes().await?.to_vec())),
            StatusCode::NOT_FOUND => Ok(None),
            status => anyhow::bail!("expected status 200 or 404 (got: {status})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Release {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Asset {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub content_type: String,
}
