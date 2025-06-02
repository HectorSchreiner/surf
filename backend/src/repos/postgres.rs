use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::secrecy::SecretString;
use ::sqlx::migrate;
use ::sqlx::postgres::{PgConnectOptions, PgPool};
use ::sqlx::prelude::*;
use ::uuid::Uuid;
use anyhow::Error;
use url::ParseError;

use crate::config::DatabaseConfig;
use crate::domains::alerts::*;
use crate::domains::users::*;
use crate::domains::vulnerabilities::*;

#[derive(Debug, Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn connect(config: DatabaseConfig) -> anyhow::Result<Self> {
        use ::secrecy::ExposeSecret;

        let DatabaseConfig { url, user, password } = config;

        let options = PgConnectOptions::new()
            .host(url.host())
            .port(url.port())
            .database(url.database())
            .username(&user)
            .password(password.expose_secret());

        let pool = PgPool::connect_with(options).await?;

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

    #[tracing::instrument(skip(self, m))]
    async fn insert_alert(&self, m: AlertModel) -> sqlx::Result<()> {
        let Self { pool } = self;

        let sql = r#"
            INSERT INTO alerts (id, created_at, name, message, severity)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        let query = sqlx::query(sql)
            .bind(m.id)
            .bind(m.created_at)
            .bind(m.name)
            .bind(m.message)
            .bind(m.severity);

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
    fn from(user: User) -> Self {
        use secrecy::ExposeSecret;

        Self {
            id: user.id.into(),
            email: user.email.into(),
            password: user.password.expose_secret().to_string(),
            name: user.name,
            reset: user.reset,
        }
    }
}

impl TryFrom<UserModel> for User {
    type Error = anyhow::Error;

    fn try_from(model: UserModel) -> Result<Self, Self::Error> {
        Ok(User {
            id: UserId::from(model.id),
            email: EmailAddress::parse(&model.email)?,
            password: SecretString::from(model.password),
            name: model.name,
            reset: model.reset,
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
    pub name: String,
    pub description: String,
    pub key: String,
}

impl Into<Vulnerability> for VulnerabilityModel {
    fn into(self) -> Vulnerability {
        Vulnerability {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: self.name,
            description: self.description,
            key: self.key,
        }
    }
}

#[async_trait]
impl VulnerabilityRepo for Postgres {
    async fn list_vulnerabilities(&self) -> Result<Vec<Vulnerability>, ListVulnerabilitiesError> {
        let Self { pool } = &self;

        let sql = r#"SELECT * FROM vulnerabilities"#;
        let query = sqlx::query_as::<_, VulnerabilityModel>(sql);

        match query.fetch_all(pool).await {
            Ok(models) => Ok(models.into_iter().map(Into::into).collect()),
            Err(err) => Err(ListVulnerabilitiesError::Other(err.into())),
        }
    }

    async fn new_vulnerability(
        &self,
        _r: NewVulnerability,
    ) -> Result<Vulnerability, NewVulnerabilityError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct AlertModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub message: String,
    pub severity: AlertSeverity,
}

impl TryFrom<AlertModel> for Alert {
    type Error = anyhow::Error;

    fn try_from(value: AlertModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.into(),
            created_at: value.created_at,
            name: AlertName::parse(value.name)?,
            message: AlertMessage::parse(value.message)?,
            severity: value.severity,
        })
    }
}

impl Into<AlertModel> for Alert {
    fn into(self) -> AlertModel {
        AlertModel {
            id: self.id.to_uuid(),
            created_at: self.created_at,
            name: self.name.to_string(),
            message: self.message.to_string(),
            severity: self.severity,
        }
    }
}

#[async_trait]
impl AlertRepo for Postgres {
    async fn list_alerts(&self) -> Result<Vec<Alert>, ListAlertsError> {
        let Self { pool } = &self;

        let sql = r#"SELECT * FROM alerts"#;
        let query = sqlx::query_as::<_, AlertModel>(sql);

        match query.fetch_all(pool).await {
            Ok(models) => {
                let alerts = models.into_iter().map(TryInto::try_into);
                Ok(alerts.collect::<Result<_, _>>().unwrap())
            }
            Err(err) => Err(ListAlertsError::Other(err.into())),
        }
    }

    async fn new_alert(&self, new_alert: NewAlert) -> Result<Alert, NewAlertError> {
        let name = AlertName::parse(new_alert.name)
            .map_err(|e| NewAlertError::Other(anyhow::format_err!(e)))?;

        let message = AlertMessage::parse(new_alert.message)
            .map_err(|e| NewAlertError::Other(anyhow::format_err!(e)))?;

        let alert = Alert {
            id: AlertId::new(),
            created_at: new_alert.created_at,
            name: name,
            message: message,
            severity: new_alert.severity,
        };

        tracing::info!("inserting alert in database");
        match self.insert_alert(alert.clone().into()).await {
            Ok(_) => {
                tracing::info!("successfully inserted alert in database");
                Ok(alert)
            }
            Err(err) => {
                tracing::error!("failed to insert alert in database");
                Err(NewAlertError::Other(err.into()))
            }
        }
    }
}
