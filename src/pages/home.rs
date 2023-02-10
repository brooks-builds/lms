use ycl::{
    elements::{icon::BBIconType, title::BBTitleLevel},
    foundations::container::{BBContainer, BBContainerMargin},
    modules::{
        card_list::{BBCardData, BBCardDataBuilder, BBCardList},
        hero::BBHero,
        site_footer::BBSiteFooter,
    },
};
use yew::prelude::*;

use crate::router::Routes;

#[function_component(Home)]
pub fn component() -> Html {
    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBHero text="Welcome to Brooks Builds Learning. Here you can find high-quality education-first courses on Rust"
                title="Brooks Builds Learning"
            />
            <BBCardList<Routes>
                card_data={community_cards_data()}
                card_title_level={BBTitleLevel::Two}
                icon={BBIconType::Heart}
                title="Community Driven"
            />
        </BBContainer>
    }
}

fn community_cards_data() -> Vec<BBCardData<Routes>> {
    vec![
        BBCardDataBuilder::new()
            .title("Discord")
            .text("Join our community on Discord to get help with courses, programming, and stay up to date with the latest BB news")
            .href("https://discord.gg/y7GkU6UMrm")
            .href_text("Join Discord")
            .build(),
    ]
}
