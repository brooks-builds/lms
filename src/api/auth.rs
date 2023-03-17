use crate::{
    database_queries::{create_lms_account, CreateLmsAccount},
    errors::LmsError,
};
use dotenvy_macro::dotenv;
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

use super::SendToGraphql;

static AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");

pub async fn create_account(
    email: String,
    password: String,
) -> Result<create_lms_account::ResponseData, LmsError> {
    let variables = create_lms_account::Variables { email, password };
    let query = CreateLmsAccount::build_query(variables);

    let response = SendToGraphql::new()
        .json(query)?
        .send::<create_lms_account::ResponseData>()
        .await?;

    Ok(response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub nickname: String,
}

pub async fn get_userinfo(token: &str) -> Result<UserInfo, LmsError> {
    let url = format!("{AUTH0_DOMAIN}/userinfo");
    let authorization = format!("Bearer {token}");
    let response = gloo::net::http::Request::get(&url)
        .header("Authorization", &authorization)
        .send()
        .await
        .map_err(|error| LmsError::GettingUserInfo(error.to_string()))?
        .json::<UserInfo>()
        .await
        .map_err(|error| LmsError::GettingUserInfo(error.to_string()))?;
    Ok(response)
}
