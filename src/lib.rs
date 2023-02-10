use app::App;

mod api;
mod app;
mod database_queries;
mod errors;
mod pages;
mod router;
mod stores;

pub fn run() {
    yew::Renderer::<App>::new().render();
}
