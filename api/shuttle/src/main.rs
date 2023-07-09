use axum::extract::State;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::Executor;
use sqlx::PgPool;

async fn version(State(pool): State<PgPool>) -> impl IntoResponse {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(&pool)
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let router = Router::new()
        .route("/version", get(version))
        .with_state(pool);

    Ok(router.into())
}
