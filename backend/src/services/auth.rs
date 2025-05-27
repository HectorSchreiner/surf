use std::sync::Arc;

use ::secrecy::SecretString;

use crate::config::SecurityConfig;
use crate::domains::users::{NewUser, UserId, UserRepo};

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
    pub async fn setup(user_repo: impl UserRepo, config: &SecurityConfig) -> anyhow::Result<Self> {
        let SecurityConfig { admin_email, admin_password, .. } = config;

        let users = user_repo.list_users().await?;
        let admin_user = users.iter().find(|user| &user.email.0 == admin_email);
        if users.is_empty() && admin_user.is_none() {
            let new_user = NewUser {
                email: admin_email.clone(),
                password: admin_password.clone(),
                name: "admin".to_string(),
                reset: true,
            };

            user_repo.new_user(new_user).await?;
        }

        Ok(todo!())
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
