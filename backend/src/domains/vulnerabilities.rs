use std::ops::Range;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::futures::Stream;
use ::serde::{Deserialize, Serialize};
use ::thiserror::Error;
use ::url::Url;
use ::uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct VulnerabilityId(Uuid);

impl VulnerabilityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for VulnerabilityId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl Into<Uuid> for VulnerabilityId {
    fn into(self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct VulnerabilityReference {
    pub url: Url,
    pub name: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct Vulnerability {
    pub id: VulnerabilityId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub key: String,
    pub reserved_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: String,
    pub references: Vec<VulnerabilityReference>,
}

impl Vulnerability {
    pub fn new(args: NewVulnerability) -> Self {
        let now = Utc::now();
        Self {
            id: VulnerabilityId::new(),
            created_at: now,
            updated_at: now,
            key: args.key,
            reserved_at: args.reserved_at,
            published_at: args.published_at,
            rejected_at: args.rejected_at,
            name: args.name,
            description: args.description,
            references: args.references,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListVulnerabilities {
    pub range: Option<Range<usize>>,
}

#[derive(Debug, Clone)]
pub struct ListedVulnerabilities {
    pub total_vulnerabilities: usize,
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Error)]
pub enum ListVulnerabilitiesError {
    #[error(transparent)]
    Other(anyhow::Error),
}

impl ListVulnerabilitiesError {
    pub fn other(err: impl Into<anyhow::Error>) -> Self {
        Self::Other(err.into())
    }
}

#[derive(Debug, Clone)]
pub struct SearchVulnerabilities {
    pub words: Vec<String>,
}

#[derive(Debug, Error)]
pub enum SearchVulnerabilitiesError {
    #[error(transparent)]
    Other(anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct NewVulnerability {
    pub key: String,
    pub reserved_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: String,
    pub references: Vec<VulnerabilityReference>,
}

#[derive(Debug, Error)]
pub enum NewVulnerabilityError {
    #[error(transparent)]
    Other(anyhow::Error),
}

#[async_trait]
pub trait VulnerabilityRepo: Send + Sync + 'static {
    /// Lists all vulnerabilities in the repository
    async fn list_vulnerabilities(
        &self,
        req: ListVulnerabilities,
    ) -> Result<ListedVulnerabilities, ListVulnerabilitiesError>;

    /// Searches for vulnerabilities
    async fn search_vulnerabilities(
        &self,
        req: SearchVulnerabilities,
    ) -> Result<Vec<Vulnerability>, SearchVulnerabilitiesError>;

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
