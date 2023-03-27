use std::collections::HashMap;

use crate::{
    errors::LmsError,
    utils::cookies::{load_cookie, save_cookie},
};
use dotenvy_macro::dotenv;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use url::form_urlencoded;
use url::Url;
use ycl::foundations::roles::BBRole;
use yewdux::store::Store;

static AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");
static AUTH0_CLIENT_ID: &str = dotenv!("AUTH0_CLIENT_ID");
static AUTH_REDIRECT_URI: &str = dotenv!("AUTH_REDIRECT_URI");
static AUTH_LOGOUT_REDIRECT: &str = dotenv!("LOGOUT_REDIRECT");
static STATE_COOKIE_KEY: &str = "auth_state";
static TOKEN_COOKIE_KEY: &str = "auth_token";
static STATE_COOKIE_MAX_LIFE: u32 = 60 * 5;

#[derive(Clone, PartialEq, Eq, Store, Debug, Default)]
pub struct AuthStore {
    pub logged_in: bool,
    pub access_token: Option<String>,
    pub scope: String,
    pub expires_in: u32,
    pub token_type: String,
    pub nickname: Option<String>,
    pub roles: Vec<BBRole>,
}

impl AuthStore {
    /// # login
    ///
    /// Create the URI for logging in a user using username / password. This has a side
    /// affect of creating a state cookie for use when validating the login token later.
    pub fn login(&self) -> Result<String, LmsError> {
        let state = self.create_state();
        save_cookie(STATE_COOKIE_KEY, &state, STATE_COOKIE_MAX_LIFE)?;

        Ok(self.create_login_uri(&state))
    }

    pub fn handle_redirect(&mut self, uri: &str) -> Result<String, LmsError> {
        let saved_state = load_cookie(STATE_COOKIE_KEY)?;
        let parsed_uri = Url::parse(uri)
            .map_err(|error| LmsError::HandleAuthRedirectError(error.to_string()))?;
        let fragment = parsed_uri
            .fragment()
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting fragment".into()))?;
        let url_encoded = form_urlencoded::parse(fragment.as_bytes()).collect::<HashMap<_, _>>();

        let access_token = url_encoded
            .get("access_token")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting access token".to_owned()))?
            .to_string();
        let scope = url_encoded
            .get("scope")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting scope".to_owned()))?
            .to_string();
        let expires_in = url_encoded
            .get("expires_in")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting expires in".to_owned()))?
            .parse::<u32>()
            .map_err(|error| LmsError::HandleAuthRedirectError(error.to_string()))?;
        let token_type = url_encoded
            .get("token_type")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting token type".to_owned()))?
            .to_string();
        let url_state = url_encoded
            .get("state")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting state".to_owned()))?
            .to_string();

        if let Some(saved_state) = saved_state {
            if saved_state != url_state {
                return Err(LmsError::AuthStateDoesNotMatch);
            }
        } else {
            return Err(LmsError::AuthStateMissing);
        }

        save_cookie(TOKEN_COOKIE_KEY, &access_token, expires_in)?;

        self.logged_in = true;
        self.scope = scope;
        self.expires_in = expires_in;
        self.token_type = token_type;
        self.access_token = Some(access_token.clone());

        Ok(access_token)
    }

    pub fn is_author(&self) -> bool {
        self.roles.contains(&BBRole::Author)
    }

    fn create_state(&self) -> String {
        let mut rng = thread_rng();
        (0..24).map(|_| rng.sample(Alphanumeric) as char).collect()
    }

    fn create_login_uri(&self, state: &str) -> String {
        format!("{AUTH0_DOMAIN}/authorize?response_type=token&client_id={AUTH0_CLIENT_ID}&redirect_uri={AUTH_REDIRECT_URI}&scope=openid%20profile%20email&state={state}&audience=Hasura")
    }
}

pub fn logout_url() -> String {
    format!("{AUTH0_DOMAIN}/v2/logout?returnTo={AUTH_LOGOUT_REDIRECT}&client_id={AUTH0_CLIENT_ID}")
}
