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
        Self { repo }
    }
}

#[async_trait]
impl<VR: VulnerabilityRepo> VulnerabilityService for Service<VR> {
    async fn list_vulnerabilities(&self) -> Result<Vec<Vulnerability>, ListVulnerabilitiesError> {
        self.repo.list_vulnerabilities().await
    }
}
