use ycl::modules::banner::BBBanner;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::stores::main_store::{self, MainStore};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(Alert)]
pub fn component(_props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let onclick = Callback::from(move |_| {
        main_store::reset_alert(dispatch.clone());
    });

    if let Some(message) = store.alert.message.clone() {
        html! {
            <BBBanner
                text={message}
                banner_type={store.alert.alert_type}
                icon={store.alert.icon}
                close_onclick={onclick}
            />
        }
    } else {
        html! {}
    }
}
