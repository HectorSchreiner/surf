use std::sync::{Arc, RwLock};

<<<<<<< HEAD
pub use github::*;
pub use postgres::Postgres;
=======
use ::async_trait::async_trait;
use ::uuid::Uuid;

pub use self::github::{Github, GithubConfig};
pub use self::postgres::Postgres;
use crate::domains::users::*;

mod github;
pub mod postgres;

#[derive(Debug, Clone, Default)]
pub struct Mock {
    users: Arc<RwLock<Vec<User>>>,
}

#[async_trait]
impl UserRepo for Mock {
    async fn list_users(&self) -> Result<Vec<User>, ListUsersError> {
        Ok(self.users.read().unwrap().clone())
    }

    async fn find_user(&self, _r: FindUser) -> Result<Option<User>, FindUserError> {
        Ok(None)
    }

    async fn new_user(&self, r: NewUser) -> Result<User, NewUserError> {
        let user = User {
            id: UserId::from(Uuid::new_v4()),
            email: EmailAddress::parse(r.email).unwrap(),
            name: r.name,
            password: r.password,
            reset: false,
        };

        self.users.write().unwrap().push(user.clone());

        Ok(user)
    }
}
>>>>>>> main
