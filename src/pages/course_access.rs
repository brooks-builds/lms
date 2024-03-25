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
    },
    modules::card_list::{BBCardData, BBCardDataBuilder, BBCardDataWidth, BBCardList},
};
use yew::{function_component, html, Html, Properties};
use yew_router::hooks::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    components::course_nav::CourseNav,
    router::Routes,
    stores::main_store::{error_alert, MainStore},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseAccess)]
pub fn component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let course_id = props.id;
    let navigator = use_navigator().unwrap();

    let Some(course) = store.courses.get(&course_id) else {
        error_alert(dispatch, "Unable to load course");
        navigator.push(&Routes::Courses);

        return html! {<></>};
    };

    if let Some(article_id) = store.get_next_article_for_course(course_id) {
        navigator.push(&Routes::CourseAccessArticle {
            course_id,
            article_id,
        });

        html! {
            <></>
        }
    } else {
        let articles_card_data = course
            .articles
            .iter()
            .map(|article| {
                BBCardDataBuilder::new()
                    .title(article.title.clone())
                    .width(BBCardDataWidth::Medium)
                    .build()
            })
            .collect::<Vec<BBCardData<Routes>>>();

        html! {
            <BBContainer>
                <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
                    {course.title.clone()}
                </BBTitle>
                <BBText>
                    {course.long_description.clone()}
                </BBText>
                <BBCardList<Routes>
                    card_data={articles_card_data}
                    card_title_level={BBTitleLevel::Six}
                />
            </BBContainer>
        }
    }
}
