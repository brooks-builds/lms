use graphql_client::GraphQLQuery;

use crate::{
    database_queries::{course_by_id, list_lms_courses, CourseById, ListLmsCourses},
    errors::LmsError,
    logging::log_data,
    stores::courses_store::StoreCourse,
};

use super::send_to_graphql;

pub async fn get() -> Result<Vec<StoreCourse>, LmsError> {
    let graphql_variables = list_lms_courses::Variables {};
    let body = ListLmsCourses::build_query(graphql_variables);
    let response = send_to_graphql::<list_lms_courses::ResponseData>(body, None).await?;

    Ok(response
        .lms_courses
        .into_iter()
        .map(|api_course| {
            log_data("api course", &api_course);
            let mut course = StoreCourse::default();
            course.name = api_course.title;
            course.id = api_course.id;
            course.tag = api_course.lms_tag.name.into();
            course.description = api_course.short_description;
            course.price = api_course.price;
            course.long_description = api_course.long_description;
            course.trailer_uri = api_course.trailer_uri;

            course
        })
        .collect::<Vec<StoreCourse>>())
}

pub async fn get_by_id(id: i64) -> Result<StoreCourse, LmsError> {
    let graphql_variables = course_by_id::Variables { id };
    let body = CourseById::build_query(graphql_variables);

    let response = send_to_graphql::<course_by_id::ResponseData>(body, None).await?;

    if let Some(response_course) = response.lms_courses_by_pk {
        Ok(StoreCourse {
            trailer_uri: response_course.trailer_uri,
            name: response_course.title,
            id: response_course.id,
            description: response_course.short_description,
            tag: response_course.lms_tag.name.into(),
            price: response_course.price,
            long_description: response_course.long_description,
        })
    } else {
        Err(LmsError::CourseNotFound)
    }
}
