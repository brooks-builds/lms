use graphql_client::GraphQLQuery;

use crate::{
    database_queries::{create_tag, CreateTag},
    errors::LmsError,
};

use super::SendToGraphql;

pub async fn insert_tag(name: impl ToString) -> Result<create_tag::ResponseData, LmsError> {
    let variables = create_tag::Variables {
        name: name.to_string(),
    };
    let body = CreateTag::build_query(variables);

    SendToGraphql::new()
        .json(body)?
        .send::<create_tag::ResponseData>()
        .await
}
