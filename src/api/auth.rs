use graphql_client::GraphQLQuery;

use crate::{
    database_queries::{create_lms_account, get_user_info, CreateLmsAccount, GetUserInfo},
    errors::LmsError,
};

use super::SendToGraphql;

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

pub async fn get_userinfo(token: &str) -> Result<get_user_info::ResponseData, LmsError> {
    let variables = get_user_info::Variables {};
    let query = GetUserInfo::build_query(variables);
    let response = SendToGraphql::new()
        .auth(token)
        .json(query)?
        .send::<get_user_info::ResponseData>()
        .await?;
    Ok(response)
}
