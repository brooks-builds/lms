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
};
use yew::{function_component, html, Html, Properties};
use yewdux::prelude::use_store;

use crate::{
    components::course_nav::CourseNav, stores::main_store::MainStore,
    utils::nav_article_onclick::article_nav_onclick,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseAccess)]
pub fn component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let course_id = props.id;

    let Some(course) = store.courses.get(&props.id) else {
        return html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{"Loading course"}</BBTitle>
        </BBContainer>
        };
    };

    let preview_articles = store
        .preview_articles_by_course
        .get(&course.id)
        .cloned()
        .unwrap_or_default();

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
                    <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{course.title.clone()}</BBTitle>
            <BBRow>
                <BBCol width={BBColWidth::Three}>
                    <CourseNav {course_id} {preview_articles} />
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
