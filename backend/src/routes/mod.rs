use std::sync::Arc;

use ::axum::Router;
use ::axum::response::{Html, Json};
use ::axum::routing::{get, post};
use ::chrono::Utc;
use ::http::StatusCode;
use ::tokio::sync::Mutex;
use ::uuid::Uuid;

use crate::domains::alerts::Alert;
use crate::routes::alerts::{create_alert, list_alerts};

mod alerts;
mod users;
mod vulnerabilities;

pub fn setup() -> Router {
    let router = Router::new()
        .route("/api/v1/vulnerabilities", get(vulnerabilities::list))
        .route("/api/v1/alerts", get(list_alerts))
        .route("/api/v1/alerts", post(create_alert))
        .with_state(Arc::new(Mutex::new(Vec::new())));

    #[cfg(feature = "docs")]
    let router = router
        .route("/api/schema", get(schema))
        .route("/api", get(docs));

    router
}

#[cfg(feature = "docs")]
#[derive(utoipa::OpenApi)]
#[openapi(paths(vulnerabilities::list, alerts::list_alerts, alerts::create_alert))]
struct ApiDocs;

#[cfg(feature = "docs")]
async fn schema() -> Json<utoipa::openapi::OpenApi> {
    use ::utoipa::OpenApi;

    Json(ApiDocs::openapi())
}

#[cfg(feature = "docs")]
async fn docs() -> Html<String> {
    use utoipa::OpenApi;
    use utoipa_redoc::Redoc;

    Html(Redoc::new(ApiDocs::openapi()).to_html())
}
