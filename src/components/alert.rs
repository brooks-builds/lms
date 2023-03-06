use ycl::modules::banner::BBBanner;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::stores::alerts::AlertsStore;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(Alert)]
pub fn component(_props: &Props) -> Html {
    let (alert_store, alert_dispatch) = use_store::<AlertsStore>();
    let onclick = Callback::from(move |_| {
        alert_dispatch.reduce_mut(|store| *store = AlertsStore::default());
    });

    if let Some(message) = &alert_store.message {
        html! {
            <BBBanner
                text={message.clone()}
                banner_type={alert_store.alert_type.clone()}
                icon={alert_store.icon.clone().unwrap()}
                close_onclick={onclick}
            />
        }
    } else {
        html! {}
    }
}
