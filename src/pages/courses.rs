use std::rc::Rc;

use crate::{router::Routes, stores::main_store::MainStore};
use ycl::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::container::{BBContainer, BBContainerMargin},
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
    let courses = create_card_data_list(store.clone(), false);

    html! {
        <BBContainer>
            <BBHero
                text="All the courses available"
                title="Course Library"
            />
            <BBTitle level={BBTitleLevel::Two}>{"Live Courses"}</BBTitle>
            <BBContainer margin={BBContainerMargin::Normal}>
                <BBText>
                    {"Join Brooks and up to 10 other developers in taking a course. When purchasing one of the live courses you'll get access to the course materials, as well as customized lessons throughout the course period. You'll also get some one-on-one time with Brooks as you learn."}
                </BBText>
            </BBContainer>
            <BBCardList<Routes>
                card_data={courses}
                card_title_level={BBTitleLevel::Three}
                title_level={BBTitleLevel::Two}
                title="Featured Courses"
            />
        </BBContainer>
    }
}

fn create_card_data_list(store: Rc<MainStore>, live: bool) -> Vec<BBCardData<Routes>> {
    store
        .courses
        .iter()
        .filter_map(move |(id, store_course)| {
            if store_course.live != live {
                return None;
            }

            Some(
                BBCardDataBuilder::<Routes>::new()
                    .title(store_course.title.as_str())
                    .text(store_course.short_description.as_str())
                    .link(Routes::CourseDetails { id: *id })
                    .build(),
            )
        })
        .collect::<Vec<BBCardData<Routes>>>()
}
