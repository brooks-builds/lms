#![allow(non_camel_case_types)]
use std::{collections::HashMap, ops::Deref, rc::Rc};

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
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        articles::{Article, ArticlesStore},
        auth_store::AuthStore,
        courses_store::CourseStore,
        main_store::{self, MainStore},
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
}

#[function_component(CourseArticles)]
pub fn component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let navigator = use_navigator().unwrap();
    let assigned_article_titles = use_state(HashMap::<i64, Article>::new);

    {
        let course_id = props.course_id;
        let assigned_article_titles = assigned_article_titles.clone();
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
        let assigned_article_titles = assigned_article_titles.clone();
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
        let assigned_article_titles = assigned_article_titles.clone();
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
                .find(|course_article| course_article.id == *article_id)
                .is_some()
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
                    <BBButtonList items={create_preview_buttons(&course.articles)} onclick={Callback::from(|_| {})} />
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

fn extract_article_titles(titles: &HashMap<i64, Article>) -> Vec<BBButtonListItem> {
    titles
        .iter()
        .map(|(_id, title)| BBButtonListItem {
            label: AttrValue::from(title.title.clone()),
            id: AttrValue::from(title.id.to_string()),
        })
        .collect()
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

fn create_preview_buttons(articles: &[crate::types::Article]) -> Vec<BBButtonListItem> {
    articles
        .iter()
        .map(|article| BBButtonListItem {
            label: "Preview".into(),
            id: article.id.to_string().into(),
        })
        .collect()
}
