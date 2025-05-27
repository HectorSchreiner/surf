use ::std::ops::Deref;
use ::thiserror::Error;
use ::chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use ::uuid::Uuid;
use ::derive_more::{Deref, AsRef};

use crate::domains::{StringNormalize, StringSanitize, ValidateLength};

#[derive(Serialize, Clone)]
#[derive(utoipa::ToSchema)]
pub struct Alert {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: AlertName,
    pub message: AlertMessage,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct AlertID(Uuid);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Deref, AsRef)]
#[deref(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct AlertName(pub String);

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[error("{0} is not a valid alert name")]
pub struct AlertNameError(String);
impl AlertName {
    pub fn new(raw_name: String) -> Result<Self, AlertNameError> {
        let name = raw_name
            .normalize()
            .sanitize();
         
        if let Result::Ok(name) = Self::validate_length(&name) {
            Ok(AlertName(name.to_string()))
        } else {
            Err(AlertNameError(name))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Deref, AsRef)]
#[deref(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct AlertMessage(pub String);

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[error("{0} is not a valid alert name")]
pub struct AlertMessageError(String);
impl AlertMessage {
    pub fn new(raw_message: String) -> Result<Self, AlertMessageError> {        
        let message = raw_message
            .normalize()
            .sanitize();
        
        if let Result::Ok(message) = Self::validate_length(&message) {
            Ok(AlertMessage(message.to_string()))
        } else {
            Err(AlertMessageError(message))
        }
    }
}  

impl ValidateLength for AlertName {
    const MIN_LEN: usize = 1;
    const MAX_LEN: usize = 30;
}

impl ValidateLength for AlertMessage {
    const MIN_LEN: usize = 1;
    const MAX_LEN: usize = 9999;
}



