use std::ops::Deref;

use ycl::{
    elements::{
        external_link::BBLink,
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::stores::{
    alerts::{AlertsStore, AlertsStoreBuilder},
    auth_store::AuthStore,
};

#[function_component(Login)]
pub fn component() -> Html {
    let (auth_store, _) = use_store::<AuthStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    let login_uri = match auth_store.auth.login() {
        Ok(uri) => uri,
        Err(error) => {
            alert_dispatch.reduce_mut(|store| {
                *store = AlertsStoreBuilder::new()
                    .alert_type(ycl::modules::banner::BBBannerType::Error)
                    .message(error.to_string())
                    .icon(ycl::elements::icon::BBIconType::Warning)
                    .build()
                    .unwrap()
            });
            "".to_owned()
        }
    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Login"}</BBTitle>
            <BBText align={AlignText::Center}>{"Login with your existing account!"}</BBText>
            <BBContainer
                classes={AlignText::Center.class()}
            >
                <BBLink
                    href={login_uri}
                >
                    {"Username and Password"}
                </BBLink>
            </BBContainer>
        </BBContainer>
    }
}
