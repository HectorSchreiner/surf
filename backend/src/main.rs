use ::tokio::net::TcpListener;
use repos::Github;
use services::users::UserService;

use crate::config::Config;
use crate::repos::Postgres;

mod config;
mod domains;
mod repos;
mod routes;
mod services;
mod telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init().await;

    let config = Config::init().await?;
    tracing::info!(?config, "initialized config");

    tracing::info!("connecting to postgres");
    let postgres = Postgres::connect().await?;
    tracing::info!("successfully connected to postgres");

    let github = Github::new(config.services.github).await.unwrap();

    let user_service = UserService::new(postgres.clone(), config.security).await?;

    let listener = TcpListener::bind("localhost:4000").await?;
    println!("listening on port 4000");
    axum::serve(listener, routes::setup()).await?;

    Ok(())
}
