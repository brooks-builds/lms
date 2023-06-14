use std::rc::Rc;

use yew::Callback;
use yewdux::prelude::Dispatch;

use crate::{
    api,
    stores::main_store::{self, MainStore},
};

pub fn article_nav_onclick(store: Rc<MainStore>, dispatch: Dispatch<MainStore>) -> Callback<i64> {
    Callback::from(move |article_id: i64| {
        let Some(user) = &store.db_user else { return };
        if user.has_started_article(article_id) {
            return;
        };
        let dispatch = dispatch.clone();
        let Some(token) = store.user.token.clone() else { return };
        let user_id = user.id;

        wasm_bindgen_futures::spawn_local(async move {
            if let Err(error) = api::insert_user_article(token, user_id, article_id).await {
                gloo::console::error!("error inserting user article:", error.to_string());
            }
        });
    })
}
