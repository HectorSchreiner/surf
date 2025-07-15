use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use ::chrono::{DateTime, NaiveDateTime, Utc};
use ::mimalloc::MiMalloc;
use ::serde::Deserialize;
use ::serde_json::Value as JsonValue;
use ::tokio::net::TcpListener;
<<<<<<< HEAD
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
=======
use clap::Parser;
use config::Args;
use repos::Mock;

use crate::config::{Config, ServeConfig};
use crate::repos::{Github, Postgres};
use crate::services::users::UserService;
>>>>>>> main

mod config;
mod domains;
mod repos;
mod routes;
mod services;
mod telemetry;

<<<<<<< HEAD
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
=======
struct Services {
    user_service: UserService,
}

async fn setup_services(config: Config) -> anyhow::Result<Services> {
    tracing::info!("connecting to postgres");
    let postgres = Postgres::connect(config.database).await?;
    tracing::info!("successfully connected to postgres");

    Ok(Services {
        user_service: UserService::new(postgres.clone(), config.security).await?,
    })
}

async fn setup_mock_services(config: Config) -> Services {
    let repo = Mock::default();

    Services {
        user_service: UserService::new(repo, config.security).await.unwrap(),
    }
}
>>>>>>> main

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init().await;

    let Args { config_file, mock } = Args::parse();
    let config = Config::init(&config_file).await?;
    tracing::info!(?config, "initialized config");

    let services = if !mock {
        setup_services(config.clone()).await?
    } else {
        tracing::warn!("using memory mocking configuration");
        setup_mock_services(config.clone()).await
    };

    // let github = Github::new(config.services.github).await.unwrap();

<<<<<<< HEAD
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

    // github.start();

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
=======
    let ServeConfig { endpoint } = config.serve;
    let listener = TcpListener::bind(endpoint.clone()).await?;
    tracing::info!(?endpoint, addr = ?listener.local_addr().unwrap(), "listening");
    axum::serve(listener, routes::setup()).await?;
>>>>>>> main

    Ok(())
}
