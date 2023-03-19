use crate::{
    api,
    logging::{log_error},
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::AuthStore,
    },
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

    wasm_bindgen_futures::spawn_local(async move {
        let alert_dispatch = alert_dispatch.clone();
        let navigation = navigation.clone();

        auth_dispatch
            .reduce_mut_future(move |state| {
                let alert_dispatch = alert_dispatch.clone();
                let navigation = navigation.clone();

                Box::pin(async move {
                    let url = gloo::utils::window().location().href().unwrap();
                    match state.handle_redirect(&url) {
                        Ok(token) => {
                            match api::auth::get_userinfo(&token).await {
                                Ok(userinfo) => {
                                    state.nickname = Some(userinfo.nickname);
                                    state.roles = userinfo.brooks_builds.roles;
                                    navigation.push(&Routes::Home);
                                },
                                Err(error) => {
                                    log_error("error handling auth redirect", &error);
                                    alert_dispatch.reduce_mut(move |alert_state| {
                                        *alert_state = AlertsStoreBuilder::new()
                                            .icon(BBIconType::Warning)
                                            .message("Encountered an error when attempting to get your userdata")
                                            .alert_type(ycl::modules::banner::BBBannerType::Error)
                                            .build()
                                            .unwrap()
                                    });
                                }
                            }
                        }
                        Err(error) => {
                            log_error("error handling auth redirect", &error);
                            alert_dispatch.reduce_mut(move |alert_state| {
                                *alert_state = AlertsStoreBuilder::new()
                                    .icon(BBIconType::Warning)
                                    .message("Encountered an error when logging you in, please try again")
                                    .alert_type(ycl::modules::banner::BBBannerType::Error)
                                    .build()
                                    .unwrap()
                            });
                        }
                    }
                })
            })
            .await;
    });

    html! {
        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
            {"Logging In"}
        </BBTitle>
    }
}
