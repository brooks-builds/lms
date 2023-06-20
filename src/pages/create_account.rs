use std::ops::Deref;

use stylist::{yew::styled_component, Style};
use web_sys::FormData;
use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle, BBButtonType},
        form::BBForm,
        input::{BBInput, BBInputType},
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api,
    router::Routes,
    stores::main_store::{self, MainStore},
};

#[styled_component(CreateAccount)]
pub fn component() -> Html {
    let account_state = use_state(NewUser::default);
    let (_, dispatch) = use_store::<MainStore>();
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let account_state = account_state.clone();

        Callback::from(move |form_data: FormData| {
            let email = form_data.get("email").as_string().unwrap();
            let password = form_data.get("password").as_string().unwrap();
            let dispatch = dispatch.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let Err(error) = api::create_account(email, password).await else {
                    main_store::set_alert(dispatch, "Account created, please log to complete registration");
                    navigator.push(&Routes::Login);
                    return;
                };
                gloo::console::error!("Error creating account", error.to_string());
                main_store::error_alert(
                    dispatch,
                    "There was an error creating the account, please try again",
                );
            })
        })
    };

    let username_onchange = {
        let account_state = account_state.clone();

        Callback::from(move |email: AttrValue| {
            let mut state = account_state.deref().clone();
            state.email = Some(email.to_string());
            account_state.set(state);
        })
    };

    let password_onchange = {
        let account_state = account_state.clone();

        Callback::from(move |password: AttrValue| {
            let mut state = account_state.deref().clone();
            state.password = Some(password.to_string());
            account_state.set(state);
        })
    };

    html! {
        <BBContainer>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{"Create Account"}</BBTitle>
            <BBText align={AlignText::Center}>{"Create an account username/password in order to purchase and complete courses. You can preview courses without creating an account."}</BBText>
            <BBContainer margin={BBContainerMargin::Normal}>
                <BBForm {onsubmit}>
                    <BBInput
                        id="email"
                        label="email"
                        name="email"
                        required={true}
                        value={account_state.email.clone().unwrap_or_default()}
                        onchange={username_onchange}
                    />
                    <BBInput
                        id="password"
                        label="password"
                        name="password"
                        input_type={BBInputType::Password}
                        required={true}
                        message="Password requirements: 8 characters, 3 of the four types of characters ( a-z, A-Z, 0-9, !@#$%^&*() )"
                        value={account_state.password.clone().unwrap_or_default()}
                        onchange={password_onchange}
                    />
                    <div>
                        <BBButton
                            button_type={BBButtonType::Submit}
                            button_style={BBButtonStyle::PrimaryLight}
                            classes={classes!(Style::new(css!("margin-top: 1rem;")).unwrap())}
                        >
                            {"Create Account"}
                        </BBButton>
                    </div>
                </BBForm>
            </BBContainer>
        </BBContainer>
    }
}

#[derive(Default, Clone)]
struct NewUser {
    pub email: Option<String>,
    pub password: Option<String>,
}
