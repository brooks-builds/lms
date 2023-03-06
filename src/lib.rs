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

pub fn run() {
    yew::Renderer::<App>::new().render();
}
