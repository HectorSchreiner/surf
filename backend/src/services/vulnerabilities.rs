use std::io::{Cursor, Read};
use std::ops::Range;

use ::async_trait::async_trait;

use crate::domains::vulnerabilities::{
    ListVulnerabilities, ListVulnerabilitiesError, ListedVulnerabilities, NewVulnerability,
    Vulnerability, VulnerabilityRepo,
};

#[async_trait]
pub trait VulnerabilityService: Send + Sync + 'static {
    async fn list_vulnerabilities(
        &self,
        req: ListVulnerabilities,
    ) -> Result<ListedVulnerabilities, ListVulnerabilitiesError>;
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
    async fn list_vulnerabilities(
        &self,
        req: ListVulnerabilities,
    ) -> Result<ListedVulnerabilities, ListVulnerabilitiesError> {
        self.repo.list_vulnerabilities(req).await
    }
}
