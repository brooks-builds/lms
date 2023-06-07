use std::rc::Rc;

use crate::{router::Routes, stores::main_store::MainStore};
use ycl::{
    elements::{
        external_link::BBLink, image::BBImage, internal_link::BBInternalLink,
        youtube_video::BBYouTubeVideo,
    },
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
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseDetails)]
pub fn component(props: &Props) -> Html {
    let (store, _dispatch) = use_store::<MainStore>();

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
                    main={hero_main(props.id, store)}
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

fn hero_main(course_id: i64, store: Rc<MainStore>) -> Html {
    html! {
        <BBContainer>
            {
                if store.logged_in() && !store.own_course(course_id) {
                    store.courses.get(&course_id).map(|course| {
                        if course.payment_uri.is_none() {
                            return html! {}
                        }

                        let payment_uri = format!("{}?client_reference_id={}", course.payment_uri.clone().unwrap(), store.db_user.clone().unwrap().id);
                        html! {
                            <BBLink
                                href={payment_uri}
                                button={true}
                                classes={classes!("mx-1")}
                            >
                                {"Purchase"}
                            </BBLink>
                        }
                    })
                } else {
                    None
                }
            }
            <BBInternalLink<Routes>
                to={Routes::CourseAccess { id: course_id }}
                button={true}
            >
                { if store.own_course(course_id) {"Open Course"} else {"Preview"}}
            </BBInternalLink<Routes>>
        </BBContainer>
    }
}
