use std::process::Output;

<<<<<<< HEAD
#[derive(Serialize, Clone)]
=======
use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::derive_more::{AsRef, Deref};
use ::serde::{Deserialize, Serialize};
use ::serde_with::{TryFromInto, serde_as};
use ::sqlx::*;
use ::thiserror::Error;
use ::uuid::Uuid;

use crate::domains::{StringNormalize, StringSanitize, ValidateLength};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
#[serde(transparent)]
pub struct AlertId(Uuid);

impl AlertId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for AlertId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[error("{0:?} is not a valid alert name")]
pub struct ParseAlertNameError(String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Deref, AsRef)]
#[deref(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct AlertName(String);

impl AlertName {
    pub fn parse(raw: impl Into<String>) -> Result<Self, ParseAlertNameError> {
        let name = raw.into().normalize().sanitize();

        if let Result::Ok(name) = Self::validate_length(&name) {
            Ok(AlertName(name.to_string()))
        } else {
            Err(ParseAlertNameError(name))
        }
    }
}

impl ValidateLength for AlertName {
    const MAX_LEN: usize = 30;
    const MIN_LEN: usize = 1;
}

impl Into<String> for AlertName {
    fn into(self) -> String {
        self.0
    }
}

impl TryFrom<String> for AlertName {
    type Error = ParseAlertNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[error("{0:?} is not a valid alert message")]
pub struct ParseAlertMessageError(String);

#[derive(Debug, Clone, PartialEq, Eq, Deref, AsRef)]
#[deref(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct AlertMessage(String);

impl AlertMessage {
    pub fn parse(raw: impl Into<String>) -> Result<Self, ParseAlertMessageError> {
        let message = raw.into().normalize().sanitize();

        if let Result::Ok(message) = Self::validate_length(&message) {
            Ok(AlertMessage(message.to_string()))
        } else {
            Err(ParseAlertMessageError(message))
        }
    }
}

impl ValidateLength for AlertMessage {
    const MAX_LEN: usize = 9999;
    const MIN_LEN: usize = 1;
}

impl Into<String> for AlertMessage {
    fn into(self) -> String {
        self.0
    }
}

impl TryFrom<String> for AlertMessage {
    type Error = ParseAlertMessageError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, sqlx::Type)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
#[sqlx(type_name = "severity", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
>>>>>>> main
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
<<<<<<< HEAD
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
=======
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
>>>>>>> main
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
