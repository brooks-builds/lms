use crate::{
    router::Routes,
    stores::{auth_store::AuthStore, courses_store::CourseStore},
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
    let (course_store, course_store_dispatch) = use_store::<CourseStore>();
    let (auth_store, _) = use_store::<AuthStore>();

    let fetch_course = {
        let id = props.id;
    };

    let course = course_store.courses.get(&props.id);

    {
        let have_course = course.is_some();
        let fetch_course = fetch_course.clone();
        use_effect_once(move || {
            if !have_course {}

            || {}
        });
    }

    {
        let fetching_course = course.is_none();
        use_effect(move || {
            let return_closure = || {};

            if !fetching_course {
                return return_closure;
            }

            return_closure
        });
    };

    if let Some(course) = course {
        html! {
            <div>
                {
                    if auth_store.is_author() {
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
                    subtitle={course.name.clone()}
                    text={course.long_description.clone()}
                    media={hero_left_media(course.trailer_uri.clone(), course.name.clone())}
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
