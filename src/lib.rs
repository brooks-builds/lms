use app::App;

mod app;
mod pages;
mod router;

pub fn run() {
    yew::Renderer::<App>::new().render();
}
