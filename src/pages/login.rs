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

use crate::auth::login;

#[function_component(Login)]
pub fn component() -> Html {
    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Login"}</BBTitle>
            <BBText align={AlignText::Center}>{"Login with your existing account!"}</BBText>
            <BBContainer
                classes={AlignText::Center.class()}
            >
                <BBLink
                    href={login().unwrap()}
                >
                    {"Username and Password"}
                </BBLink>
            </BBContainer>
        </BBContainer>
    }
}
