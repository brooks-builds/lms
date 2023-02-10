use crate::{
    database_queries::{list_lms_courses, ListLmsCourses},
    errors::LmsError,
    stores::courses_store::StoreCourse,
};
use dotenvy_macro::dotenv;
use graphql_client::GraphQLQuery;

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

pub async fn get_courses() -> Result<Vec<StoreCourse>, LmsError> {
    let graphql_variables = list_lms_courses::Variables {};
    let body = ListLmsCourses::build_query(graphql_variables);

    gloo::console::log!("graphql uri", GRAPHQL_URI);

    Ok(vec![])
}
