use std::sync::Arc;

<<<<<<< HEAD
use ::axum::extract::{Json, State};
use ::axum::http::StatusCode;
use ::axum::response::IntoResponse;
use ::chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use ::tokio::sync::Mutex;
use ::uuid::Uuid;

use crate::domains::alerts::*;
use crate::routes::App;
=======
use ::axum::Json;
use ::axum::extract::State;
use ::axum::http::StatusCode;
use ::chrono::Utc;
use ::tokio::sync::Mutex;

use crate::domains::alerts::*;
use crate::repos::postgres::{self, *};
>>>>>>> main

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
<<<<<<< HEAD
pub async fn create_alert(state: State<App>, payload: Json<CreateAlert>) -> impl IntoResponse {
    let App { alerts, .. } = &state.0;
    let Json(payload) = payload;

=======
pub async fn create_alert(
    State(alert_db): State<AlertDB>,
    Json(payload): Json<CreateAlert>,
) -> (StatusCode, Json<Alert>) {
>>>>>>> main
    let alert = Alert {
        id: AlertId::new(),
        created_at: Utc::now(),
<<<<<<< HEAD
=======
        name: payload.name,
>>>>>>> main
        message: payload.message,
        severity: payload.severity,
    };

    alerts.lock().await.push(alert.clone());

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
pub async fn list_alerts(state: State<App>) -> (StatusCode, Json<Vec<Alert>>) {
    let App { alerts, .. } = state.0;
    let alerts = alerts.lock().await.clone();
    (StatusCode::OK, Json::from(alerts))
}
