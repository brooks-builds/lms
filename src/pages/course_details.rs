use crate::{
    api,
    logging::{log, log_error},
    router::Routes,
    stores::courses_store::{CourseStore, StoreCourse},
};
use std::rc::Rc;
use ycl::{
    elements::title::{BBTitle, BBTitleLevel},
    modules::hero::BBHero,
};
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseDetails)]
pub fn component(props: &Props) -> Html {
    let (course_store, course_store_dispatch) = use_store::<CourseStore>();
    let navigation = use_navigator().unwrap();
    let fetch_course = {
        let id = props.id;
        use_async(async move { api::get_course_by_id(id).await })
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
                    text={course.description.clone()}
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
