use app::App;

mod api;
mod app;
mod auth;
mod components;
mod database_queries;
mod errors;
mod logging;
mod pages;
mod router;
mod stores;
mod utils;

pub fn run() {
    yew::Renderer::<App>::new().render();
}
