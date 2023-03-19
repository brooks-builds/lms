use graphql_client::GraphQLQuery;

use crate::{
    database_queries::{course_by_id, list_lms_courses, CourseById, ListLmsCourses},
    errors::LmsError,
    logging::log_data,
    stores::courses_store::StoreCourse,
};

use super::SendToGraphql;

pub async fn get() -> Result<Vec<StoreCourse>, LmsError> {
    let graphql_variables = list_lms_courses::Variables {};
    let body = ListLmsCourses::build_query(graphql_variables);
    let response = SendToGraphql::new()
        .json(body)?
        .send::<list_lms_courses::ResponseData>()
        .await?;

    Ok(response
        .lms_courses
        .into_iter()
        .map(|api_course| {
            log_data("api course", &api_course);
            StoreCourse{
                name: api_course.title,
                id: api_course.id,
                tag: api_course.lms_tag.name.into(),
                description: api_course.short_description,
                price: api_course.price,
                long_description: api_course.long_description,
                trailer_uri: api_course.trailer_uri,
            }
        })
        .collect::<Vec<StoreCourse>>())
}

pub async fn get_by_id(id: i64) -> Result<StoreCourse, LmsError> {
    let graphql_variables = course_by_id::Variables { id };
    let body = CourseById::build_query(graphql_variables);

    let response = SendToGraphql::new()
        .json(body)?
        .send::<course_by_id::ResponseData>()
        .await?;

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
