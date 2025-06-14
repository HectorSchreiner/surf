use std::borrow::Cow;
use std::num::NonZeroUsize;

use ::anyhow::anyhow;
use ::axum::Json;
use ::axum::extract::{Query, State};
use ::axum::response::IntoResponse;
use ::http::StatusCode;
use ::serde::{Deserialize, Serialize};
use ::serde_with::{TryFromInto, serde_as};

use crate::domains::vulnerabilities::{Vulnerability, VulnerabilityId};
use crate::routes::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListPageSize(NonZeroUsize);

impl ListPageSize {
    pub fn get(&self) -> usize {
        self.0.get()
    }
}

impl TryFrom<usize> for ListPageSize {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > 0 && value <= 10_000 {
            Ok(Self(NonZeroUsize::new(value).unwrap()))
        } else {
            Err(anyhow!(
                "invalid value (got: {value:?}, expected: 1..=10000)"
            ))
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "docs", derive(utoipa::IntoParams))]
#[serde(rename_all = "camelCase")]
pub struct ListParams {
    pub page: usize,
    #[serde_as(as = "TryFromInto<usize>")]
    #[cfg_attr(feature = "docs", param(value_type = usize, minimum = 1, maximum = 10000))]
    pub page_size: ListPageSize,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "docs", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total_pages: usize,
    pub total_items: usize,
    pub page: usize,
    pub page_size: usize,
}

#[cfg_attr(feature = "docs", utoipa::path(
        get,
        path = "/api/v1/vulnerabilities",
        description = "Retrieves pages of vulnerabilities",
        responses(
            (status = 200, description = "Successfully retrieved vulnerabilities page", body = Paginated<Vulnerability>),
            (status = 404, description = "Failed to retrieve vulnerabilities page, because it doesn't exist", body=String),
            (status = 500, description = "Failed to retrieve vulnerabilities page vulnerabilities, because of an internal error", body=String)
        ),
        params(
            ListParams
        )
    ))
]
#[tracing::instrument(skip(state), name = "routes::vulnerabilities::list")]
pub async fn list(state: State<App>, q: Query<ListParams>) -> impl IntoResponse {
    let App { vulnerability_service, .. } = &state.0;
    let ListParams { page, page_size } = q.0;

    tracing::info!("started to list vulnerabilities");
    match vulnerability_service.list_vulnerabilities().await {
        Ok(vulnerabilities) => {
            tracing::info!("successfully listed vulnerabilities");

            let total_items = vulnerabilities.len();
            let total_pages = vulnerabilities.len() / page_size.get();

            let page_start = page * page_size.get();
            let page_end = (page + 1) * page_size.get();

            if let Some(vulnerabilities) = vulnerabilities.get(page_start..page_end) {
                let vulnerabilities = Paginated {
                    items: vulnerabilities.to_vec(),
                    total_items,
                    total_pages,
                    page,
                    page_size: page_size.get(),
                };

                tracing::info!("started to encode response");
                let response = (StatusCode::OK, Json(vulnerabilities)).into_response();
                tracing::info!("finished encoding response");

                return response;
            } else {
                todo!()
            }
        }
        Err(err) => {
            tracing::error!(?err, "failed to list vulnerabilities");
            return (StatusCode::INTERNAL_SERVER_ERROR,).into_response();
        }
    }
}
