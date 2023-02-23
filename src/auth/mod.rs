use dotenvy_macro::dotenv;
use rand::{distributions::Alphanumeric, rngs::ThreadRng, thread_rng, Rng};

use crate::logging::log_data;

static AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");
static AUTH0_CLIENT_ID: &str = dotenv!("AUTH0_CLIENT_ID");
static AUTH_REDIRECT_URI: &str = dotenv!("AUTH_REDIRECT_URI");

pub fn init() {
    let state = create_state();
    store_state(&state);
}

pub fn create_login_uri() -> String {
    format!("{AUTH0_DOMAIN}/authorize?response_type=token&client_id={AUTH0_CLIENT_ID}&redirect_uri={AUTH_REDIRECT_URI}&scope=openid%20profile%20email")
}

fn create_state() -> String {
    let mut rng = thread_rng();

    (0..24).map(|_| rng.sample(Alphanumeric) as char).collect()
}

fn store_state(state: &str) {
    todo!("set cookie, we'll need to get the html document to do it")
}
