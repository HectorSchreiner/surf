use std::sync::Arc;

use ::serde::{Deserialize, Serialize};

use crate::config::SecurityConfig;
use crate::domains;
use crate::domains::users::{NewUser, UserId, UserRepo};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub name: String,
}

impl From<domains::users::User> for User {
    fn from(value: domains::users::User) -> Self {
        Self { id: value.id, email: value.email, name: value.name }
    }
}

pub struct UserService {
    user_repo: Arc<dyn UserRepo>,
}

impl UserService {
    pub async fn new(user_repo: impl UserRepo, config: SecurityConfig) -> anyhow::Result<Self> {
        let SecurityConfig { admin_email, admin_password, .. } = config;

        let users = user_repo.list_users().await?;
        let admin_user = users.iter().find(|user| user.email == admin_email);
        if admin_user.is_none() {
            let new_user = NewUser {
                email: admin_email,
                password: admin_password,
                name: "admin".to_string(),
                reset: true,
            };

            user_repo.new_user(new_user).await?;
        }

        Ok(Self { user_repo: Arc::new(user_repo) })
    }

    pub async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        let Self { user_repo } = self;

        let users = user_repo.list_users().await?;
        Ok(users.into_iter().map(Into::into).collect())
    }
}
