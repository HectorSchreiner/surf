use std::sync::Arc;

use ::axum::extract::{Json, State};
use ::axum::http::StatusCode;
use ::axum::response::IntoResponse;
use ::chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use ::tokio::sync::Mutex;
use ::uuid::Uuid;

use crate::domains::alerts::*;
use crate::routes::App;

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
pub async fn create_alert(state: State<App>, payload: Json<CreateAlert>) -> impl IntoResponse {
    let App { alerts, .. } = &state.0;
    let Json(payload) = payload;

    let alert = Alert {
        name: payload.name,
        id: Uuid::new_v4(),
        created_at: Utc::now(),
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
