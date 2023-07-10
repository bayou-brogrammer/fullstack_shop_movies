#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
mod components;
mod models;
mod services;

use components::{FilmModal, Footer, Header};
use dioxus::prelude::*;
use dioxus_signals::*;
use models::FilmModalVisibility;
use services::films::delete_film;
use shared::models::Film;

use crate::components::FilmCard;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));

    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
}

// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    use_init_signal_rt(cx);
    use_shared_state_provider(cx, || FilmModalVisibility(false));
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();

    let force_get_films = use_state(cx, || ());
    let films = use_state::<Option<Vec<Film>>>(cx, || None);
    let mut selected_film = use_signal::<Option<Film>>(cx, || None);

    let delete_film = move |filmId| {
        let force_get_films = force_get_films.clone();
        cx.spawn({
            async move {
                match delete_film(filmId).await {
                    Ok(_data) => {
                        log::info!("Film deleted");
                        force_get_films.set(());
                    }
                    Err(err) => {
                        log::info!("Error deleting film: {:?}", err);
                    }
                }
            }
        });
    };

    let create_or_update_film = move |film: Film| {
        let force_get_films = force_get_films.clone();
        let is_modal_visible = is_modal_visible.clone();
        let mut current_selected_film = selected_film;

        cx.spawn({
            async move {
                let response = if current_selected_film.get().is_some() {
                    services::films::update_film(film).await
                } else {
                    services::films::create_film(film).await
                };
                match response {
                    Ok(_data) => {
                        log::info!("Film created");
                        current_selected_film.set(None);
                        is_modal_visible.write().0 = false;
                        force_get_films.set(());
                    }
                    Err(err) => {
                        log::info!("Error creating film: {:?}", err);
                    }
                }
            }
        });
    };

    {
        let films = films.clone();
        use_effect(cx, force_get_films, |_| async move {
            let existing_films = services::films::get_films().await;
            if existing_films.is_empty() {
                films.set(None);
            } else {
                films.set(Some(existing_films));
            }
        })
    }

    cx.render(rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {},
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
                if let Some(films) = films.get() {
                    rsx!(
                        ul {
                           class: "flex flex-row justify-center items-stretch gap-4 flex-wrap",
                           {films.iter().map(|film| {
                               rsx!(
                                    div{},
                                    FilmCard {
                                        key: "{film.id}",
                                        film: film,
                                        on_edit: move |_| {
                                            selected_film.set(Some(film.clone()));
                                            is_modal_visible.write().0 = true
                                        },
                                        on_delete: move |_| {
                                            delete_film(film.id)
                                        }
                                    }
                                )
                            })}
                        }
                    )
                }
            },
            Footer {},
            FilmModal {
                film: selected_film.get().clone(),
                 on_create_or_update: move |new_film| {
                    create_or_update_film(new_film);
                 },
                 on_cancel: move |_| {
                    selected_film.set(None);
                    is_modal_visible.write().0 = false;
                 }
             }
        }
    })
}
