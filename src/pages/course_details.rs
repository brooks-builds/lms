use crate::{api, logging::log_error, stores::courses_store::CourseStore};
use ycl::{
    elements::{image::BBImage, youtube_video::BBYouTubeVideo},
    modules::hero::{BBHero, BBHeroLeftMedia},
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
        let fetch_course = fetch_course.clone();
        let fetching_course = course.is_none();
        let course_store_dispatch = course_store_dispatch.clone();
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
