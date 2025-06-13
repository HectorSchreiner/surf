use std::io::{Cursor, Read};

use ::async_trait::async_trait;
use ::tokio::fs;
use ::zip::ZipArchive;

use crate::domains::vulnerabilities::{
    ListVulnerabilitiesError, NewVulnerability, Vulnerability, VulnerabilityRepo,
};
use crate::{CveCnaContainer, CveRecord};

#[async_trait]
pub trait VulnerabilityService: Send + Sync + 'static {
    async fn list_vulnerabilities(&self) -> Result<Vec<Vulnerability>, ListVulnerabilitiesError>;
}

pub struct Service<VR: VulnerabilityRepo> {
    repo: VR,
}

impl<VR: VulnerabilityRepo> Service<VR> {
    pub async fn new(repo: VR) -> Self {
        let contents = fs::read("../vulnerabilities.zip").await.unwrap();

        tracing::info!("reading archive");
        let mut archive = ZipArchive::new(Cursor::new(contents)).unwrap();
        let file = archive.by_index(0).unwrap();
        let file = file.bytes().collect::<Result<Vec<_>, _>>().unwrap();

        tracing::info!("reading nested archive");
        let mut archive = ZipArchive::new(Cursor::new(file)).unwrap();
        for file_name in archive
            .file_names()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
        {
            if let Ok(file) = archive.by_name(&file_name) {
                if file_name.ends_with(".json") && file.is_file() {
                    if ["cves/delta.json", "cves/deltaLog.json"].contains(&file_name.as_str()) {
                        continue;
                    }

                    let mut contents = file.bytes().collect::<Result<Vec<_>, _>>().unwrap();

                    tracing::info!("{file_name}");
                    let mut record: CveRecord = serde_json::from_slice(&contents).unwrap();

                    if let Some(value) = record.containers.remove("cna") {
                        if record.metadata.rejected_at.is_some() {
                            tracing::warn!("skipping, because of rejection");
                            continue;
                        }

                        let cna: CveCnaContainer = serde_json::from_value(value).unwrap();
                        let description = cna.descriptions[0].value.clone();

                        let new_vulnerability = NewVulnerability {
                            key: record.metadata.id.clone(),
                            reserved_at: Some(record.metadata.reserved_at.and_utc()),
                            published_at: record.metadata.published_at.map(|ts| ts.and_utc()),
                            name: cna.title.unwrap_or_else(move || record.metadata.id),
                            description,
                        };

                        repo.new_vulnerability(new_vulnerability).await.unwrap();
                    } else {
                        tracing::error!("failed to get cna key")
                    }
                } else {
                    println!("{file_name} is a directory");
                }
            }
        }

        Self { repo }
    }
}

#[async_trait]
impl<VR: VulnerabilityRepo> VulnerabilityService for Service<VR> {
    async fn list_vulnerabilities(&self) -> Result<Vec<Vulnerability>, ListVulnerabilitiesError> {
        self.repo.list_vulnerabilities().await
    }
}
