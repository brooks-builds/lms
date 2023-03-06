use stylist::{yew::styled_component, Style};
use web_sys::FormData;
use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle, BBButtonType},
        form::BBForm,
        icon::BBIconType,
        input::{BBInput, BBInputType},
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
    modules::banner::BBBannerType,
};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api,
    logging::log_error,
    router::Routes,
    stores::alerts::{AlertsStore, AlertsStoreBuilder},
};

#[styled_component(CreateAccount)]
pub fn component() -> Html {
    let account_state = use_state(|| NewUser::default());
    let navigator = use_navigator().unwrap();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    let create_account_state = {
        let account_state = account_state.clone();
        use_async(async move {
            let email = account_state.email.clone().unwrap();
            let password = account_state.password.clone().unwrap();
            api::auth::create_account(email, password).await
        })
    };

    let onsubmit = {
        let account_state = account_state.clone();
        let create_account_state = create_account_state.clone();

        Callback::from(move |form_data: FormData| {
            let email = form_data.get("email").as_string().unwrap();
            let password = form_data.get("password").as_string().unwrap();
            account_state.set(NewUser {
                email: Some(email),
                password: Some(password),
            });
            create_account_state.run();
        })
    };

    {
        let create_account_state = create_account_state.clone();
        let navigator = navigator.clone();
        let alert_dispatch = alert_dispatch.clone();

        use_effect(move || {
            if !create_account_state.loading {
                if let Some(_data) = &create_account_state.data {
                    let alert = AlertsStoreBuilder::new()
                        .message("Account Created")
                        .icon(BBIconType::Heart)
                        .build()
                        .unwrap();
                    alert_dispatch.reduce_mut(move |store| *store = alert);
                    navigator.push(&Routes::Login);
                } else if let Some(error) = &create_account_state.error {
                    let alert = AlertsStoreBuilder::new()
                        .message("Error creating account, please try again")
                        .icon(BBIconType::Warning)
                        .alert_type(BBBannerType::Error)
                        .build()
                        .unwrap();
                    alert_dispatch.reduce_mut(|store| *store = alert);
                    log_error("error creating account", error);
                }
            }

            || {}
        });
    }

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
                    />
                    <BBInput
                        id="password"
                        label="password"
                        name="password"
                        input_type={BBInputType::Password}
                        required={true}
                        message="Password requirements: 8 characters, 3 of the four types of characters ( a-z, A-Z, 0-9, !@#$%^&*() )"
                        value={account_state.password.clone().unwrap_or_default()}
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

#[derive(Default)]
struct NewUser {
    pub email: Option<String>,
    pub password: Option<String>,
}
