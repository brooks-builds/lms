use crate::{
    auth::handle_redirect::HandleAuthRedirectUser,
    logging::{log_data, log_error},
    stores::alerts::{AlertsStore, AlertsStoreBuilder},
    utils::cookies::load_cookie,
};
use serde::Deserialize;
use url::Url;
use ycl::{
    elements::{
        icon::BBIconType,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::align_text::AlignText,
};
use yew::{function_component, html, Html};
use yew_router::prelude::use_location;
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
    let uri = gloo::utils::window().location().href().unwrap();
    let handle_auth_redirect = HandleAuthRedirectUser::new(&uri).unwrap();
    if let Ok(cookie) = load_cookie("auth_state") {
        if let Some(state) = cookie {
            // compare state to make sure it's good
        } else {
            alert_dispatch.reduce_mut(|store| {
                *store = AlertsStoreBuilder::new()
                    .icon(BBIconType::Warning)
                    .message("Login timed out, please try again")
                    .alert_type(ycl::modules::banner::BBBannerType::Error)
                    .build()
                    .unwrap();
            });
        }
    } else {
        alert_dispatch.reduce_mut(|store| {
            *store = AlertsStoreBuilder::new()
                .icon(BBIconType::Warning)
                .message("Could not log in please try again")
                .alert_type(ycl::modules::banner::BBBannerType::Error)
                .build()
                .unwrap();
        });
    };

    html! {
        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
            {"Logging In"}
        </BBTitle>
    }
}
