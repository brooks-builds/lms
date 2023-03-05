pub mod auth;
pub mod courses;

use dotenvy_macro::dotenv;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::errors::LmsError;

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

pub async fn send_to_graphql<RESPONSE: DeserializeOwned>(
    body: impl Serialize,
) -> Result<RESPONSE, LmsError> {
    Ok(gloo::net::http::Request::post(GRAPHQL_URI)
        .json(&body)
        .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
        .send()
        .await
        .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
        .json::<Response<RESPONSE>>()
        .await
        .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
        .data)
}

#[derive(Deserialize)]
pub struct Response<T> {
    data: T,
}
