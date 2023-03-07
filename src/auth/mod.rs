pub mod handle_redirect;

use crate::{
    errors::LmsError,
    logging::{log_data, log_error},
};
use dotenvy_macro::dotenv;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

static AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");
static AUTH0_CLIENT_ID: &str = dotenv!("AUTH0_CLIENT_ID");
static AUTH_REDIRECT_URI: &str = dotenv!("AUTH_REDIRECT_URI");

pub fn login() -> Result<String, LmsError> {
    let state = create_state();
    store_state(&state)?;
    Ok(create_login_uri(&state))
}

fn create_login_uri(state: &str) -> String {
    format!("{AUTH0_DOMAIN}/authorize?response_type=token&client_id={AUTH0_CLIENT_ID}&redirect_uri={AUTH_REDIRECT_URI}&scope=openid%20profile%20email&state={state}")
}

fn create_state() -> String {
    let mut rng = thread_rng();

    (0..24).map(|_| rng.sample(Alphanumeric) as char).collect()
}

/// # Store the Auth0 state
///
/// We are storing the state into a cookie so that we can check the state they send back and verify
/// it is them
///
/// Read more about storing into cookies at [MDN](https://developer.mozilla.org/en-US/docs/web/api/document/cookie)
fn store_state(state: &str) -> Result<(), LmsError> {
    let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
    let max_age = 60 * 5; // in seconds
    let cookie = format!("authstate={state}; max-age={max_age}; samesite=strict; secure");
    document.set_cookie(&cookie).map_err(|_error| {
        let error = LmsError::SavingCookie;
        log_error("Error storing state into cookie", &error);
        error
    })?;
    Ok(())
}
