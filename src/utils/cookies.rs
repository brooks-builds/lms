use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

use crate::{
    errors::LmsError,
    logging::{log_data, log_error},
};

pub fn save_cookie(key: &str, value: &str, max_age: u32) -> Result<(), LmsError> {
    let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
    let cookie = format!("{key}={value}; max-age={max_age}; samesite=strict; secure");
    document.set_cookie(&cookie).map_err(|_error| {
        let error = LmsError::SavingCookie;
        log_error("Error storing state into cookie", &error);
        error
    })?;
    Ok(())
}

pub fn load_cookie(key: &str) -> Result<Option<String>, LmsError> {
    let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
    let all_cookies = document.cookie().map_err(|error| {
        let error = if let Some(error) = error.as_string() {
            error
        } else {
            String::new()
        };

        LmsError::GettingCookie(error)
    })?;
    for cookie in all_cookies.split("; ") {
        let trim_by = format!("{key}=");
        if let Some(value) = cookie.strip_prefix(trim_by.as_str()) {
            return Ok(Some(value.to_owned()));
        }
    }
    Ok(None)
}
