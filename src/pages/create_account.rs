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
use yew_hooks::use_async;

use crate::{
    api,
};

#[styled_component(CreateAccount)]
pub fn component() -> Html {
    let account_state = use_state(|| NewUser::default());

    let create_account_state = {
        let account_state = account_state.clone();
        use_async(async move {
            let email = account_state.email.clone().unwrap();
            let password = account_state.password.clone().unwrap();
            api::create_account(email, password).await
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
                    />
                    <BBInput
                        id="password"
                        label="password"
                        name="password"
                        input_type={BBInputType::Password}
                        required={true}
                        message="Password requirements: 8 characters, 3 of the four types of characters ( a-z, A-Z, 0-9, !@#$%^&*() )"
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
