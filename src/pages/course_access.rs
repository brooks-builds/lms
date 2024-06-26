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
use yew::{function_component, html, Callback, Html, Properties};
use yew_router::hooks::use_navigator;
use yewdux::prelude::use_store;

use crate::{router::Routes, stores::main_store::MainStore};

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

                if article.preview {
                    let article_id = article.id;
                    let navigator = navigator.clone();

                    card_builder = card_builder
                        .card_type(ycl::modules::card::BBCardType::CallToAction)
                        .href_text("Preview the article")
                        .onclick(Callback::from(move |_| {
                            navigator.push(&Routes::CourseAccessArticle {
                                course_id,
                                article_id,
                            });
                        }));
                }

                card_builder.build()
            })
            .collect::<Vec<BBCardData<Routes>>>();

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
                    <BBCol width={BBColWidth::Twelve}>
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
