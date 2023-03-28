use graphql_client::GraphQLQuery;
use ycl::foundations::roles::BBRole;

use crate::{
    database_queries::{create_tag, CreateTag},
    errors::LmsError,
};

use super::SendToGraphql;

pub async fn insert_tag(
    name: impl ToString,
    token: &str,
) -> Result<create_tag::ResponseData, LmsError> {
    let variables = create_tag::Variables {
        name: name.to_string(),
    };
    let body = CreateTag::build_query(variables);

    SendToGraphql::new()
        .authorization(token)
        .role(BBRole::Author)
        .json(body)?
        .send::<create_tag::ResponseData>()
        .await
}
