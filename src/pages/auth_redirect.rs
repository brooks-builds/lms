use ycl::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
};
use yew::{function_component, html, Html};
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::main_store::{self, MainStore},
};

#[function_component(AuthRedirect)]
pub fn component() -> Html {
    let (_, dispatch) = use_store::<MainStore>();
    let navigator = use_navigator().unwrap();

    use_effect_once(move || {
        wasm_bindgen_futures::spawn_local(async move {
            main_store::login_from_redirect(dispatch).await;
            navigator.push(&Routes::Home);
        });

        || {}
    });

    html! {
        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
            {"Logging In"}
        </BBTitle>
    }
}
