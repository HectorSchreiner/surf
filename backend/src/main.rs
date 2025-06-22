use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use ::chrono::{DateTime, NaiveDateTime, Utc};
use ::mimalloc::MiMalloc;
use ::serde::Deserialize;
use ::serde_json::Value as JsonValue;
use ::tokio::net::TcpListener;
use ::url::Url;
use tokio::sync::broadcast::error::RecvError;

use crate::config::Config;
use crate::domains::vulnerabilities::{
    NewVulnerability, VulnerabilityReference, VulnerabilityRepo,
};
use crate::repos::{CveCnaContainer, CveCnaPublishedContainer, CveMeta, Github, Postgres};
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

    {
        let postgres = Postgres::clone(&postgres);
        let mut listener = github.listen();
        tokio::spawn(async move {
            loop {
                match listener.recv().await {
                    Ok(record) => match record.meta {
                        CveMeta::Published(meta) => {
                            let cna = record.containers.get("cna").unwrap();
                            let cna: CveCnaPublishedContainer =
                                serde_json::from_value(cna.clone()).unwrap();

                            let new_vulnerability_args = NewVulnerability {
                                key: meta.id.into(),
                                reserved_at: meta.reserved_at,
                                published_at: meta.published_at,
                                rejected_at: None,
                                name: cna.title.unwrap_or_else(|| meta.id.into()),
                                description: cna.descriptions[0].value.clone(),
                                references: cna
                                    .references
                                    .into_iter()
                                    .map(|reference| VulnerabilityReference {
                                        url: reference.url,
                                        name: reference.name,
                                        tags: reference.tags,
                                    })
                                    .collect(),
                            };

                            match postgres.new_vulnerability(new_vulnerability_args).await {
                                Err(err) => {
                                    tracing::error!(?err, "failed to create vulnerability")
                                }
                                _ => {}
                            }
                        }
                        CveMeta::Rejected(meta) => {}
                    },
                    Err(RecvError::Lagged(n)) => {
                        tracing::warn!(?n, "lost vulnerabilities");
                    }
                    Err(RecvError::Closed) => break,
                }
            }
        });
    }

    github.start();

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
