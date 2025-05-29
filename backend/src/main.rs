use ::tokio::net::TcpListener;
use clap::Parser;
use config::Args;
use repos::Mock;

use crate::config::{Config, ServeConfig};
use crate::repos::{Github, Postgres};
use crate::services::users::UserService;

mod config;
mod domains;
mod repos;
mod routes;
mod services;
mod telemetry;

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

    let ServeConfig { endpoint } = config.serve;
    let listener = TcpListener::bind(endpoint.clone()).await?;
    tracing::info!(?endpoint, addr = ?listener.local_addr().unwrap(), "listening");
    axum::serve(listener, routes::setup()).await?;

    Ok(())
}
