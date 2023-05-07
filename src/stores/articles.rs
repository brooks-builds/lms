use std::collections::HashMap;

use yewdux::store::Store;

#[derive(Store, Default, PartialEq, Clone)]
pub struct ArticlesStore {
    pub articles: HashMap<i64, Article>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Article {
    pub id: i64,
    pub created_at: String,
    pub title: String,
    pub content: Option<String>,
}
