use ycl::{elements::icon::BBIconType, modules::banner::BBBannerType};
use yew::AttrValue;
use yewdux::prelude::*;

use crate::errors::LmsError;

#[derive(Clone, Default, PartialEq, Eq, Store, Debug)]
pub struct AlertsStore {
    pub message: Option<AttrValue>,
    pub icon: Option<BBIconType>,
    pub alert_type: BBBannerType,
}

#[derive(Default)]
pub struct AlertsStoreBuilder {
    pub message: Option<AttrValue>,
    pub icon: Option<BBIconType>,
    pub alert_type: BBBannerType,
}

impl AlertsStoreBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message(mut self, message: impl Into<AttrValue>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn icon(mut self, icon: BBIconType) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn alert_type(mut self, alert_type: BBBannerType) -> Self {
        self.alert_type = alert_type;
        self
    }

    pub fn build(self) -> Result<AlertsStore, LmsError> {
        let message = if let Some(message) = self.message {
            message
        } else {
            return Err(LmsError::BuildingAlertStore("missing message".into()));
        };

        let icon = if let Some(icon) = self.icon {
            icon
        } else {
            return Err(LmsError::BuildingAlertStore("missing icon".into()));
        };

        Ok(AlertsStore {
            message: Some(message),
            icon: Some(icon),
            alert_type: self.alert_type,
        })
    }

    pub fn new_error(message: impl Into<AttrValue>) -> AlertsStore {
        Self::new()
            .message(message)
            .icon(BBIconType::Warning)
            .alert_type(BBBannerType::Error)
            .build()
            .unwrap()
    }
}
