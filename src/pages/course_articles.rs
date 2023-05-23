#![allow(non_camel_case_types)]

use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle},
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::BBCol,
        container::{BBContainer, BBContainerMargin},
        row::BBRow,
        states::BBLoadingState,
    },
    modules::lists::button_list::{BBButtonList, BBButtonListItem},
};
pub use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::main_store::{self, MainStore},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
}

#[function_component(CourseArticles)]
pub fn component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let navigator = use_navigator().unwrap();

    {
        let dispatch = dispatch.clone();
        let store = store.clone();

        use_effect(move || {
            let result = || {};

            if store.courses_loaded != BBLoadingState::Loaded {
                return result;
            }

            if !store.user.is_author() {
                main_store::error_alert(
                    dispatch.clone(),
                    "Only Authors can manage course articles",
                );
                navigator.push(&Routes::Home);
                return result;
            }

            result
        });
    }

    let all_articles_onclick = {
        let store = store.clone();
        let dispatch = dispatch.clone();
        let course_id = props.course_id;

        Callback::from(move |id: AttrValue| {
            let id = id.to_string().parse::<i64>().unwrap();
            let Some(article) = store.articles.get(&id) else {
                main_store::error_alert(dispatch.clone(), "Article not found");
                return;
            };
            main_store::add_article_to_course(dispatch.clone(), article.to_owned(), course_id);
        })
    };

    let assigned_articles_onclick = {
        let dispatch = dispatch.clone();
        let course_id = props.course_id;

        Callback::from(move |id: AttrValue| {
            let id = id
                .to_string()
                .parse::<i64>()
                .expect("assigned article id is not a number");
            main_store::remove_article_from_course(dispatch.clone(), id, course_id);
        })
    };

    let save_onclick = {
        let course_id = props.course_id;

        Callback::from(move |_| {
            let dispatch = dispatch.clone();

            wasm_bindgen_futures::spawn_local(async move {
                main_store::save_course_articles(dispatch.clone(), course_id).await;
            });
        })
    };

    let Some(course)= &store.courses.get(&props.course_id) else {
        return html!{};
    };

    let available_articles = store
        .articles
        .iter()
        .filter_map(|(article_id, article)| {
            if course
                .articles
                .iter()
                .any(|course_article| course_article.id == *article_id)
            {
                None
            } else {
                Some(article.clone())
            }
        })
        .collect::<Vec<crate::types::Article>>();

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
                {"Course Articles"}
            </BBTitle>
            <BBRow>
                <BBCol>
                    <BBTitle level={BBTitleLevel::Two} align={AlignText::Center}>
                        {"Assigned"}
                    </BBTitle>
                    <BBButtonList items={create_article_titles(&course.articles)} onclick={assigned_articles_onclick} />
                </BBCol>
                <BBCol>
                    <BBTitle level={BBTitleLevel::Two} align={AlignText::Center}>
                        {"All Articles"}
                    </BBTitle>
                    <BBButtonList items={create_article_titles(&available_articles)} onclick={all_articles_onclick} />
                </BBCol>
            </BBRow>
            <BBRow>
                <BBButton button_style={BBButtonStyle::PrimaryLight} onclick={save_onclick}>{"Save"}</BBButton>
            </BBRow>
        </BBContainer>
    }
}

fn create_article_titles(articles: &[crate::types::Article]) -> Vec<BBButtonListItem> {
    articles
        .iter()
        .map(|article| BBButtonListItem {
            label: article.title.clone(),
            id: article.id.to_string().into(),
        })
        .collect()
}
