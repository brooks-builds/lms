use yewdux::prelude::*;

#[derive(Clone, Default, PartialEq, Eq, Store, Debug)]
pub struct CourseStore {
    pub courses: Vec<StoreCourse>,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct StoreCourse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub tags: Vec<CourseTag>,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub enum CourseTag {
    #[default]
    None,
    Yew,
}
