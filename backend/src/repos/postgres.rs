use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::sqlx::migrate;
use ::sqlx::postgres::PgPool;
use ::sqlx::prelude::*;
use ::uuid::Uuid;

use crate::domains::users::*;
use crate::domains::vulnerabilities::*;

#[derive(Debug, Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn connect() -> anyhow::Result<Self> {
        let pool = PgPool::connect("postgresql://user:password@localhost:5432/main").await?;

        migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    #[tracing::instrument(skip(self, m))]
    async fn insert_user(&self, m: UserModel) -> sqlx::Result<()> {
        let Self { pool } = self;

        let sql = r#"
            INSERT INTO users (id, email, password, name, reset)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        let query = sqlx::query(sql)
            .bind(m.id)
            .bind(m.email)
            .bind(m.password)
            .bind(m.name)
            .bind(m.reset);

        query.execute(pool).await.map(|_| ())
    }
}

#[derive(FromRow)]
struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
    pub reset: bool,
}

impl From<User> for UserModel {
    fn from(value: User) -> Self {
        use ::secrecy::ExposeSecret;

        Self {
            id: value.id.into(),
            email: value.email,
            password: value.password.expose_secret().to_string(),
            name: value.name,
            reset: value.reset,
        }
    }
}

impl TryFrom<UserModel> for User {
    type Error = anyhow::Error;

    fn try_from(value: UserModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.into(),
            email: value.email,
            password: value.password.into(),
            name: value.name,
            reset: value.reset,
        })
    }
}

#[async_trait]
impl UserRepo for Postgres {
    #[tracing::instrument(skip(self))]
    async fn list_users(&self) -> Result<Vec<User>, ListUsersError> {
        let Self { pool } = self;

        let sql = r#"SELECT * FROM users"#;
        let query = sqlx::query_as::<_, UserModel>(sql);

        tracing::info!(?sql, "selecting users from database");
        match query.fetch_all(pool).await {
            Ok(models) => {
                tracing::info!("successfully selected users from database");
                let users = models.into_iter().map(TryInto::try_into);
                Ok(users.collect::<Result<Vec<_>, _>>().unwrap())
            }
            Err(err) => {
                tracing::info!("failed to select users from database");
                Err(ListUsersError::Other(err.into()))
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn find_user(&self, r: FindUser) -> Result<Option<User>, FindUserError> {
        let Self { pool } = self;

        let sql = {
            let condition = match r.by {
                FindUserBy::Email(_) => "email = $1",
            };

            format!(r#"SELECT * FROM users WHERE {condition}"#)
        };

        let mut query = sqlx::query_as::<_, UserModel>(&sql);
        match r.by {
            FindUserBy::Email(email) => query = query.bind(email),
        }

        match query.fetch_optional(pool).await {
            Ok(user) => Ok(user.map(TryInto::try_into).transpose().unwrap()),
            Err(err) => Err(FindUserError::Other(err.into())),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn new_user(&self, r: NewUser) -> Result<User, NewUserError> {
        let user = User {
            id: UserId::from(Uuid::new_v4()),
            email: r.email,
            password: r.password,
            name: r.name,
            reset: r.reset,
        };

        tracing::info!("inserting user in database");
        match self.insert_user(user.clone().into()).await {
            Ok(_) => {
                tracing::info!("successfully inserted user in database");
                Ok(user)
            }
            Err(err) => {
                tracing::error!("failed to insert user in database");
                Err(NewUserError::Other(err.into()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct VulnerabilityModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub key: String,
    pub reserved_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: String,
}

impl From<Vulnerability> for VulnerabilityModel {
    fn from(value: Vulnerability) -> Self {
        VulnerabilityModel {
            id: value.id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            key: value.key,
            reserved_at: value.reserved_at,
            published_at: value.published_at,
            rejected_at: value.rejected_at,
            name: value.name,
            description: value.description,
        }
    }
}

impl TryFrom<VulnerabilityModel> for Vulnerability {
    type Error = anyhow::Error;

    fn try_from(value: VulnerabilityModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: VulnerabilityId::from(value.id),
            created_at: value.created_at,
            updated_at: value.updated_at,
            key: value.key,
            reserved_at: value.reserved_at,
            published_at: value.published_at,
            rejected_at: value.rejected_at,
            name: value.name,
            description: value.description,
        })
    }
}

type NewVulnerabilityResult = Result<Vulnerability, NewVulnerabilityError>;

#[async_trait]
impl VulnerabilityRepo for Postgres {
    async fn list_vulnerabilities(&self) -> Result<Vec<Vulnerability>, ListVulnerabilitiesError> {
        let Self { pool } = &self;

        let sql = r#"SELECT * FROM vulnerabilities"#;
        let query = sqlx::query_as::<_, VulnerabilityModel>(sql);

        match query.fetch_all(pool).await {
            Ok(models) => {
                let results = models.into_iter().map(TryFrom::try_from);
                Ok(results.collect::<Result<_, _>>().unwrap())
            }
            Err(err) => Err(ListVulnerabilitiesError::Other(err.into())),
        }
    }

    async fn search_vulnerabilities(
        &self,
        req: SearchVulnerabilities,
    ) -> Result<Vec<Vulnerability>, SearchVulnerabilitiesError> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn new_vulnerability(&self, args: NewVulnerability) -> NewVulnerabilityResult {
        let Self { pool } = self;

        let vulnerability = Vulnerability::new(args);
        let model: VulnerabilityModel = vulnerability.clone().into();

        let sql = r#"
            INSERT INTO vulnerabilities (id, created_at, updated_at, key, reserved_at, published_at, rejected_at, name, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#;

        let query = sqlx::query(sql)
            .bind(model.id)
            .bind(model.created_at)
            .bind(model.updated_at)
            .bind(model.key)
            .bind(model.reserved_at)
            .bind(model.published_at)
            .bind(model.rejected_at)
            .bind(model.name)
            .bind(model.description);

        match query.execute(pool).await {
            Ok(_) => Ok(vulnerability),
            Err(err) => Err(NewVulnerabilityError::Other(err.into())),
        }
    }
}
