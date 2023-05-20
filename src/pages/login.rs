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

use crate::stores::main_store::MainStore;

#[function_component(Login)]
pub fn component() -> Html {
    let (store, dispatch) = use_store::<MainStore>();

    let login_uri = match auth_store.login() {
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
