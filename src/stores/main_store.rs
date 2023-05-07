use std::collections::HashMap;

use ycl::foundations::states::BBLoadingState;
use yewdux::prelude::*;

use crate::{
    api,
    types::{Alert, Course, User},
};

#[derive(Store, Default, Clone, PartialEq)]
pub struct MainStore {
    pub courses: HashMap<i64, Course>,
    pub courses_loaded: BBLoadingState,
    pub user: User,
    pub alert: Alert,
}

pub async fn load_all_courses(dispatch: Dispatch<MainStore>) {
    dispatch
        .reduce_mut_future(|store| {
            Box::pin(async move {
                store.courses_loaded = BBLoadingState::Loading;

                match api::courses::get_all_courses(store.user.token.clone(), store.user.role).await
                {
                    Ok(courses) => {
                        for course in courses {
                            store.courses.insert(course.id, course);
                            store.courses_loaded = BBLoadingState::Loaded
                        }
                    }
                    Err(error) => {
                        gloo::console::error!("Error getting courses:", error.to_string());
                        store.alert.message = "There was an error getting courses".into();
                    }
                }
            })
        })
        .await;
}
