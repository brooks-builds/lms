use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LmsError {
    #[error("console.log exists")]
    LeftInLog,
    #[error("error saving cookie")]
    SavingCookie,
    #[error("error getting cookie: {0}")]
    GettingCookie(String),
    #[error("Error sending request to graphQL ({0}): {1}")]
    SendingToGraphqlApi(String, String),
    #[error("Error getting the url")]
    CannotGetURL,
}
