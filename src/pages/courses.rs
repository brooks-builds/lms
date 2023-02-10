use crate::{
    api,
    router::Routes,
    stores::courses_store::{self, CourseStore},
};
use ycl::{
    elements::title::BBTitleLevel,
    foundations::container::BBContainer,
    modules::{
        card_list::{BBCardDataBuilder, BBCardList},
        hero::BBHero,
    },
};
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};
use yewdux::prelude::use_store;

#[function_component(Courses)]
pub fn component() -> Html {
    let courses = vec![BBCardDataBuilder::new()
        .title("Introduction to Yew")
        .text("Learn how to build web applications using Yew.rs and Rust.")
        .build()];

    let load_courses_state = use_async(async { api::get_courses().await });
    let (course_store, course_store_dispatch) = use_store::<CourseStore>();

    {
        let load_courses_state = load_courses_state.clone();
        use_effect_once(move || {
            gloo::console::log!("running course state");
            load_courses_state.run();

            || {}
        });
    }

    {
        let load_courses_state = load_courses_state.clone();
        let course_store_dispatch = course_store_dispatch.clone();
        let course_store = course_store.clone();

        use_effect(move || {
            gloo::console::log!("load courses state", load_courses_state.loading);
            if let Some(courses) = &load_courses_state.data {
                course_store_dispatch
                    .reduce_mut(move |course_store| course_store.courses = courses.clone());
            } else {
                gloo::console::log!("no courses yet");
            }

            gloo::console::log!("course_store", format!("{:?}", &course_store));

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
