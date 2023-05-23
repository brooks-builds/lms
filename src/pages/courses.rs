use crate::{router::Routes, stores::main_store::MainStore};
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
    let (store, _dispatch) = use_store::<MainStore>();
    let courses = store
        .courses
        .iter()
        .map(|(id, store_course)| {
            BBCardDataBuilder::<Routes>::new()
                .title(store_course.title.as_str())
                .text(store_course.short_description.as_str())
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
