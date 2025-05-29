use std::fmt::{self, Debug, Formatter};
use std::path::PathBuf;

use ::clap::{ArgAction, Parser};
use ::config::{Case, FileFormat};
use ::secrecy::SecretString;
use ::serde::Deserialize;
use ::url::Url;

use crate::repos::GithubConfig;

fn default_serve_endpoint() -> String {
    String::from("localhost:4000")
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServeConfig {
    #[serde(default = "default_serve_endpoint")]
    pub endpoint: String,
}

fn default_admin_email() -> String {
    "admin@localhost".to_string()
}

fn default_admin_password() -> SecretString {
    "admin".into()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SecurityConfig {
    #[serde(default = "default_admin_email")]
    pub admin_email: String,
    #[serde(default = "default_admin_password")]
    pub admin_password: SecretString,
    pub secret: SecretString,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseUrl(Url);

impl DatabaseUrl {
    pub fn host(&self) -> &str {
        self.0.host_str().unwrap()
    }

    pub fn port(&self) -> u16 {
        self.0.port().unwrap()
    }

    pub fn database(&self) -> &str {
        &self.0.path()[1..]
    }
}

impl Debug for DatabaseUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DatabaseUrl({})", self.0.as_str())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DatabaseConfig {
    pub url: DatabaseUrl,
    pub user: String,
    pub password: SecretString,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServicesConfig {
    pub github: GithubConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub serve: ServeConfig,
    pub security: SecurityConfig,
    pub database: DatabaseConfig,
    pub services: ServicesConfig,
}

impl Config {
    pub async fn init(config_file: &PathBuf) -> anyhow::Result<Self> {
        let config_file = config_file.to_str().unwrap();

        let file = config::File::with_name(config_file)
            .format(FileFormat::Toml)
            .required(false);

        let environment = config::Environment::default()
            .separator(".")
            .convert_case(Case::Kebab);
        let config = config::Config::builder()
            .add_source(file)
            .add_source(environment)
            .build()?;

        let task = tokio::task::spawn_blocking(move || Ok(config.try_deserialize()?));
        task.await.unwrap()
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, value_name = "FILE", default_value = "config.toml")]
    pub config_file: PathBuf,

    #[arg(long, action=ArgAction::SetTrue)]
    pub mock: bool,
}
