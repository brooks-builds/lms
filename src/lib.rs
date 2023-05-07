use app::App;

mod api;
mod app;
mod components;
mod database_queries;
mod errors;
mod logging;
mod pages;
mod router;
mod stores;
mod types;
mod utils;

pub fn run() {
    yew::Renderer::<App>::new().render();
}
