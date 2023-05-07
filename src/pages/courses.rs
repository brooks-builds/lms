use crate::{router::Routes, stores::courses_store::CourseStore};
use ycl::{
    elements::title::BBTitleLevel,
    foundations::container::BBContainer,
    modules::{
        card_list::{BBCardData, BBCardDataBuilder, BBCardList},
        hero::BBHero,
    },
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Courses)]
pub fn component() -> Html {
    let (course_store, course_store_dispatch) = use_store::<CourseStore>();

    let courses = course_store
        .courses
        .iter()
        .map(|(id, store_course)| {
            BBCardDataBuilder::<Routes>::new()
                .title(store_course.name.clone())
                .text(store_course.description.clone())
                .link(Routes::CourseDetails { id: *id })
                .build()
        })
        .collect::<Vec<BBCardData<Routes>>>();

    html! {
        <BBContainer>
            <BBHero
                text="All the courses available"
                title="Course Library"
            />
            <BBCardList<Routes>
                card_data={courses}
                card_title_level={BBTitleLevel::Three}
                title_level={BBTitleLevel::Two}
                title="Featured Courses"
            />
        </BBContainer>
    }
}
