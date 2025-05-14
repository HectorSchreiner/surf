use ::axum::Router;
use ::axum::response::Html;
use ::axum::routing::{get, post};
use ::http::StatusCode;

pub fn setup() -> Router {
    let router = Router::new().route("/api/v1/vulnerabilities", get(vulnerabilities::list));

    #[cfg(feature = "docs")]
    let router = router.route("/api", get(docs));

    router
}

#[cfg(feature = "docs")]
#[derive(utoipa::OpenApi)]
#[openapi(paths(vulnerabilities::list))]
struct ApiDocs;

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
        ),
    ))]
    pub async fn list() -> (StatusCode,) {
        (StatusCode::OK,)
    }
}
