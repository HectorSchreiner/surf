use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::domains::alerts::*;


#[axum::debug_handler]
#[cfg_attr(feature = "docs", utoipa::path(
    post,
    path = "/api/v1/alerts",
    request_body = CreateAlert,
    responses(
        (status = 200, description = "Successfully created alert", body = Alert),
        (status = 500, description = "Failed to create alert, because of an internal server error", body=String)
    ),
))]
pub async fn create_alert(State(alert_db): State<AlertDB>, Json(payload): Json<CreateAlert>) -> (StatusCode, Json<Alert>) {
    let alert = Alert {
        name: payload.name,
        id: Uuid::new_v4(),
        created_at: Utc::now(),
        message: payload.message, 
        severity: payload.severity
    };
    alert_db.lock().await.push(alert.clone());
    (StatusCode::CREATED, Json::from(alert))
}

// skal ændres så det ender i databasen. Sebastian fix pls
type AlertDB = Arc<Mutex<Vec<Alert>>>;

#[cfg_attr(feature = "docs", utoipa::path(
    get,
    path = "/api/v1/alerts",
    responses(
        (status = 200, description = "Successfully listed alerts", body = Vec<Alert>),
        (status = 500, description = "Failed to list alerts, because of an internal server error", body=String)
    ),
))]
pub async fn list_alerts(State(alert_db): State<AlertDB>) -> (StatusCode, Json<Vec<Alert>>) {
    let alerts = alert_db.lock().await.clone();
    (StatusCode::OK, Json::from(alerts))
}

