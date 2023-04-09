pub mod articles;
pub mod auth;
pub mod courses;
pub mod tags;

use dotenvy_macro::dotenv;
use gloo::net::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ycl::foundations::roles::BBRole;

use crate::errors::LmsError;

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

#[derive(Deserialize)]
pub struct Response<T> {
    data: T,
}

pub struct SendToGraphql {
    request: Request,
}

impl SendToGraphql {
    pub fn new() -> Self {
        let request = Request::post(GRAPHQL_URI);
        Self { request }
    }

    pub fn json(mut self, body: impl Serialize) -> Result<Self, LmsError> {
        self.request = self.request.json(&body).map_err(|error| {
            LmsError::SendingToGraphqlApi("adding json".to_owned(), error.to_string())
        })?;

        Ok(self)
    }

    pub async fn send<R: DeserializeOwned>(self) -> Result<R, LmsError> {
        Ok(self
            .request
            .send()
            .await
            .map_err(|error| {
                LmsError::SendingToGraphqlApi("sending".to_owned(), error.to_string())
            })?
            .json::<Response<R>>()
            .await
            .map_err(|error| {
                LmsError::SendingToGraphqlApi(
                    "converting response to json".to_owned(),
                    error.to_string(),
                )
            })?
            .data)
    }

    pub fn authorization(mut self, token: &str) -> Self {
        let bearer_token = format!("Bearer {token}");
        self.request = self.request.header("Authorization", &bearer_token);
        self
    }

    pub fn role(mut self, role: BBRole) -> Self {
        let role_string = role.to_string();
        self.request = self.request.header("x-hasura-role", &role_string);
        self
    }
}
