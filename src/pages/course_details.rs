use crate::{
    router::Routes,
    stores::{auth_store::AuthStore, courses_store::CourseStore, main_store::MainStore},
};
use ycl::{
    elements::{image::BBImage, internal_link::BBInternalLink, youtube_video::BBYouTubeVideo},
    foundations::container::{BBContainer, BBContainerMargin},
    modules::{
        hero::{BBHero, BBHeroLeftMedia},
        nav::{
            admin::BBAdminNav,
            navbar_link::{BBNavbarLink, BBNavbarLinkBuilder},
        },
    },
};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseDetails)]
pub fn component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();

    if let Some(course) = store.courses.get(&props.id) {
        html! {
            <div>
                {
                    if store.user.is_author() {
                        html! {
                            <BBContainer margin={BBContainerMargin::Normal}>
                                <BBAdminNav<Routes>
                                    links={create_admin_nav_routes(props.id)}
                                />
                            </BBContainer>
                        }
                    } else {
                    html! {}
                }
                }
                <BBHero
                    title={format!("${}", course.price.unwrap_or(0))}
                    subtitle={course.title.clone()}
                    text={course.long_description.clone()}
                    media={hero_left_media(course.trailer_uri.clone().map(|uri| uri.to_string()), course.title.to_string())}
                    main={hero_main(props.id)}
                />
            </div>
        }
    } else {
        html! {
            <div>
            </div>
        }
    }
}

fn hero_left_media(trailer_uri: Option<String>, title: String) -> BBHeroLeftMedia {
    let node = if let Some(uri) = trailer_uri {
        html! {
            <BBYouTubeVideo
                src={uri}
                {title}
            />
        }
    } else {
        html! {
            <BBImage alt="Code stand in for the trailer" src="/public/code.png" />
        }
    };

    BBHeroLeftMedia::LeftMedia(node)
}

fn create_admin_nav_routes(course_id: i64) -> Vec<BBNavbarLink<Routes>> {
    vec![BBNavbarLinkBuilder::<Routes>::new()
        .to(Routes::CourseArticles { id: course_id })
        .label("Course Articles")
        .build()
        .unwrap()]
}

fn hero_main(course_id: i64) -> Html {
    html! {
        <BBInternalLink<Routes>
            to={Routes::CourseAccess { id: course_id }}
            button={true}
        >
            {"Preview"}
        </BBInternalLink<Routes>>
    }
}
