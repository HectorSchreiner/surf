use ::tokio::net::TcpListener;
use config::SecurityConfig;
use services::users::UserService;

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

    tracing::info!("connecting to postgres");
    let postgres = Postgres::connect().await?;
    tracing::info!("successfully connected to postgres");

    let config = SecurityConfig {
        admin_email: "admin@localhost".to_string(),
        admin_password: "password".into(),
    };

    let user_service = UserService::new(postgres.clone(), config).await?;

    let listener = TcpListener::bind("localhost:4000").await?;
    println!("listening on port 4000");
    axum::serve(listener, routes::setup()).await?;

    Ok(())
}
