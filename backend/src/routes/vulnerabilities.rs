use ::axum::extract::State;
use ::axum::response::IntoResponse;

use super::*;
use crate::domains::vulnerabilities::{Vulnerability, VulnerabilityId};

#[cfg_attr(feature = "docs", utoipa::path(
        get,
        path = "/api/v1/vulnerabilities",
        responses(
            (status = 200, description = "Successfully listed vulnerabilities", body = Vec<Vulnerability>),
            (status = 500, description = "Failed to list vulnerabilities, because of an internal server error", body=String)
        ),
    ))]
pub async fn list(state: State<App>) -> impl IntoResponse {
    let App { vulnerability_service, .. } = &state.0;

    match vulnerability_service.list_vulnerabilities().await {
        Ok(vulnerabilities) => (StatusCode::OK, Json(vulnerabilities)).into_response(),
        Err(err) => {
            tracing::error!(?err, "failed to list vulnerabilities");
            (StatusCode::INTERNAL_SERVER_ERROR,).into_response()
        }
    }
}
