use std::sync::Arc;
use crate::routes::alerts::{create_alert, list_alerts};
use alerts::Alert;
use ::axum::Router;
use ::axum::response::{Html, Json};
use ::axum::routing::{get, post};
use ::chrono::Utc;
use ::http::StatusCode;
use tokio::sync::Mutex;
use ::uuid::Uuid;

mod alerts;
mod users;

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

mod vulnerabilities {
    use super::*;
    use crate::domains::vulnerabilities::Vulnerability;

    #[cfg_attr(feature = "docs", utoipa::path(
        get,
        path = "/api/v1/vulnerabilities",
        responses(
            (status = 200, description = "Successfully listed vulnerabilities", body = Vec<Vulnerability>),
            (status = 500, description = "Failed to list vulnerabilities, because of an internal server error", body=String)
        ),
    ))]
    pub async fn list() -> (StatusCode, Json<Vec<Vulnerability>>) {
        let vulnerabilities = vec![Vulnerability {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: "skrt".to_string(),
            description: "bob bob".to_string(),
            key: "CVE-2025-0001".to_string(),
        }];

        (StatusCode::OK, Json(vulnerabilities))
    }
}
