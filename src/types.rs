use eyre::bail;
use ycl::foundations::roles::BBRole;
use yew::AttrValue;

use crate::database_queries::{api_get_all_courses, ApiGetAllCourses};

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
}

impl From<api_get_all_courses::ApiGetAllCoursesLmsCourses> for Course {
    fn from(api_course: api_get_all_courses::ApiGetAllCoursesLmsCourses) -> Self {
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
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub name: AttrValue,
}

impl From<api_get_all_courses::ApiGetAllCoursesLmsCoursesLmsTag> for Tag {
    fn from(value: api_get_all_courses::ApiGetAllCoursesLmsCoursesLmsTag) -> Self {
        Self {
            name: value.name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub title: AttrValue,
    pub id: i64,
    pub content: Option<AttrValue>,
}

impl From<api_get_all_courses::ApiGetAllCoursesLmsCoursesCourseArticlesArticle> for Article {
    fn from(
        api_article: api_get_all_courses::ApiGetAllCoursesLmsCoursesCourseArticlesArticle,
    ) -> Self {
        Self {
            title: api_article.title.into(),
            id: api_article.id,
            content: api_article
                .content
                .map(|api_content| api_content.content.into()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct User {
    pub role: BBRole,
    pub token: Option<AttrValue>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alert {
    pub message: AttrValue,
}
