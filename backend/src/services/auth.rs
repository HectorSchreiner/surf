use std::sync::Arc;

use ::secrecy::SecretString;

use crate::domains::users::{UserId, UserRepo};

pub struct Identity {
    pub id: UserId,
    pub name: String,
    token: SecretString,
}

impl From<Identity> for SecretString {
    fn from(value: Identity) -> Self {
        value.token
    }
}
//pub struct RefreshToken(String);

pub struct IdentityService {
    user_repo: Arc<dyn UserRepo>,
}

impl IdentityService {
    pub async fn setup(user_repo: impl UserRepo) -> anyhow::Result<Self> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    pub async fn login(&self, email: String, password: SecretString) -> anyhow::Result<Identity> {
        let Self { user_repo } = self;

        // user_repo.find_user(&);
        todo!()
    }

    pub async fn validate(&self, token: SecretString) -> anyhow::Result<Identity> {
        todo!()
    }
}
