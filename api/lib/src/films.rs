use axum::{http::StatusCode, response::IntoResponse, routing, Router};
use tracing::instrument;

pub fn service() -> Router {
    Router::new()
        // get all films
        .route("/", routing::get(get_all))
        // get by id
        .route("/:film_id", routing::get(get))
        // post new film
        .route("/", routing::post(post))
        // update film
        .route("/", routing::put(put))
        // delete film
        .route("/:film_id", routing::delete(delete))
}

#[instrument(name = "[films] get all")]
async fn get_all() -> impl IntoResponse {
    StatusCode::OK
}

#[instrument(name = "[films] get")]
async fn get() -> impl IntoResponse {
    StatusCode::OK
}

#[instrument(name = "[films] post")]
async fn post() -> impl IntoResponse {
    StatusCode::OK
}

#[instrument(name = "[films] put")]
async fn put() -> impl IntoResponse {
    StatusCode::OK
}

#[instrument(name = "[films] delete")]
async fn delete() -> impl IntoResponse {
    StatusCode::OK
}
