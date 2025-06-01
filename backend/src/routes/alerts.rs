use std::sync::Arc;

use ::axum::Json;
use ::axum::extract::State;
use ::axum::http::StatusCode;
use ::chrono::Utc;
use ::tokio::sync::Mutex;

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
pub async fn create_alert(
    State(alert_db): State<AlertDB>,
    Json(payload): Json<CreateAlert>,
) -> (StatusCode, Json<Alert>) {
    let alert = Alert {
        id: AlertId::new(),
        created_at: Utc::now(),
        name: payload.name,
        message: payload.message,
        severity: payload.severity,
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
