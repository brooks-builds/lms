pub mod app;

pub fn run() {
    yew::Renderer::<app::App>::new().render();
}
