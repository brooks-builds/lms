use stylist::{yew::styled_component, Style};
use web_sys::FormData;
use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle, BBButtonType},
        external_link::BBLink,
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

use crate::logging::{log, log_data};

#[styled_component(CreateAccount)]
pub fn component() -> Html {
    let onsubmit = Callback::from(|form_data: FormData| {
        let email = form_data.get("email").as_string().unwrap();
        let password = form_data.get("password").as_string().unwrap();
        log_data("form submitted:", (email, password));
    });

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
