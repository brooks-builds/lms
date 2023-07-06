use std::ops::Deref;

use stylist::{yew::styled_component, Style};
use web_sys::FormData;
use ycl::foundations::states::BBValidationState;
use ycl::modules::password_strength::BBPasswordStrength;
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
    let username_validation_state = use_state(BBValidationState::default);
    let password_validation_state = use_state(BBValidationState::default);
    let (_, dispatch) = use_store::<MainStore>();
    let navigator = use_navigator().unwrap();
    let password_state = use_state(|| AttrValue::from(""));
    let username_value = use_state(AttrValue::default);
    let password_value = use_state(AttrValue::default);

    let onsubmit = {
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

    let username_onisvalid = {
        let username_validation_state = username_validation_state.clone();

        Callback::from(move |is_valid| {
            username_validation_state.set(is_valid);
        })
    };

    let password_onisvalid = {
        let password_validation_state = password_validation_state.clone();

        Callback::from(move |is_valid| {
            password_validation_state.set(is_valid);
        })
    };

    let username_oninput = {
        let value = username_value.clone();

        Callback::from(move |username: AttrValue| {
            value.set(username);
        })
    };

    let password_oninput = {
        let value = password_value.clone();

        Callback::from(move |password: AttrValue| {
            value.set(password);
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
                        input_type={BBInputType::Email}
                        onisvalid={username_onisvalid}
                        is_valid={username_validation_state.deref().clone()}
                        oninput={username_oninput}
                        value={username_value.deref().clone()}
                    />
                    <BBInput
                        id="password"
                        label="password"
                        name="password"
                        input_type={BBInputType::Password}
                        required={true}
                        message="Password requirements: At least 8 characters, no more than 64 characters"
                        onisvalid={password_onisvalid}
                        is_valid={password_validation_state.deref().clone()}
                        oninput={password_oninput}
                        min={8}
                        max={64}
                        validation_debounce={1000}
                        value={password_value.deref().clone()}
                    />
                    <div>
                        <BBPasswordStrength password={password_state.deref().clone()}/>
                    </div>
                    <div>
                        <BBButton
                            button_type={BBButtonType::Submit}
                            button_style={BBButtonStyle::PrimaryLight}
                            classes={classes!(Style::new(css!("margin-top: 1rem;")).unwrap())}
                            disabled={username_validation_state.not_valid() || password_validation_state.not_valid()}
                        >
                            {"Create Account"}
                        </BBButton>
                    </div>
                </BBForm>
            </BBContainer>
        </BBContainer>
    }
}
