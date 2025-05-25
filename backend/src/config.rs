use std::fmt::{self, Debug, Formatter};

use ::secrecy::SecretString;
use ::serde::{Deserialize, Serialize};
use ::url::Url;

use crate::repos::GithubConfig;

fn default_admin_email() -> String {
    "admin@localhost".to_string()
}

fn default_admin_password() -> SecretString {
    "admin".into()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SecurityConfig {
    #[serde(default = "default_admin_email")]
    pub admin_email: String,
    #[serde(default = "default_admin_password")]
    pub admin_password: SecretString,
    pub secret: SecretString,
}

#[derive(Deserialize)]
pub struct DatabaseUrl(Url);

impl Debug for DatabaseUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DatabaseUrl({})", self.0.as_str())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DatabaseConfig {
    pub url: DatabaseUrl,
    pub user: String,
    pub password: SecretString,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServicesConfig {
    pub github: GithubConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub security: SecurityConfig,
    pub database: DatabaseConfig,
    pub services: ServicesConfig,
}

impl Config {
    pub async fn init() -> anyhow::Result<Self> {
        let file = config::File::with_name("config.toml");
        let environment = config::Environment::with_prefix("SURF").separator(".");
        let config = config::Config::builder()
            .add_source(file)
            .add_source(environment)
            .build()?;

        let task = tokio::task::spawn_blocking(move || Ok(config.try_deserialize()?));
        task.await.unwrap()
    }
}
