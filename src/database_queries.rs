use graphql_client::GraphQLQuery;

#[allow(non_camel_case_types)]
pub type smallint = u8;

#[allow(non_camel_case_types)]
pub type timestamp = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries/get_all_courses.graphql",
    response_derives = "Debug"
)]
pub struct ApiGetAllCourses;
