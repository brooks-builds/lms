use ycl::{elements::title::{BBTitle, BBTitleLevel}, foundations::align_text::AlignText};
use yew::{function_component, html, Html};

#[function_component(CreateCourse)]
pub fn component() -> Html {
    html! {
        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Create Course"}</BBTitle>
    }
}
