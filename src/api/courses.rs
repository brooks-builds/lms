use graphql_client::GraphQLQuery;
use ycl::foundations::roles::BBRole;

use crate::{
    database_queries::{api_get_all_courses, ApiGetAllCourses},
    errors::LmsError,
    stores::courses_store::{StoreCourse, StoreTag},
    types::Course,
};

use super::SendToGraphql;

pub async fn get_all_courses(token: Option<&str>, role: BBRole) -> eyre::Result<Vec<Course>> {
    let variables = api_get_all_courses::Variables {};
    let query = ApiGetAllCourses::build_query(variables);
    let mut request = SendToGraphql::new().json(query)?.role(role);

    if let Some(token) = token {
        request = request.authorization(token);
    }

    let response = request.send::<api_get_all_courses::ResponseData>().await?;

    Ok(response.lms_courses.into_iter().map(Into::into).collect())
}
