use yewdux::prelude::*;

#[derive(Clone, Default, PartialEq, Eq, Store, Debug)]
pub struct CourseStore {
    pub courses: Vec<StoreCourse>,
}

impl CourseStore {
    pub fn get_by_course_id(&self, id: i64) -> Option<&StoreCourse> {
        self.courses.iter().find(move |course| course.id == id)
    }
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
