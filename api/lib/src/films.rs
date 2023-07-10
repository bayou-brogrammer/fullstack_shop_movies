use api_repository::film_repository::FilmRepository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};
use shared::models::{CreateFilm, Film};
use tracing::instrument;

pub fn service<R: FilmRepository + Clone>() -> Router<R> {
    Router::new()
        // get all films
        .route("/", routing::get(get_all::<R>))
        // get by id
        .route("/:film_id", routing::get(get::<R>))
        // post new film
        .route("/", routing::post(post::<R>))
        // update film
        .route("/", routing::put(put::<R>))
        // delete film
        .route("/:film_id", routing::delete(delete::<R>))
}

#[instrument(name = "[films] get all films", skip(film_repo))]
async fn get_all<R: FilmRepository>(film_repo: State<R>) -> impl IntoResponse {
    match film_repo.get_films().await {
        Ok(films) => {
            tracing::info!("Retrieved all films");
            Json(films).into_response()
        }
        Err(e) => {
            tracing::error!("Couldn't retrieve films: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[instrument(name = "[films] get film by id", skip(film_repo))]
async fn get<R: FilmRepository>(
    film_repo: State<R>,
    Path(film_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    match film_repo.get_film(&film_id).await {
        Ok(film) => match film {
            Some(film) => {
                tracing::info!("Retrieved film with id: {}", film_id);
                Json(film).into_response()
            }
            None => {
                tracing::info!("Film with id: {} not found", film_id);
                (StatusCode::NOT_FOUND).into_response()
            }
        },
        Err(e) => {
            tracing::error!("Couldn't retrieve film with id: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[instrument(name = "[films] create film", skip(film_repo))]
async fn post<R: FilmRepository>(
    film_repo: State<R>,
    create_film: Json<CreateFilm>,
) -> impl IntoResponse {
    match film_repo.create_film(&create_film).await {
        Ok(film) => {
            tracing::info!("Created film with id: {}", film.id);
            Json(film).into_response()
        }
        Err(e) => {
            tracing::error!("Couldn't create film: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[instrument(name = "[films] update film", skip(film_repo))]
async fn put<R: FilmRepository>(film_repo: State<R>, update_film: Json<Film>) -> impl IntoResponse {
    match film_repo.update_film(&update_film).await {
        Ok(film) => {
            tracing::info!("Updated film with id: {}", film.id);
            Json(film).into_response()
        }
        Err(e) => {
            tracing::error!("Couldn't update film: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[instrument(name = "[films] delete film", skip(film_repo))]
async fn delete<R: FilmRepository>(
    film_repo: State<R>,
    Path(film_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    match film_repo.delete_film(&film_id).await {
        Ok(film_id) => {
            tracing::info!("Deleted film with id: {}", film_id);
            Json(film_id).into_response()
        }
        Err(e) => {
            tracing::error!("Couldn't update film: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
