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

use crate::{
    auth::auth_login_uri,
    stores::main_store::{self, MainStore},
};

#[function_component(Login)]
pub fn component() -> Html {
    let (store, dispatch) = use_store::<MainStore>();

    let login_uri = match auth_login_uri() {
        Ok(uri) => uri,
        Err(error) => {
            gloo::console::error!("Error getting login uri:", error.to_string());
            main_store::error_alert(dispatch, "Error calculating the login uri");
            return html! {};
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
                    href={login_uri.to_string()}
                >
                    {"Username and Password"}
                </BBLink>
            </BBContainer>
        </BBContainer>
    }
}
