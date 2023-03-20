use graphql_client::GraphQLQuery;

use crate::{
    database_queries::{
        course_by_id, list_lms_courses, lms_tags, CourseById, ListLmsCourses, LmsTags,
    },
    errors::LmsError,
    logging::log_data,
    stores::courses_store::{StoreCourse, StoreTag},
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
            StoreCourse {
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

pub async fn get_tags() -> Result<Vec<StoreTag>, LmsError> {
    let variables = lms_tags::Variables {};
    let body = LmsTags::build_query(variables);

    let response = SendToGraphql::new()
        .json(body)?
        .send::<lms_tags::ResponseData>()
        .await?;

    let store_tags = response
        .lms_tags
        .into_iter()
        .map(|lms_tag| StoreTag {
            id: lms_tag.id,
            name: lms_tag.name,
        })
        .collect::<Vec<StoreTag>>();

    Ok(store_tags)
}
