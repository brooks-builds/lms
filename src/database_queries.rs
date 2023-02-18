use graphql_client::GraphQLQuery;

pub type smallint = u8;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries.graphql",
    response_derives = "Debug"
)]
pub struct ListLmsCourses;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries.graphql",
    response_derives = "Debug"
)]
pub struct CourseById;
