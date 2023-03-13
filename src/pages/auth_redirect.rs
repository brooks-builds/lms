use crate::{
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::AuthStore,
    }, logging::log_data, api,
};
use serde::Deserialize;
use ycl::{
    elements::{
        icon::BBIconType,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::align_text::AlignText,
};
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

#[derive(PartialEq, Debug, Default, Deserialize)]
pub struct AuthRedirectUser {
    pub scope: String,
    pub expires_in: u32,
    pub token_type: String,
    pub state: String,
}

#[function_component(AuthRedirect)]
pub fn component() -> Html {
    let (_, alert_dispatch) = use_store::<AlertsStore>();
    let (_, auth_dispatch) = use_store::<AuthStore>();
    let navigation = use_navigator().unwrap();

    auth_dispatch.reduce_mut(move |store| {
        wasm_bindgen_futures::spawn_local(async move {
        let uri = gloo::utils::window().location().href().unwrap();
        match store.handle_redirect(&uri) {
            Ok(_) => navigation.push(&Routes::Home),
            Err(error) => alert_dispatch.reduce_mut(|alert_store| {
                *alert_store = AlertsStoreBuilder::new()
                    .icon(BBIconType::Warning)
                    .message("Login timed out, please try again")
                    .alert_type(ycl::modules::banner::BBBannerType::Error)
                    .build()
                    .unwrap();
            }),
        }

        if let Some(token) = store.access_token {
            let user_info = api::auth::get_userinfo(&token).await.unwrap();
        }

        log_data("auth store:", &store);
    });
});

    html! {
        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
            {"Logging In"}
        </BBTitle>
    }
}
