use shared::models::{CreateFilm, Film};
use uuid::Uuid;

mod memory_film_repository;
mod postgres_film_repository;
pub use memory_film_repository::MemoryFilmRepository;
pub use postgres_film_repository::PostgresFilmRepository;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait::async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films(&self) -> FilmResult<Vec<Film>>;
    async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<Option<Film>>;
    async fn create_film(&self, id: &CreateFilm) -> FilmResult<Film>;
}
