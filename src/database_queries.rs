use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries.graphql",
    response_derives = "Debug"
)]
pub struct ListLmsCourses;
