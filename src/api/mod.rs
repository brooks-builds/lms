pub mod api_old;
pub mod courses;
pub mod auth;

use serde::{Serialize, de::DeserializeOwned, Deserialize};
use dotenvy_macro::dotenv;

use crate::{errors::LmsError, database_queries::course_by_id::ResponseData};

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

pub async fn send_to_graphql<RESPONSE: DeserializeOwned>(body: impl Serialize) -> Result<RESPONSE, LmsError> {
     Ok(gloo::net::http::Request::post(GRAPHQL_URI)
         .json(&body)
         .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
         .send()
         .await
         .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
         .json::<Response<RESPONSE>>()
         .await
         .map_err(|error| LmsError::SendingToGraphqlApi(error.to_string()))?
         .data
         )
}

#[derive(Deserialize)]
pub struct Response<T> {
    data: T
}
