use axum::Router;
use axum::routing::{get, post};
use http::StatusCode;
use tokio::net::TcpListener;

use crate::repos::Postgres;

mod domains;
mod repos;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let vulnerability_repo = Postgres::connect().await?;

    let listener = TcpListener::bind("localhost:4000").await?;
    axum::serve(listener, routes::setup()).await?;

    Ok(())
}
