use std::collections::HashMap;

use yewdux::store::Store;

use crate::database_queries::get_lms_article_titles;

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
}
