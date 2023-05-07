use crate::types::{Auth0User, User};
use dotenvy_macro::dotenv;
use eyre::Result;

static AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");

pub async fn get_user_info(token: &str) -> Result<Auth0User> {
    let url = format!("{AUTH0_DOMAIN}/userinfo");
    let bearer = format!("Bearer {token}");

    let result = gloo::net::http::Request::get(&url)
        .header("Authorization", &bearer)
        .send()
        .await?
        .json::<Auth0User>()
        .await?;

    Ok(result)
}
