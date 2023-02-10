use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries.graphql"
)]
pub struct ListLmsCourses;
