use crate::{
    database_queries::{list_lms_courses, ListLmsCourses},
    errors::LmsError,
    stores::courses_store::StoreCourse,
};
use dotenvy_macro::dotenv;
use graphql_client::{GraphQLQuery, Response};

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

pub async fn get_courses() -> Result<Vec<StoreCourse>, LmsError> {
    let graphql_variables = list_lms_courses::Variables {};
    let body = ListLmsCourses::build_query(graphql_variables);

    Ok(gloo::net::http::Request::post(GRAPHQL_URI)
        .json(&body)
        .map_err(|error| {
            LmsError::FetchingCourses("building request json body".into(), error.to_string())
        })?
        .send()
        .await
        .map_err(|error| LmsError::FetchingCourses("getting response".into(), error.to_string()))?
        .json::<Response<list_lms_courses::ResponseData>>()
        .await
        .map_err(|error| LmsError::FetchingCourses("converting to json".into(), error.to_string()))?
        .data
        .ok_or_else(|| {
            LmsError::FetchingCourses(
                "Extracting data from response".into(),
                "data missing".into(),
            )
        })?
        .lms_courses
        .into_iter()
        .map(|api_course| {
            let mut course = StoreCourse::default();
            course.name = api_course.title;
            course.id = api_course.id;
            course.tag = api_course.lms_tag.name.into();
            course.description = api_course.short_description;
            course
        })
        .collect::<Vec<StoreCourse>>())
}
