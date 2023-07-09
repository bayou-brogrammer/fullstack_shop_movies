use std::{collections::HashMap, sync::RwLock};

use async_trait::async_trait;
use chrono::Utc;
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use super::{FilmRepository, FilmResult};

pub struct MemoryFilmRepository {
    films: RwLock<HashMap<Uuid, Film>>,
}

impl MemoryFilmRepository {
    pub fn new() -> Self {
        Self {
            films: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryFilmRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FilmRepository for MemoryFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        match self.films.read() {
            Ok(films) => Ok(films.clone().into_values().collect::<Vec<_>>()),
            Err(e) => {
                let err = format!("An error happened while trying to read films: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }

    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Option<Film>> {
        match self.films.read() {
            Ok(films) => {
                let result = films.get(film_id).cloned();

                if result.is_none() {
                    tracing::error!("Couldn't retrive a film with id {}", film_id);
                }

                Ok(result)
            }
            Err(e) => {
                let err = format!("An error happened while trying to read films: {}", e);
                tracing::error!(err);
                return Err(err);
            }
        }
    }

    async fn create_film(&self, create_film: &CreateFilm) -> FilmResult<Film> {
        match self.films.write() {
            Ok(mut films) => {
                let new_film = Film {
                    id: uuid::Uuid::new_v4(),
                    title: create_film.title.clone(),
                    director: create_film.director.clone(),
                    year: create_film.year,
                    poster: create_film.poster.clone(),
                    created_at: Some(Utc::now()),
                    updated_at: None,
                };

                films.insert(new_film.id, new_film.clone());
                tracing::trace!("Film with id {} correctly created", new_film.id);
                Ok(new_film)
            }
            Err(e) => {
                let err = format!("An error happened while trying to update film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }

    async fn update_film(&self, film: &Film) -> FilmResult<Film> {
        match self.films.write() {
            Ok(mut films) => {
                let old_film = films.get_mut(&film.id);
                match old_film {
                    Some(old_film) => {
                        let mut updated_film = film.to_owned();
                        updated_film.created_at = old_film.created_at;
                        updated_film.updated_at = Some(Utc::now());

                        films.insert(film.id, updated_film.clone());
                        tracing::debug!("Film with id {} correctly updated", film.id);

                        Ok(updated_film)
                    }
                    None => {
                        let err = format!("Film with id {} does not exist", film.id);
                        tracing::error!(err);
                        Err(err)
                    }
                }
            }
            Err(e) => {
                let err = format!("An error happened while trying to update film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }

    async fn delete_film(&self, film_id: &uuid::Uuid) -> FilmResult<Uuid> {
        match self.films.write() {
            Ok(mut films) => {
                films.remove(film_id);
                Ok(film_id.to_owned())
            }
            Err(e) => {
                let err = format!("An error happened while trying to delete film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }
}
