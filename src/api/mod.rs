pub mod articles;
pub mod auth;
pub mod tags;

use dotenvy_macro::dotenv;
use eyre::Result;
use gloo::net::http::Request;
use graphql_client::GraphQLQuery;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ycl::foundations::roles::BBRole;
use yew::AttrValue;

use crate::{
    database_queries::{
        api_get_all_data, api_insert_article, api_insert_course, api_insert_tag, ApiGetAllData,
        ApiInsertArticle, ApiInsertCourse, ApiInsertTag,
    },
    errors::LmsError,
    types::{ApiAllData, Article, Course, Tag},
};

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

#[derive(Deserialize)]
pub struct Response<T> {
    data: T,
}

pub struct SendToGraphql {
    request: Request,
}

impl SendToGraphql {
    pub fn new() -> Self {
        let request = Request::post(GRAPHQL_URI);
        Self { request }
    }

    pub fn json(mut self, body: impl Serialize) -> Result<Self, LmsError> {
        self.request = self.request.json(&body).map_err(|error| {
            LmsError::SendingToGraphqlApi("adding json".to_owned(), error.to_string())
        })?;

        Ok(self)
    }

    pub async fn send<R: DeserializeOwned>(self) -> Result<R, LmsError> {
        Ok(self
            .request
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

    let all_data = ApiAllData {
        courses: all_data.lms_courses.into_iter().map(Into::into).collect(),
        tags: all_data.lms_tags.into_iter().map(Into::into).collect(),
        articles: all_data.lms_articles.into_iter().map(Into::into).collect(),
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
) -> Result<Course> {
    let variables = api_insert_course::Variables {
        long_description: long_description.to_string(),
        title: title.to_string(),
        tag_id,
        short_description: short_description.to_string(),
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
