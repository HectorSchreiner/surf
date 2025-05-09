use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use ::uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct UserId(Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct User {
    pub id: UserId,
    pub created_at: DateTime<Utc>,
    pub created_by: UserId,
    pub email: String,
}

pub enum ListUsersError {
    Other(anyhow::Error),
}

#[async_trait]
pub trait UserRepo {
    /// Lists all users in the repository
    async fn list_users(&self) -> Result<Vec<User>, ListUsersError>;
}
