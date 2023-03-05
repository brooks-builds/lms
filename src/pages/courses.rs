use crate::{api, logging::log_error, router::Routes, stores::courses_store::CourseStore};
use ycl::{
    elements::title::BBTitleLevel,
    foundations::container::BBContainer,
    modules::{
        card_list::{BBCardData, BBCardDataBuilder, BBCardList},
        hero::BBHero,
    },
};
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};
use yewdux::prelude::use_store;

#[function_component(Courses)]
pub fn component() -> Html {
    let (course_store, course_store_dispatch) = use_store::<CourseStore>();

    let courses = course_store
        .courses
        .iter()
        .map(|store_course| {
            BBCardDataBuilder::<Routes>::new()
                .title(store_course.name.clone())
                .text(store_course.description.clone())
                .link(Routes::CourseDetails {
                    id: store_course.id,
                })
                .build()
        })
        .collect::<Vec<BBCardData<Routes>>>();

    let load_courses_state = use_async(async { api::courses::get().await });

    {
        let load_courses_state = load_courses_state.clone();
        use_effect_once(move || {
            load_courses_state.run();

            || {}
        });
    }

    {
        let load_courses_state = load_courses_state.clone();
        let course_store_dispatch = course_store_dispatch.clone();

        use_effect(move || {
            if let Some(courses) = &load_courses_state.data {
                course_store_dispatch
                    .reduce_mut(move |course_store| course_store.courses = courses.clone());
            }

            if let Some(error) = &load_courses_state.error {
                log_error("Error loading courses", error);
            }

            || {}
        });
    }

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
