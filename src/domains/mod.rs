use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use ::uuid::Uuid;

pub mod users;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
        r: NewVulnerability,
    ) -> Result<Vulnerability, NewVulnerabilityError>;
}

// pub trait VulnerabilityFeed {
//     async fn list_vulnerabilities(&self) ->
// }
