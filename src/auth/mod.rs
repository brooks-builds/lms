pub mod handle_redirect;

use crate::{
    errors::LmsError,
    logging::{log_data, log_error},
    utils::cookies::save_cookie,
};
use dotenvy_macro::dotenv;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

static AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");
static AUTH0_CLIENT_ID: &str = dotenv!("AUTH0_CLIENT_ID");
static AUTH_REDIRECT_URI: &str = dotenv!("AUTH_REDIRECT_URI");

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Auth {
    pub logged_in: bool,
}

impl Auth {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn login(&self) -> Result<String, LmsError> {
        let state = self.create_state();

        save_cookie("auth_state", &state, 60 * 5)?;

        Ok(self.create_login_uri(&state))
    }

    fn create_state(&self) -> String {
        let mut rng = thread_rng();
        (0..24).map(|_| rng.sample(Alphanumeric) as char).collect()
    }

    fn create_login_uri(&self, state: &str) -> String {
        format!("{AUTH0_DOMAIN}/authorize?response_type=token&client_id={AUTH0_CLIENT_ID}&redirect_uri={AUTH_REDIRECT_URI}&scope=openid%20profile%20email&state={state}")
    }
}
