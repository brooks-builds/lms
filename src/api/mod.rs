pub mod auth;
pub mod courses;

use dotenvy_macro::dotenv;
use gloo::net::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::errors::LmsError;

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

pub async fn send_to_graphql<RESPONSE: DeserializeOwned>(
    body: impl Serialize,
    token: Option<&str>,
) -> Result<RESPONSE, LmsError> {
    let request = gloo::net::http::Request::post(GRAPHQL_URI);
    request
        .json(&body)
        .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
        .send()
        .await
        .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
        .json::<Response<RESPONSE>>()
        .await
        .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
        .data;

    todo!()
}

#[derive(Deserialize)]
pub struct Response<T> {
    data: T,
}

pub struct SendToGraphql {
    request: Request
}

impl SendToGraphql {
    pub fn new() -> Self {
        let request = Request::post(GRAPHQL_URI);
        Self {request}
    }

    pub fn auth(mut self, token: &str) -> Self {
        self.request = self.request.header("Authorization", &format!("Bearer {token}"));
        self
    }

    pub fn json(mut self, body: impl Serialize) -> Result<Self, LmsError> {
        self.request = self.request.json(&body)
        .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?;

        Ok(self)
    }

    pub async fn send<R: DeserializeOwned>(mut self) -> Result<R, LmsError> {
        Ok(self.request.send().await.map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
        .json::<Response<R>>()
        .await
        .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
        .data
           )
    }
}
