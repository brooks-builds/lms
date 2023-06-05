use std::collections::HashMap;

use crate::database_queries::{
    api_get_all_data, api_insert_article, api_insert_course, api_insert_tag,
};
use serde::{Deserialize, Serialize};
use ycl::{elements::icon::BBIconType, foundations::roles::BBRole, modules::banner::BBBannerType};
use yew::AttrValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Course {
    pub id: i64,
    pub tag: Tag,
    pub long_description: AttrValue,
    pub price: Option<u8>,
    pub short_description: AttrValue,
    pub title: AttrValue,
    pub trailer_uri: Option<AttrValue>,
    pub articles: Vec<Article>,
    pub articles_dirty: bool,
    pub live: bool,
    pub launches_on: Option<AttrValue>,
    pub payment_uri: Option<AttrValue>,
}

impl From<api_get_all_data::ApiGetAllDataLmsCourses> for Course {
    fn from(api_course: api_get_all_data::ApiGetAllDataLmsCourses) -> Self {
        Self {
            id: api_course.id,
            tag: api_course.lms_tag.into(),
            long_description: AttrValue::from(api_course.long_description),
            price: api_course.price,
            short_description: AttrValue::from(api_course.short_description),
            title: AttrValue::from(api_course.title),
            trailer_uri: api_course.trailer_uri.map(AttrValue::from),
            articles: api_course
                .course_articles
                .into_iter()
                .filter_map(|api_course_articles| api_course_articles.article.map(Into::into))
                .collect(),
            articles_dirty: false,
            live: api_course.live,
            launches_on: api_course.launches_on.map(Into::into),
            payment_uri: api_course.payment_uri.map(Into::into),
        }
    }
}

impl From<api_insert_course::ApiInsertCourseInsertLmsCoursesOne> for Course {
    fn from(value: api_insert_course::ApiInsertCourseInsertLmsCoursesOne) -> Self {
        Self {
            id: value.id,
            tag: Tag {
                id: value.lms_tag.id,
                name: value.lms_tag.name.into(),
            },
            long_description: value.long_description.into(),
            price: None,
            short_description: value.short_description.into(),
            title: value.title.into(),
            trailer_uri: None,
            articles: vec![],
            articles_dirty: false,
            live: value.live,
            launches_on: value.launches_on.map(Into::into),
            payment_uri: value.payment_uri.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub id: i64,
    pub name: AttrValue,
}

impl From<api_get_all_data::ApiGetAllDataLmsCoursesLmsTag> for Tag {
    fn from(value: api_get_all_data::ApiGetAllDataLmsCoursesLmsTag) -> Self {
        Self {
            name: value.name.into(),
            id: value.id,
        }
    }
}

impl From<api_get_all_data::ApiGetAllDataLmsTags> for Tag {
    fn from(lms_tag: api_get_all_data::ApiGetAllDataLmsTags) -> Self {
        Self {
            id: lms_tag.id,
            name: lms_tag.name.into(),
        }
    }
}

impl From<api_insert_tag::ResponseData> for Tag {
    fn from(value: api_insert_tag::ResponseData) -> Self {
        let api_insert_tag::ApiInsertTagInsertLmsTagsOne { id, name } = value
            .insert_lms_tags_one
            .expect("id and name missing from tag response data");
        Self {
            id,
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub title: AttrValue,
    pub id: i64,
    pub content: Option<AttrValue>,
    pub preview: Option<bool>,
}

impl From<api_get_all_data::ApiGetAllDataLmsCoursesCourseArticlesArticle> for Article {
    fn from(api_article: api_get_all_data::ApiGetAllDataLmsCoursesCourseArticlesArticle) -> Self {
        Self {
            title: api_article.title.into(),
            id: api_article.id,
            content: api_article
                .content
                .map(|api_content| api_content.content.into()),
            preview: None,
        }
    }
}

impl From<api_insert_article::ApiInsertArticleInsertLmsArticlesOne> for Article {
    fn from(api_article: api_insert_article::ApiInsertArticleInsertLmsArticlesOne) -> Self {
        Self {
            title: api_article.title.into(),
            id: api_article.id,
            content: api_article
                .content
                .map(|api_content| api_content.content.into()),
            preview: None,
        }
    }
}

impl From<api_get_all_data::ApiGetAllDataLmsArticles> for Article {
    fn from(value: api_get_all_data::ApiGetAllDataLmsArticles) -> Self {
        Self {
            title: value.title.into(),
            id: value.id,
            content: None,
            preview: None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct User {
    pub role: BBRole,
    pub token: Option<AttrValue>,
    pub id: Option<AttrValue>,
    pub nickname: Option<AttrValue>,
    pub name: Option<AttrValue>,
    pub picture: Option<AttrValue>,
    pub email: Option<AttrValue>,
    pub email_verified: Option<bool>,
}

impl User {
    pub fn is_author(&self) -> bool {
        self.role == BBRole::Author
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth0User {
    pub sub: String,
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: String,
    pub email: String,
    pub email_verified: bool,
    #[serde(rename = "https://brooksbuilds.com")]
    pub metadata: Auth0UserMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth0UserMetadata {
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alert {
    pub message: Option<AttrValue>,
    pub alert_type: BBBannerType,
    pub icon: BBIconType,
}

impl Alert {
    pub fn error(&mut self, message: impl Into<AttrValue>) {
        self.message = Some(message.into());
        self.alert_type = BBBannerType::Error;
        self.icon = BBIconType::Warning;
    }

    pub fn success(&mut self, message: impl Into<AttrValue>) {
        self.message = Some(message.into());
        self.alert_type = BBBannerType::Success;
        self.icon = BBIconType::Star;
    }
}

pub struct ApiAllData {
    pub courses: Vec<Course>,
    pub tags: Vec<Tag>,
    pub articles: Vec<Article>,
    pub preview_articles_by_course: HashMap<i64, Vec<i64>>,
}
