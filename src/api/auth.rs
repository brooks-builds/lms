use crate::{auth::auth_login_uri, types::Auth0User};
use dotenvy_macro::dotenv;
use eyre::{eyre, Result};

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

pub fn navigate_to_login() -> Result<()> {
    let login_uri = auth_login_uri()?;

    gloo::utils::window()
        .location()
        .set_href(login_uri.as_str())
        .map_err(|error| {
            gloo::console::error!("Error navigating to login uri", error);

            eyre!("Error navigating to login URI")
        })?;

    Ok(())
}
