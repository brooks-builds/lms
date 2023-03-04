use graphql_client::GraphQLQuery;

#[allow(non_camel_case_types)]
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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries/create_lms_account.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CreateLmsAccount;
