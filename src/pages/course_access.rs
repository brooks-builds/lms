use ycl::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::{BBCol, BBColWidth},
        container::{BBContainer, BBContainerMargin},
        row::BBRow,
        states::BBLoadingState,
    },
};
use yew::{function_component, html, use_effect, use_state, Html, Properties};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    components::course_nav::CourseNav,
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        articles::ArticlesStore,
        courses_store::CourseStore,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseAccess)]
pub fn component(props: &Props) -> Html {
    let course_loading_state = use_state(|| BBLoadingState::Initialized);
    let (courses_store, courses_dispatch) = use_store::<CourseStore>();
    let course_id = props.id;
    let (_, alert_dispatch) = use_store::<AlertsStore>();
    let article_titles_loading_state = use_state(|| BBLoadingState::Initialized);
    let navigator = use_navigator().unwrap();
    let (_articles_store, articles_dispatch) = use_store::<ArticlesStore>();

    {
        let alert_dispatch = alert_dispatch;
        let courses_store = courses_store.clone();

        use_effect(move || {
            let result = || {};
            let alert_dispatch = alert_dispatch.clone();
            let courses_dispatch = courses_dispatch.clone();
            let course_loading_state = course_loading_state.clone();
            let courses_store = courses_store.clone();

            if *course_loading_state == BBLoadingState::Initialized {
                let course_loading_state = course_loading_state.clone();
                let alert_dispatch = alert_dispatch.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    course_loading_state.clone().set(BBLoadingState::Loading);
                });
            }

            if *article_titles_loading_state == BBLoadingState::Initialized
                && *course_loading_state == BBLoadingState::Loaded
            {
                let article_titles_loading_state = article_titles_loading_state.clone();
                let courses_store = courses_store;
                let Some(_course) = courses_store.courses.get(&course_id) else{
                    alert_dispatch.reduce_mut(|alert_state| *alert_state = AlertsStoreBuilder::new_error("Could not find course"));
                    navigator.push(&Routes::Courses);
                    return result;
                };
                let articles_dispatch = articles_dispatch.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    article_titles_loading_state
                        .clone()
                        .set(BBLoadingState::Loading);
                });
            }

            result
        });
    }

    let Some(course) = courses_store.courses.get(&props.id) else {
        return html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{"Loading course"}</BBTitle>
        </BBContainer>
        };

    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
                    <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{&course.name}</BBTitle>
            <BBRow>
                <BBCol width={BBColWidth::Three}>
                    <CourseNav {course_id} />
                </BBCol>
                <BBCol>
                    <BBTitle
                        align={AlignText::Center}
                        level={BBTitleLevel::Two}
                    >
                        {"Select an Article"}
                    </BBTitle>
                    <BBText>
                        {"Click an article to the left to load it"}
                    </BBText>
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}
