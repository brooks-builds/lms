use crate::{
    api,
    logging::log_error,
    router::Routes,
    stores::{auth_store::AuthStore, courses_store::CourseStore},
};
use ycl::{
    elements::{image::BBImage, youtube_video::BBYouTubeVideo},
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
use yew_hooks::{use_async, use_effect_once};
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
        use_async(async move { api::courses::get_by_id(id).await })
    };

    let course = course_store.get_by_course_id(props.id);

    {
        let have_course = course.is_some();
        let fetch_course = fetch_course.clone();
        use_effect_once(move || {
            if !have_course {
                fetch_course.run();
            }

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

            if fetch_course.loading {
                return return_closure;
            }

            if let Some(error) = &fetch_course.error {
                log_error("error fetching one course", error);
            }

            if let Some(course) = fetch_course.data.clone() {
                course_store_dispatch.reduce_mut(|course_store| {
                    course_store.courses.push(course);
                });
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
                                    links={create_admin_nav_routes()}
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

fn create_admin_nav_routes() -> Vec<BBNavbarLink<Routes>> {
    vec![BBNavbarLinkBuilder::<Routes>::new()
        .to(Routes::CourseArticles)
        .label("Course Articles")
        .build()
        .unwrap()]
}
