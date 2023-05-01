use std::collections::HashMap;

use yewdux::store::Store;

use crate::database_queries::{
    api_get_article_titles_by_ids::ApiGetArticleTitlesByIdsLmsArticles, get_lms_article_titles,
};

#[derive(Store, Default, PartialEq, Clone)]
pub struct ArticlesStore {
    pub articles: HashMap<i64, Article>,
}

impl From<get_lms_article_titles::ResponseData> for ArticlesStore {
    fn from(value: get_lms_article_titles::ResponseData) -> Self {
        let mut articles = HashMap::new();
        for db_article in value.lms_articles {
            articles.insert(
                db_article.id,
                Article {
                    id: db_article.id,
                    created_at: db_article.created_at,
                    title: db_article.title,
                    preview: db_article.preview,
                    content: None,
                },
            );
        }

        Self { articles }
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Article {
    pub id: i64,
    pub created_at: String,
    pub title: String,
    pub preview: bool,
    pub content: Option<String>,
}

impl From<ApiGetArticleTitlesByIdsLmsArticles> for Article {
    fn from(value: ApiGetArticleTitlesByIdsLmsArticles) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            title: value.title,
            preview: value.preview,
            content: value.content.map(|c| c.content),
        }
    }
}
