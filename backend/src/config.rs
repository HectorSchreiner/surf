use ::secrecy::SecretString;
use ::serde::{Deserialize, Serialize};
use ::url::Url;

fn default_admin_email() -> String {
    "admin@localhost".to_string()
}

fn default_admin_password() -> SecretString {
    "admin".into()
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    #[serde(default = "default_admin_email")]
    pub admin_email: String,
    #[serde(default = "default_admin_password")]
    pub admin_password: SecretString,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            admin_email: default_admin_email(),
            admin_password: default_admin_password(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: Url,
    pub user: String,
    pub password: SecretString,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub security: SecurityConfig,
    pub database: DatabaseConfig,
}
