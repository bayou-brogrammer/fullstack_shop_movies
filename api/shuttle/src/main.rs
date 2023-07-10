use std::path::PathBuf;

use api_repository::film_repository::PostgresFilmRepository;
use axum::{extract::MatchedPath, http::Request, Router};
use shuttle_runtime::{tracing, CustomError};
use sqlx::{Executor, PgPool};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::info_span;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    // create a film repository. In this case for postgres.
    let serve_dir = ServeDir::new(static_folder).fallback(ServeFile::new("index.html"));

    let router = Router::new()
        .nest("/api", api_router(pool))
        .nest_service("/", serve_dir)
        .layer(
            TraceLayer::new_for_http()
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
                .on_failure(DefaultOnFailure::new().level(tracing::Level::INFO)),
        );

    Ok(router.into())
}

fn api_router(pool: PgPool) -> Router {
    let film_repository = PostgresFilmRepository::new(pool);
    Router::new()
        .nest(
            "/v1/films",
            api_lib::films::service::<PostgresFilmRepository>(),
        )
        .with_state(film_repository)
        .nest("/health", api_lib::health::service())
}
