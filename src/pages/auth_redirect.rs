use crate::{
    auth::handle_redirect::HandleAuthRedirectUser,
    logging::{log_data, log_error},
};
use serde::Deserialize;
use url::Url;
use ycl::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
};
use yew::{function_component, html, Html};
use yew_router::prelude::use_location;

#[derive(PartialEq, Debug, Default, Deserialize)]
pub struct AuthRedirectUser {
    pub scope: String,
    pub expires_in: u32,
    pub token_type: String,
    pub state: String,
}

#[function_component(AuthRedirect)]
pub fn component() -> Html {
    let uri = gloo::utils::window().location().href().unwrap();
    let handle_auth_redirect = HandleAuthRedirectUser::new(&uri).unwrap();

    log_data("handle auth redirect", handle_auth_redirect);

    html! {
        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
            {"Logging In"}
        </BBTitle>
    }
}

// http://localhost:8082/auth/redirect#access_token=eyJhbGciOiJkaXIiLCJlbmMiOiJBMjU2R0NNIiwiaXNzIjoiaHR0cHM6Ly9kZXYtdW55aGE0YWkudXMuYXV0aDAuY29tLyJ9..LDt3ZUq2Xgz7QHPD.cqz6B3ypnW_oDDduk0bjZufn5rR710hY2x7l3Oylrn5EjOYhtbQ2qU_qJvRBRmuiZO5AGUuG2px2rWRjO4Yo1LXarJd6NPjCTgc8QAUaypdeoWot14_ecDwCsPVzJwuYDVdr6V052bEjygRTzmVtCzrnIYk40VBbc3QWqxm0urzQ1Tn_6yLGaIAr5-HPrKask8ZeeATq-FlNg48gZRcXT6ckBIUZj20xD5fVTkhSFxKxb6rChkvJ5P7rNgFZLkQPG30HZoGxso3JcE6aBFy-xld5xQSIDA-4R4cYcQH4mLF6o3t7YVJK6YEROIE85DVDcV5OLZ2NI6N5XfkYpSx334n6C-EsRURVRgkAlB61CdPgwycd_EBOp9vBiPrrX5NvZeDXf7rxrTe-ucOJa79Zaf8FIRXdz27tOxDqgxU6lgsraoP5gIvlW2qaceFrx6y7lG5-NbQR4acYSH3EW-Ih03_p88NOW-tMTOFdOyd0CLdDtRFDdcHuJbGpVObvfaNVlVrT9832y1g.cLkIxwsNtzdPLb-Zyh05VA&scope=openid%20profile%20email&expires_in=7200&token_type=Bearer&state=SsbzpPzLJ9yNJumpVGN2HKqt
