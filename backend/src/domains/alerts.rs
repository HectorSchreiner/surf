use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct Alert {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct CreateAlert {
    pub name: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}
<<<<<<< Updated upstream
=======

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct Alert {
    pub id: AlertId,
    pub created_at: DateTime<Utc>,
    #[serde_as(as = "TryFromInto<String>")]
    pub name: AlertName,
    #[serde_as(as = "TryFromInto<String>")]
    pub message: AlertMessage,
    pub severity: AlertSeverity,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct CreateAlert {
    pub name: AlertName,
    #[serde_as(as = "TryFromInto<String>")]
    pub message: AlertMessage,
    pub severity: AlertSeverity,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct NewAlert {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub message: String,
    pub severity: AlertSeverity,
}

pub enum ListAlertsError {
    Other(anyhow::Error),
}

pub enum NewAlertError {
    Other(anyhow::Error),
}

#[async_trait]
pub trait AlertRepo: Send + Sync {
    /// Lists all alerts in the repository
    async fn list_alerts(&self) -> Result<Vec<Alert>, ListAlertsError>;
    async fn new_alert(&self, alert: NewAlert) -> Result<Alert, NewAlertError>;
}
>>>>>>> Stashed changes
