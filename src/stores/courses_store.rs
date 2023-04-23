use yewdux::prelude::*;

#[derive(Clone, Default, PartialEq, Eq, Store, Debug)]
pub struct CourseStore {
    pub courses: Vec<StoreCourse>,
    pub tags: Vec<StoreTag>,
}

impl CourseStore {
    pub fn get_by_course_id(&self, id: i64) -> Option<&StoreCourse> {
        self.courses.iter().find(move |course| course.id == id)
    }

    pub fn upsert_course(&mut self, id: i64, new_course: StoreCourse) {
        let mut index = None;
        for (courses_index, course) in self.courses.iter().enumerate() {
            if course.id == id {
                index = Some(courses_index);
                break;
            }
        }
        if let Some(index) = index {
            self.courses.remove(index);
            self.courses.insert(index, new_course);
        } else {
            self.courses.push(new_course);
        }
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
