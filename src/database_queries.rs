#![allow(non_camel_case_types)]
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
pub struct ApiGetAllData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/mutations/api_insert_tag.graphql",
    response_derives = "Debug"
)]
pub struct ApiInsertTag;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/mutations/api_insert_course.graphql",
    response_derives = "Debug"
)]
pub struct ApiInsertCourse;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/mutations/api_insert_article.graphql",
    response_derives = "Debug"
)]
pub struct ApiInsertArticle;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/mutations/api_insert_course_articles.graphql",
    response_derives = "Debug"
)]
pub struct ApiInsertCourseArticles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/mutations/api_insert_user_article.graphql",
    response_derives = "Debug"
)]
pub struct ApiInsertUserArticle;
