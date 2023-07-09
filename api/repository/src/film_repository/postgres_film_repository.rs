use shared::models::{CreateFilm, Film};

use super::{FilmRepository, FilmResult};

#[derive(Clone, Debug)]
pub struct PostgresFilmRepository {
    pool: sqlx::PgPool,
}

impl PostgresFilmRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FilmRepository for PostgresFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        sqlx::query_as!(
            Film,
            r#"
            SELECT * FROM films
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Option<Film>> {
        sqlx::query_as!(
            Film,
            r#"
            SELECT * FROM films WHERE id = $1
            "#,
            film_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_film(&self, create_film: &CreateFilm) -> FilmResult<Film> {
        sqlx::query_as!(
            Film,
            r#"
            INSERT INTO films (title, director, year, poster)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            &create_film.title,
            &create_film.director,
            create_film.year as i16,
            &create_film.poster,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_film(&self, film: &Film) -> FilmResult<Film> {
        sqlx::query_as!(
            Film,
            r#"
            UPDATE films
            SET title = $2, director = $3, year = $4, poster = $5
            WHERE id = $1
            RETURNING *
            "#,
            film.id,
            &film.title,
            &film.director,
            film.year as i16,
            &film.poster,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_film(&self, film_id: &uuid::Uuid) -> FilmResult<uuid::Uuid> {
        sqlx::query_scalar(
            r#"
            DELETE FROM films WHERE id = $1 RETURNING id
            "#,
        )
        .bind(film_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
