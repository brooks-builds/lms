use ycl::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::BBCol,
        container::{BBContainer, BBContainerMargin},
        row::BBRow,
    },
    modules::lists::button_list::{BBButtonList, BBButtonListItem},
};
pub use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};
use yewdux::prelude::use_store;

use crate::{
    api, database_queries::get_lms_article_titles, logging::log_data, stores::auth_store::AuthStore,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(CourseArticles)]
pub fn component(_props: &Props) -> Html {
    let (auth_state, _) = use_store::<AuthStore>();
    let token = auth_state.access_token.clone().unwrap_or_default();
    let article_titles = use_state(|| Vec::<ArticleTitle>::new());
    let article_titles_state =
        use_async(async move { api::articles::get_article_titles(token).await });

    {
        let article_titles_state = article_titles_state.clone();

        use_effect_once(move || {
            article_titles_state.run();

            || {}
        });
    }

    let assigned_article_titles = use_state(|| Vec<ArticleTitle>::new());

    let all_articles_onclick = {
        let assigned_article_titles = assigned_article_titles.clone();

        Callback::from(move |id: AttrValue| {
        log_data("article clicked with id", id);
    })};


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
                </BBCol>
                <BBCol>
                    <BBTitle level={BBTitleLevel::Two} align={AlignText::Center}>
                        {"All Articles"}
                    </BBTitle>
                    {
                        if article_titles_state.loading {
                            html! {
                                <BBText>{"Loading all articles"}</BBText>
                            }
                        } else {
                            if let Some(article_titles) = &article_titles_state.data {
                                html! {
                                    <BBButtonList items={extract_article_titles(article_titles)} onclick={all_articles_onclick} />
                                }
                            } else {
                                html! {}
                            }
                        }
                    }
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}

fn extract_article_titles(titles: &get_lms_article_titles::ResponseData) -> Vec<BBButtonListItem> {
    titles
        .lms_articles
        .iter()
        .map(|title| BBButtonListItem {
            label: AttrValue::from(title.title.clone()),
            id: AttrValue::from(title.id.to_string()),
        })
        .collect()
}

struct ArticleTitle {
    pub id: i64,
    pub title: String,
}
