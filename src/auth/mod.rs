use crate::utils::cookies::save_cookie;
use dotenvy_macro::dotenv;
use eyre::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use yew::AttrValue;

static STATE_COOKIE_KEY: &str = "auth_state";
static STATE_COOKIE_MAX_LIFE: u32 = 60 * 60;
static AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");
static AUTH0_CLIENT_ID: &str = dotenv!("AUTH0_CLIENT_ID");
static AUTH_REDIRECT_URI: &str = dotenv!("AUTH_REDIRECT_URI");

pub fn auth_login_uri() -> Result<AttrValue> {
    let state = create_state();
    save_cookie(STATE_COOKIE_KEY, state.as_str(), STATE_COOKIE_MAX_LIFE)?;
    Ok(AttrValue::from(format!("{AUTH0_DOMAIN}/authorize?response_type=token&client_id={AUTH0_CLIENT_ID}&redirect_uri={AUTH_REDIRECT_URI}&state={state}&scope=openid%20profile%20email&audience=Hasura")))
}

fn create_state() -> AttrValue {
    let mut rng = thread_rng();
    (0..24)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect::<String>()
        .into()
}
