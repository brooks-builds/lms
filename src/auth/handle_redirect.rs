use std::collections::HashMap;

use url::form_urlencoded;
use url::Url;

use crate::{errors::LmsError, logging::log_data};

#[derive(PartialEq, Debug, Default)]
pub struct HandleAuthRedirectUser {
    pub access_token: String,
    pub scope: String,
    pub expires_in: u32,
    pub token_type: String,
    pub state: String,
}

impl HandleAuthRedirectUser {
    pub fn new(uri: &str) -> Result<Self, LmsError> {
        let parsed_uri = Url::parse(uri)
            .map_err(|error| LmsError::HandleAuthRedirectError(error.to_string()))?;
        let fragment = parsed_uri
            .fragment()
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting fragment".into()))?;
        let url_encoded = form_urlencoded::parse(fragment.as_bytes()).collect::<HashMap<_, _>>();

        let access_token = url_encoded
            .get("access_token")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting access token".to_owned()))?
            .to_string();
        let scope = url_encoded
            .get("scope")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting scope".to_owned()))?
            .to_string();
        let expires_in = url_encoded
            .get("expires_in")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting expires in".to_owned()))?
            .parse::<u32>()
            .map_err(|error| LmsError::HandleAuthRedirectError(error.to_string()))?;
        let token_type = url_encoded
            .get("token_type")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting token type".to_owned()))?
            .to_string();
        let state = url_encoded
            .get("state")
            .ok_or_else(|| LmsError::HandleAuthRedirectError("getting state".to_owned()))?
            .to_string();

        Ok(Self {
            access_token,
            scope,
            expires_in,
            token_type,
            state,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::LmsError;

    use super::HandleAuthRedirectUser;

    #[test]
    pub fn create_auth_redirect_user() -> Result<(), LmsError> {
        let uri = "http://localhost:8082/auth/redirect#access_token=eyJhb8u4prJkaXIiLCJlbmMiOiJBMjU2R0NNIiwiaXNzIjoiaHR0cHM6Ly9kZXYtdW55aGE0YWkudXMuYXV0aDAuY29tLyJ9..LDt3ZUq2Xgz7QHPD.cqz6B3ypnW_oD84ns9bjZufn5rR710hY2x7l3Oylrn5EjOYhtbQ2qU_qJvRBRmuiZO5AGUuG2px2rWRjO4Yo1LXarJd6NPjCTgc8QAUaypdeoWot14_ecDwCsPVzJwuYDVdr6V052bEjygRTzmVtCzrnIYk40VBbc3QWqxm0urzQ1Tn_6yLGaIAr5-HPrKask8ZeeATq-FlNg48gZRcXT6ckBIUZj20xD5fVTkhSFxKxb6rChkvJ5P7rNgFZLkQPG30HZoGxso3JcE6aBFy-xld5xQSIDA-4R4cYcQH4mLF6o3t7YVJK6YEROIE85DVDcV5OLZ2NI6N5XfkYpSx334n6C-EsRURVRgkAlB61CdPgwycd_EBOp9vBiPrrX5NvZeDXf7rxrTe-ucOJa79Zaf8FIRXdz27tOxDqgxU6lgsraoP5gIvlW2qaceFrx6y7lG5-NbQR4acYSH3EW-Ih03_p88NOW-tMTOFdOyd0CLdDtRFDdcHuJbGpVObvfaNVlVrT9832y1g.cLkIxwsNtzdPLb-Zyh05VA&scope=openid%20profile%20email&expires_in=7200&token_type=Bearer&state=SsbzpPzLJ9yNJumpVGN2HKqt";
        let redirect_user: HandleAuthRedirectUser = HandleAuthRedirectUser::new(uri)?;
        let expected_redirect_user = HandleAuthRedirectUser {
            access_token: "eyJhb8u4prJkaXIiLCJlbmMiOiJBMjU2R0NNIiwiaXNzIjoiaHR0cHM6Ly9kZXYtdW55aGE0YWkudXMuYXV0aDAuY29tLyJ9..LDt3ZUq2Xgz7QHPD.cqz6B3ypnW_oD84ns9bjZufn5rR710hY2x7l3Oylrn5EjOYhtbQ2qU_qJvRBRmuiZO5AGUuG2px2rWRjO4Yo1LXarJd6NPjCTgc8QAUaypdeoWot14_ecDwCsPVzJwuYDVdr6V052bEjygRTzmVtCzrnIYk40VBbc3QWqxm0urzQ1Tn_6yLGaIAr5-HPrKask8ZeeATq-FlNg48gZRcXT6ckBIUZj20xD5fVTkhSFxKxb6rChkvJ5P7rNgFZLkQPG30HZoGxso3JcE6aBFy-xld5xQSIDA-4R4cYcQH4mLF6o3t7YVJK6YEROIE85DVDcV5OLZ2NI6N5XfkYpSx334n6C-EsRURVRgkAlB61CdPgwycd_EBOp9vBiPrrX5NvZeDXf7rxrTe-ucOJa79Zaf8FIRXdz27tOxDqgxU6lgsraoP5gIvlW2qaceFrx6y7lG5-NbQR4acYSH3EW-Ih03_p88NOW-tMTOFdOyd0CLdDtRFDdcHuJbGpVObvfaNVlVrT9832y1g.cLkIxwsNtzdPLb-Zyh05VA".to_owned(),
            scope: "openid profile email".to_owned(),
            expires_in: 7200,
            token_type: "Bearer".to_owned(),
            state: "SsbzpPzLJ9yNJumpVGN2HKqt".to_owned()
        };

        assert_eq!(redirect_user, expected_redirect_user);
        Ok(())
    }
}
