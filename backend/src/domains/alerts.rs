use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone)]
#[derive(utoipa::ToSchema)]
pub struct Alert {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub message: String,
    pub severity: Severity
}

#[derive(Serialize, Deserialize, Clone)]
#[derive(utoipa::ToSchema)]
pub struct CreateAlert {
    pub name: String,
    pub message: String,
    pub severity: Severity
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
#[derive(utoipa::ToSchema)]
pub enum Severity {
    Low,
    Medium,
    High, 
    Critical,
}
