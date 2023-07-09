use axum::{extract::MatchedPath, http::Request, Router};
use shuttle_runtime::{tracing, CustomError};
use sqlx::Executor;
use sqlx::PgPool;
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::info_span;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        })
        .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .include_headers(true)
                .level(tracing::Level::INFO),
        )
        .on_failure(DefaultOnFailure::new().level(tracing::Level::INFO));

    let router = Router::new()
        .nest(
            "/api",
            Router::new()
                .with_state(pool)
                .nest("/health", api_lib::health::service())
                .nest("/v1/films", api_lib::films::service()),
        )
        .layer(tracing_layer);

    Ok(router.into())
}
