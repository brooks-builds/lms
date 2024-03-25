use ycl::{
    elements::{icon::BBIconType, title::BBTitleLevel},
    foundations::container::{BBContainer, BBContainerMargin},
    modules::{
        card_list::{BBCardData, BBCardDataBuilder, BBCardDataWidth, BBCardList},
        hero::BBHero,
    },
};
use yew::prelude::*;

use crate::router::Routes;

#[function_component(Home)]
pub fn component() -> Html {
    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBHero text="Welcome to Brooks Builds Learning. Here you can dive into high-quality, education-first programming courses covering a range of topics including Rust, Docker, and more. The courses are designed to build your expertise from the ground up."
                title="Brooks Builds Learning"
            />
            <BBCardList<Routes>
                card_data={about_us_cards_data()}
                card_title_level={BBTitleLevel::Two}
                icon={BBIconType::Star}
                title="About Us"
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
            .add_text("Join our community on Discord to get help with courses, programming, and stay up to date with the latest BB news")
            .href("https://discord.gg/y7GkU6UMrm")
            .href_text("Join Discord")
            .width(BBCardDataWidth::Small)
            .build(),
    ]
}

fn about_us_cards_data() -> Vec<BBCardData<Routes>> {
    vec![
        BBCardDataBuilder::new()
            .title("What is Brooks Builds?")
            .add_text("Back when Brooks was teaching coding at a CodeSchool he had a segment called 'courage time'. This was a week where students were challenged to create something in an unfamiliar language and/or framework. The most common complaint was 'I can't build something in a language until I've learned it'. Brooks then began his Twitch stream, modeling how to learn while doing and called it Brooks Builds.")
            .add_text("While Brooks isn't teaching at that CodeSchool anymore, he is still teaching on Twitch and Youtube where you can watch him learn, create projects, as well as thes courses available here.")
            .build(),
        BBCardDataBuilder::new()
            .title("What makes us different?")
            .add_text("This LMS (Learning Management System) is focused primarily on Brooks' style of teaching. We are concerned primarily with your learning and therefore won't be 'selling' you on other courses or content while taking a course.")
            .add_text("Since Brooks created this LMS he has complete control over it and is safe from companies prioritizing selling over teaching.")
            .build()
    ]
}
