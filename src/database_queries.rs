use graphql_client::GraphQLQuery;

#[allow(non_camel_case_types)]
pub type smallint = u8;

#[allow(non_camel_case_types)]
pub type timestamp = String;

#[allow(non_camel_case_types)]
pub type json = Vec<i64>;

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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries/get_tags.graphql",
    response_derives = "Debug, Clone"
)]
pub struct LmsTags;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries/create_tag.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CreateTag;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries/create_course.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CreateLmsCourse;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries/create_lms_article.graphql",
    response_derives = "Debug, Clone"
)]
pub struct InsertLmsArticle;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries/get_article_titles.graphql",
    response_derives = "Debug, Clone"
)]
pub struct GetLmsArticleTitles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "database/schema.json",
    query_path = "database/queries/set_course_articles.graphql",
    response_derives = "Debug, Clone"
)]
pub struct SetLmsCourseArticles;
