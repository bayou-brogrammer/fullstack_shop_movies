use shared::models::Film;

use crate::services::API_ENDPOINT;

fn films_endpoint() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let host = location.host().expect("should have a host");
    let protocol = location.protocol().expect("should have a protocol");
    let endpoint = format!("{}//{}/{}", protocol, host, API_ENDPOINT);
    format!("{}/films", endpoint)
}

pub async fn get_films() -> Vec<Film> {
    log::info!("Fetching films from {}", films_endpoint());
    reqwest::get(&films_endpoint())
        .await
        .unwrap()
        .json::<Vec<Film>>()
        .await
        .unwrap()
}

pub async fn create_film(film: Film) -> Result<reqwest::Response, reqwest::Error> {
    log::info!("Creating film {:?}", film);

    reqwest::Client::new()
        .post(&films_endpoint())
        .json(&film)
        .send()
        .await
}

pub async fn update_film(film: Film) -> Result<reqwest::Response, reqwest::Error> {
    log::info!("Updating film with id {}", film.id);
    reqwest::Client::new()
        .put(&films_endpoint())
        .json(&film)
        .send()
        .await
}

pub async fn delete_film(filmId: uuid::Uuid) -> Result<reqwest::Response, reqwest::Error> {
    log::info!("deleting film with id {}", filmId);
    reqwest::Client::new()
        .delete(&format!("{}/{}", &films_endpoint(), filmId))
        .send()
        .await
}
