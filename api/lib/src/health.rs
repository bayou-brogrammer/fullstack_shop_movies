use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tracing::instrument;

pub const API_VERSION: &str = "v0.0.1";

pub fn service() -> Router {
    Router::new().route("/", get(health_check))
}

#[instrument(name = "health")]
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, [("health-check", API_VERSION)])
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::*;

    #[tokio::test]
    async fn health_check_works() {
        let res = health_check().await.into_response();
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let data = res
            .headers()
            .get("health-check")
            .and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
