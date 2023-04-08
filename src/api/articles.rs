use graphql_client::GraphQLQuery;
use ycl::foundations::roles::BBRole;

use crate::api::SendToGraphql;
use crate::database_queries::{insert_lms_article, InsertLmsArticle};
use crate::errors::LmsError;

pub async fn create_article(
    title: String,
    content: String,
    token: String,
) -> Result<insert_lms_article::ResponseData, LmsError> {
    let variables = insert_lms_article::Variables { title, content };
    let query = InsertLmsArticle::build_query(variables);
    SendToGraphql::new()
        .json(query)?
        .authorization(&token)
        .role(BBRole::Author)
        .send()
        .await
}
