use graphql_client::GraphQLQuery;

use crate::{database_queries::{create_lms_account, CreateLmsAccount}, errors::LmsError};

use super::send_to_graphql;

pub async fn create_account(
    email: String,
    password: String,
) -> Result<create_lms_account::ResponseData, LmsError> {
    let variables = create_lms_account::Variables { email, password };
    let query = CreateLmsAccount::build_query(variables);

    let response = send_to_graphql::<create_lms_account::ResponseData>(query).await?;

    Ok(response)
}

