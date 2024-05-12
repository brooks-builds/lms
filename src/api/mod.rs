pub mod articles;
pub mod auth;
pub mod tags;

use crate::{
    database_queries::{
        api_complete_user_article, api_get_all_data, api_insert_article, api_insert_course,
        api_insert_course_articles, api_insert_tag, api_insert_user_article,
        ApiCompleteUserArticle, ApiGetAllData, ApiInsertArticle, ApiInsertCourse,
        ApiInsertCourseArticles, ApiInsertTag, ApiInsertUserArticle,
    },
    errors::LmsError,
    types::{ApiAllData, Article, Course, Tag},
};
use dotenvy_macro::dotenv;
use eyre::Result;
use gloo::net::http::{Request, RequestBuilder};
use graphql_client::GraphQLQuery;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use ycl::foundations::roles::BBRole;
use yew::AttrValue;

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

#[derive(Deserialize)]
pub struct Response<T> {
    data: T,
}

pub struct SendToGraphql<T: Serialize> {
    request: RequestBuilder,
    body: Option<T>,
}

impl<T: Serialize> SendToGraphql<T> {
    pub fn new() -> Self {
        let request = Request::post(GRAPHQL_URI);
        Self {
            request,
            body: None,
        }
    }

    pub fn json(mut self, body: T) -> Result<Self, LmsError> {
        self.body = Some(body);

        Ok(self)
    }

    pub async fn send<R: DeserializeOwned>(self) -> Result<R, LmsError> {
        let request = if let Some(body) = self.body {
            self.request.json(&body).map_err(|error| {
                LmsError::SendingToGraphqlApi("sending".to_owned(), error.to_string())
            })?
        } else {
            self.request.build().map_err(|error| {
                LmsError::SendingToGraphqlApi("sending".to_owned(), error.to_string())
            })?
        };

        Ok(request
            .send()
            .await
            .map_err(|error| {
                LmsError::SendingToGraphqlApi("sending".to_owned(), error.to_string())
            })?
            .json::<Response<R>>()
            .await
            .map_err(|error| {
                LmsError::SendingToGraphqlApi(
                    "converting response to json".to_owned(),
                    error.to_string(),
                )
            })?
            .data)
    }

    pub fn authorization(mut self, token: &str) -> Self {
        let bearer_token = format!("Bearer {token}");
        self.request = self.request.header("Authorization", &bearer_token);
        self
    }

    pub fn role(mut self, role: BBRole) -> Self {
        let role_string = role.to_string();
        self.request = self.request.header("x-hasura-role", &role_string);
        self
    }
}

pub async fn get_all_data(token: Option<AttrValue>, role: BBRole) -> eyre::Result<ApiAllData> {
    let variables = api_get_all_data::Variables {};
    let query = ApiGetAllData::build_query(variables);
    let mut request = SendToGraphql::new().json(query)?.role(role);

    if let Some(token) = token {
        request = request.authorization(token.as_str());
    }

    let all_data = request.send::<api_get_all_data::ResponseData>().await?;

    let mut preview_articles_by_course: HashMap<i64, Vec<i64>> = HashMap::new();
    for course in all_data.lms_courses.iter() {
        for course_article in course.course_articles.iter() {
            if course_article.preview {
                let Some(article) = &course_article.article else {
                    continue;
                };
                let articles = preview_articles_by_course.entry(course.id).or_default();
                articles.push(article.id)
            }
        }
    }

    let db_user = all_data.users.first().map(Into::into);

    let all_data = ApiAllData {
        courses: all_data.lms_courses.into_iter().map(Into::into).collect(),
        tags: all_data.lms_tags.into_iter().map(Into::into).collect(),
        articles: all_data.lms_articles.into_iter().map(Into::into).collect(),
        preview_articles_by_course,
        db_user,
    };

    Ok(all_data)
}

pub async fn insert_tag(token: &str, name: AttrValue) -> Result<Tag> {
    let variables = api_insert_tag::Variables {
        name: name.to_string(),
    };
    let mutation = ApiInsertTag::build_query(variables);
    let result = SendToGraphql::new()
        .role(BBRole::Author)
        .authorization(token)
        .json(mutation)?
        .send::<api_insert_tag::ResponseData>()
        .await?;

    Ok(result.into())
}

pub async fn insert_course(
    token: AttrValue,
    long_description: AttrValue,
    title: AttrValue,
    tag_id: i64,
    short_description: AttrValue,
    live: bool,
) -> Result<Course> {
    let variables = api_insert_course::Variables {
        long_description: long_description.to_string(),
        title: title.to_string(),
        tag_id,
        short_description: short_description.to_string(),
        live,
    };
    let mutation = ApiInsertCourse::build_query(variables);
    let result = SendToGraphql::new()
        .role(BBRole::Author)
        .authorization(token.as_str())
        .json(mutation)?
        .send::<api_insert_course::ResponseData>()
        .await?
        .insert_lms_courses_one
        .ok_or_else(|| eyre::eyre!("Course not returned when created"))?;
    Ok(result.into())
}

pub async fn insert_article(
    token: AttrValue,
    title: AttrValue,
    content: AttrValue,
) -> Result<Article> {
    let variables = api_insert_article::Variables {
        title: title.to_string(),
        content: content.to_string(),
    };
    let mutation = ApiInsertArticle::build_query(variables);
    let result = SendToGraphql::new()
        .role(BBRole::Author)
        .authorization(token.as_str())
        .json(mutation)?
        .send::<api_insert_article::ResponseData>()
        .await?
        .insert_lms_articles_one
        .ok_or_else(|| eyre::eyre!("Missing article after inserting"))?;

    Ok(result.into())
}

pub async fn set_course_articles(
    token: AttrValue,
    course_id: i64,
    articles: &[Article],
) -> Result<()> {
    let variables = api_insert_course_articles::Variables {
        course_id,
        course_articles: articles
            .iter()
            .enumerate()
            .map(
                |(id, article)| api_insert_course_articles::lms_course_articles_insert_input {
                    article_id: Some(article.id),
                    course_id: Some(course_id),
                    order_by: Some(id as i64),
                    article: None,
                    course: None,
                    preview: Some(true),
                },
            )
            .collect(),
    };
    let mutation = ApiInsertCourseArticles::build_query(variables);
    SendToGraphql::new()
        .role(BBRole::Author)
        .authorization(token.as_str())
        .json(mutation)?
        .send::<api_insert_course_articles::ResponseData>()
        .await?;
    Ok(())
}

pub async fn insert_user_article(token: AttrValue, user_id: i64, article_id: i64) -> Result<()> {
    let variables = api_insert_user_article::Variables {
        user_id,
        article_id,
    };
    let mutation = ApiInsertUserArticle::build_query(variables);

    SendToGraphql::new()
        .authorization(token.as_str())
        .role(BBRole::Learner)
        .json(mutation)?
        .send::<api_insert_user_article::ResponseData>()
        .await?;

    Ok(())
}

pub async fn completed_user_article(token: AttrValue, user_id: i64, article_id: i64) -> Result<()> {
    let variables = api_complete_user_article::Variables {
        user_id,
        article_id,
    };
    let mutation = ApiCompleteUserArticle::build_query(variables);
    SendToGraphql::new()
        .authorization(token.as_str())
        .role(BBRole::Learner)
        .json(mutation)?
        .send::<api_complete_user_article::ResponseData>()
        .await?;
    Ok(())
}
