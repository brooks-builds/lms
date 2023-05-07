use serde::Deserialize;
use ycl::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
};
use yew::{function_component, html, Html};

#[derive(PartialEq, Debug, Default, Deserialize)]
pub struct AuthRedirectUser {
    pub scope: String,
    pub expires_in: u32,
    pub token_type: String,
    pub state: String,
}

#[function_component(AuthRedirect)]
pub fn component() -> Html {
    html! {
        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
            {"Logging In"}
        </BBTitle>
    }
}
