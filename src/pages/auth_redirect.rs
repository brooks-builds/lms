use ycl::{elements::title::{BBTitle, BBTitleLevel}, foundations::align_text::AlignText};
use yew::{function_component, Html, html};

#[function_component(AuthRedirect)]
pub fn component() -> Html {
    html! {
        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
            {"Logging In"}
        </BBTitle>
    }
}
