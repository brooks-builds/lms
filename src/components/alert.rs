use ycl::{
    elements::icon::{BBIcon, BBIconType},
    modules::banner::{BBBanner, BBBannerType},
};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::stores::{
    alerts::AlertsStore,
    main_store::{self, MainStore},
};

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
                banner_type={BBBannerType::Error}
                icon={BBIconType::Star}
                close_onclick={onclick}
            />
        }
    } else {
        html! {}
    }
}
