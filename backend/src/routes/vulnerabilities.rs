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
pub async fn list() -> (StatusCode, Json<Vec<Vulnerability>>) {
    let vulnerabilities = vec![Vulnerability {
        id: VulnerabilityId::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        key: "CVE-2025-0001".to_string(),
        reserved_at: None,
        published_at: None,
        name: "skrt".to_string(),
        description: "bob bob".to_string(),
    }];

    (StatusCode::OK, Json(vulnerabilities))
}
