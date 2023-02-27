use ycl::{
    elements::{
        external_link::BBLink,
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{align_text::AlignText, container::BBContainer},
};
use yew::prelude::*;

use crate::logging::log;

#[function_component(CreateAccount)]
pub fn component() -> Html {
    let username_password_onclick = Callback::from(|event: ()| {
        log("username/password create account link clicked");
    });

    // 
    todo!("Implement a form to create an account and then send the user to login");
    //
    html! {
        <BBContainer>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{"Create Account"}</BBTitle>
            <BBText align={AlignText::Center}>{"Create an account username/password in order to purchase and complete courses. You can preview courses without creating an account."}</BBText>
        </BBContainer>
    }
}
