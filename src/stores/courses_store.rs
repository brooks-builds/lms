use std::collections::HashMap;

use yewdux::prelude::*;

#[derive(Clone, Default, PartialEq, Eq, Store, Debug)]
pub struct CourseStore {
    pub courses: HashMap<i64, StoreCourse>,
    pub tags: Vec<StoreTag>,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct StoreCourse {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub tag: CourseTag,
    pub price: Option<u8>,
    pub long_description: String,
    pub trailer_uri: Option<String>,
    pub article_ids: Vec<i64>,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub enum CourseTag {
    #[default]
    None,
    Yew,
}

impl From<String> for CourseTag {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Yew" => Self::Yew,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct StoreTag {
    pub id: i64,
    pub name: String,
}
