use graphql_client::GraphQLQuery;
use ycl::foundations::roles::BBRole;

use crate::{
    database_queries::{
        course_by_id, create_lms_course, list_lms_courses, lms_tags, set_lms_course_articles,
        CourseById, CreateLmsCourse, ListLmsCourses, LmsTags, SetLmsCourseArticles,
    },
    errors::LmsError,
    logging::log_data,
    stores::{
        articles::Article,
        courses_store::{StoreCourse, StoreTag},
    },
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
                article_ids: api_course.article_ids,
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
            article_ids: response_course.article_ids,
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

pub async fn insert_course(
    long_description: String,
    short_description: String,
    tag_id: i64,
    title: String,
    token: &str,
) -> Result<create_lms_course::ResponseData, LmsError> {
    let variables = create_lms_course::Variables {
        long_description,
        short_description,
        tag_id,
        title,
    };
    let query = CreateLmsCourse::build_query(variables);
    SendToGraphql::new()
        .authorization(token)
        .role(ycl::foundations::roles::BBRole::Author)
        .json(query)?
        .send::<create_lms_course::ResponseData>()
        .await
}

pub async fn set_course_articles(
    course_id: i64,
    articles: &[Article],
    token: String,
) -> Result<set_lms_course_articles::ResponseData, LmsError> {
    let variables = set_lms_course_articles::Variables {
        id: course_id,
        article_ids: articles
            .iter()
            .map(|article| article.id)
            .collect::<Vec<i64>>(),
    };
    let query = SetLmsCourseArticles::build_query(variables);
    SendToGraphql::new()
        .authorization(&token)
        .role(BBRole::Author)
        .json(query)?
        .send::<set_lms_course_articles::ResponseData>()
        .await
}
