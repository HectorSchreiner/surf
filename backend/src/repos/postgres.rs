use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::sqlx::migrate;
use ::sqlx::postgres::PgPool;
use ::sqlx::prelude::*;
use ::uuid::Uuid;

pub use crate::domains::vulnerabilities::*;

pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn connect() -> anyhow::Result<Self> {
        let pool = PgPool::connect("postgresql://user:password@localhost:5432/main").await?;

        migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct VulnerabilityModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub key: String,
}

impl Into<Vulnerability> for VulnerabilityModel {
    fn into(self) -> Vulnerability {
        Vulnerability {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: self.name,
            description: self.description,
            key: self.key,
        }
    }
}

#[async_trait]
impl VulnerabilityRepo for Postgres {
    async fn list_vulnerabilities(&self) -> Result<Vec<Vulnerability>, ListVulnerabilitiesError> {
        let Self { pool } = &self;

        let sql = r#"SELECT * FROM vulnerabilities"#;
        let query = sqlx::query_as::<_, VulnerabilityModel>(sql);

        match query.fetch_all(pool).await {
            Ok(models) => Ok(models.into_iter().map(Into::into).collect()),
            Err(err) => Err(ListVulnerabilitiesError::Other(err.into())),
        }
    }

    async fn new_vulnerability(
        &self,
        _r: NewVulnerability,
    ) -> Result<Vulnerability, NewVulnerabilityError> {
        todo!()
    }
}
