use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use ::chrono::{DateTime, NaiveDateTime, Utc};
use ::mimalloc::MiMalloc;
use ::serde::Deserialize;
use ::serde_json::Value as JsonValue;
use ::tokio::net::TcpListener;
use ::url::Url;

use crate::config::Config;
use crate::repos::{Github, Postgres};
use crate::routes::App;
use crate::services::users::UserService;
use crate::services::vulnerabilities::{self, VulnerabilityService};

mod config;
mod domains;
mod repos;
mod routes;
mod services;
mod telemetry;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init().await;

    let config = Config::init().await?;
    tracing::info!(?config, "initialized config");

    tracing::info!("connecting to postgres");
    let postgres = Postgres::connect().await?;
    tracing::info!("successfully connected to postgres");

    let github = Github::new(config.services.github).await.unwrap();

    // let contents = fs::read("/home/sebberas/Desktop/surf/CVE-2019-1002100.json")
    //     .await
    //     .unwrap();

    // let record = serde_json::from_slice::<CveRecord>(&contents).unwrap();

    // println!("{record:?}");

    let vulnerability_service = vulnerabilities::Service::new(postgres).await;

    // let user_service = UserService::new(postgres.clone(), config.security).await?;

    let app = App {
        vulnerability_service: Arc::new(vulnerability_service),
        alerts: Arc::default(),
    };

    let listener = TcpListener::bind("localhost:4000").await?;
    tracing::info!("started listening");
    axum::serve(listener, routes::setup(app)).await?;

    Ok(())
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CveTimestamp {
    With(DateTime<Utc>),
    Without(NaiveDateTime),
}

impl CveTimestamp {
    pub fn and_utc(&self) -> DateTime<Utc> {
        match self {
            Self::With(ts) => *ts,
            Self::Without(ts) => ts.and_utc(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CveMeta {
    #[serde(rename = "cveId")]
    pub id: String,
    pub state: String,
    #[serde(rename = "dateReserved")]
    pub reserved_at: CveTimestamp,
    #[serde(rename = "datePublished")]
    pub published_at: Option<CveTimestamp>,
    #[serde(rename = "dateRejected")]
    pub rejected_at: Option<CveTimestamp>,
    #[serde(rename = "dateUpdated")]
    pub updated_at: CveTimestamp,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CveReference {
    pub url: Url,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CveDescription {
    #[serde(rename = "lang")]
    pub language: String,
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CveCnaContainer {
    pub title: Option<String>,
    pub descriptions: Vec<CveDescription>,
    pub references: Vec<CveReference>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CveContainer {
    Cna(CveCnaContainer),
    Adp {},
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CveRecord {
    pub data_type: String,
    pub data_version: String,
    #[serde(rename = "cveMetadata")]
    pub metadata: CveMeta,
    pub containers: HashMap<String, JsonValue>,
}
