use ::async_trait::async_trait;
use ::bon::Builder;
use ::chrono::{DateTime, Utc};
use ::secrecy::SecretString;
use ::serde::{Deserialize, Serialize};
use ::thiserror::Error;
use ::utoipa::ToSchema;
use ::uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct UserId(pub Uuid);

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self { Self(value) }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self { value.0 }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct EmailAddress(pub String);

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[error("{0} is not a valid email address")]
pub struct EmailAddressError(pub String);

impl EmailAddress {
    pub fn new(raw_email: &str) -> Result<Self, EmailAddressError> {
        todo!(); // check if valid, else return EmailAdressError
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct UserName(pub String);

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[error("{0} is not a valid username")]
pub struct UserNameError(pub String);
impl UserName {
    pub fn new(raw_username: &str) -> Result<Self, UserNameError> {
        todo!();
    } 
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct Password(pub String);

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[error("{0} is not a valid password")]
pub struct PasswordError(pub String);
impl Password {
    pub fn new(raw_password: &str) -> Result<Self, PasswordError> {
        todo!(); // password logic
    } 
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: EmailAddress,
    pub password: Password,
    pub name: UserName,
    pub reset: bool,
}

#[derive(Debug, Error)]
pub enum ListUsersError {
    #[error(transparent)]
    Other(anyhow::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: SecretString,
    pub name: String,
    pub reset: bool,
}

#[derive(Debug, Error)]
pub enum NewUserError {
    #[error(transparent)]
    Other(anyhow::Error),
}

#[derive(Debug, Clone)]
pub enum FindUserBy {
    Email(String),
}

#[derive(Debug, Builder)]
pub struct FindUser {
    pub by: FindUserBy,
}

#[derive(Debug, Error)]
pub enum FindUserError {
    #[error(transparent)]
    Other(anyhow::Error),
}

#[async_trait]
pub trait UserRepo: Send + Sync + 'static {
    /// Lists all users in the repository
    async fn list_users(&self) -> Result<Vec<User>, ListUsersError>;

    /// Create a new user in the repository
    async fn new_user(&self, r: NewUser) -> Result<User, NewUserError>;

    async fn find_user(&self, r: FindUser) -> Result<Option<User>, FindUserError>;
}
