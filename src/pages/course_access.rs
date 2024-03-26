use ycl::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::{BBCol, BBColWidth},
        container::BBContainer,
        row::BBRow,
    },
    modules::card_list::{BBCardData, BBCardDataBuilder, BBCardDataWidth, BBCardList},
};
use yew::{function_component, html, Html, Properties};
use yew_router::hooks::use_navigator;
use yewdux::prelude::use_store;

use crate::{components::course_nav::CourseNav, router::Routes, stores::main_store::MainStore};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseAccess)]
pub fn component(props: &Props) -> Html {
    let (store, _dispatch) = use_store::<MainStore>();
    let course_id = props.id;
    let navigator = use_navigator().unwrap();
    let Some(course) = store.courses.get(&course_id) else {
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
                let mut card_builder = BBCardDataBuilder::new()
                    .title(article.title.clone())
                    .width(BBCardDataWidth::Small);

                if let Some(description) = &article.description {
                    card_builder = card_builder.add_text(description.clone());
                }

                card_builder.build()
            })
            .collect::<Vec<BBCardData<Routes>>>();

        let preview_articles = store
            .preview_articles_by_course
            .get(&course.id)
            .cloned()
            .unwrap_or_default();

        html! {
            <BBContainer>
                <BBRow>
                    <BBCol>
                        <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
                            {course.title.clone()}
                        </BBTitle>
                    </BBCol>
                </BBRow>
                <BBRow>
                    <BBCol width={BBColWidth::Three}>
                        <CourseNav course_id={course_id} {preview_articles} />
                    </BBCol>
                    <BBCol width={BBColWidth::Nine}>
                        <BBText>
                            {course.long_description.clone()}
                        </BBText>
                        <BBCardList<Routes>
                            card_data={articles_card_data}
                            card_title_level={BBTitleLevel::Three}
                            wrap={true}
                            title_level={BBTitleLevel::Two}
                            title="Course Articles"
                        />
                    </BBCol>
                </BBRow>
            </BBContainer>
        }
    }
}
