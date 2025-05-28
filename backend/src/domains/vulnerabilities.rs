use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::futures::Stream;
use ::serde::{Deserialize, Serialize};
use ::uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct Vulnerability {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub key: String,
}

pub enum ListVulnerabilitiesError {
    Other(anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct NewVulnerability {
    pub key: String,
    pub title: String,
    pub description: String,
}

pub enum NewVulnerabilityError {
    Other(anyhow::Error),
}

#[async_trait]
pub trait VulnerabilityRepo {
    /// Lists all vulnerabilities in the repository
    async fn list_vulnerabilities(&self) -> Result<Vec<Vulnerability>, ListVulnerabilitiesError>;

    /// Creates a new vulnerability in the repository
    async fn new_vulnerability(
        &self,
        new_vulnerability: NewVulnerability,
    ) -> Result<Vulnerability, NewVulnerabilityError>;
}

#[derive(Debug, Clone)]
pub enum VulnerabilityEvent {
    Created(NewVulnerability),
}

#[async_trait]
pub trait VulnerabilityFeed {
    async fn listen(&self) -> Result<impl Stream<Item = VulnerabilityEvent> + 'static, ()>;
}
