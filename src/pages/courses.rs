use std::rc::Rc;

use crate::{router::Routes, stores::main_store::MainStore};
use ycl::{
    elements::title::BBTitleLevel,
    foundations::container::{BBContainer, BBContainerMargin},
    modules::{
        card_list::{BBCardData, BBCardDataBuilder, BBCardDataWidth, BBCardList},
        hero::BBHero,
    },
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Courses)]
pub fn component() -> Html {
    let (store, _dispatch) = use_store::<MainStore>();
    let courses = create_card_data_list(store.clone(), false);
    let live_courses = create_card_data_list(store, true);

    html! {
        <BBContainer>
            <BBHero
                text="All the courses available"
                title="Course Library"
            />
            <BBContainer margin={BBContainerMargin::Normal}>
            {
                if live_courses.is_empty() {
                    html! {}
                } else {
                    html! {
                        <BBCardList<Routes>
                            card_data={live_courses}
                            card_title_level={BBTitleLevel::Three}
                            title_level={BBTitleLevel::Two}
                            title="Live Courses"
                        />
                    }
                }
            }
            <BBCardList<Routes>
                card_data={courses}
                card_title_level={BBTitleLevel::Three}
                title_level={BBTitleLevel::Two}
                title="Featured Courses"
            />
            </BBContainer>
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
                    .add_text(store_course.short_description.clone())
                    .link(Routes::CourseDetails { id: *id })
                    .width(BBCardDataWidth::Small)
                    .build(),
            )
        })
        .collect::<Vec<BBCardData<Routes>>>()
}
