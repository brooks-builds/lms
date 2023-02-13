use std::fmt::Debug;

use crate::errors::LmsError;
use dotenvy_macro::dotenv;
use gloo::console::{error, log};

#[allow(dead_code)]
static RUST_ENV: &str = dotenv!("RUST_ENV");

pub fn log_error(message: impl Into<String>, error: &LmsError) {
    error!(message.into(), error.to_string())
}

#[allow(dead_code)]
pub fn log_data(message: impl Into<String>, data: impl Debug) {
    log!(message.into(), format!("{data:?}"));
    if RUST_ENV != "dev" {
        log_error("Left log in code", &LmsError::LeftInLog);
    }
}

#[allow(dead_code)]
pub fn log(message: impl Into<String>) {
    log!(message.into());
    if RUST_ENV != "dev" {
        log_error("Left log in code", &LmsError::LeftInLog);
    }
}
