#[derive(PartialEq, Debug, Default)]
pub struct AuthRedirectUser {
    pub access_token: String,
    pub scope: String,
    pub expires_in: u32,
    pub token_type: String,
    pub state: String,
}

impl AuthRedirectUser {
    pub fn new(fragment: &str) -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::AuthRedirectUser;

    #[test]
    pub fn create_auth_redirect_user() {
        let fragment = "#access_token=eyJhb8u4prJkaXIiLCJlbmMiOiJBMjU2R0NNIiwiaXNzIjoiaHR0cHM6Ly9kZXYtdW55aGE0YWkudXMuYXV0aDAuY29tLyJ9..LDt3ZUq2Xgz7QHPD.cqz6B3ypnW_oD84ns9bjZufn5rR710hY2x7l3Oylrn5EjOYhtbQ2qU_qJvRBRmuiZO5AGUuG2px2rWRjO4Yo1LXarJd6NPjCTgc8QAUaypdeoWot14_ecDwCsPVzJwuYDVdr6V052bEjygRTzmVtCzrnIYk40VBbc3QWqxm0urzQ1Tn_6yLGaIAr5-HPrKask8ZeeATq-FlNg48gZRcXT6ckBIUZj20xD5fVTkhSFxKxb6rChkvJ5P7rNgFZLkQPG30HZoGxso3JcE6aBFy-xld5xQSIDA-4R4cYcQH4mLF6o3t7YVJK6YEROIE85DVDcV5OLZ2NI6N5XfkYpSx334n6C-EsRURVRgkAlB61CdPgwycd_EBOp9vBiPrrX5NvZeDXf7rxrTe-ucOJa79Zaf8FIRXdz27tOxDqgxU6lgsraoP5gIvlW2qaceFrx6y7lG5-NbQR4acYSH3EW-Ih03_p88NOW-tMTOFdOyd0CLdDtRFDdcHuJbGpVObvfaNVlVrT9832y1g.cLkIxwsNtzdPLb-Zyh05VA&scope=openid%20profile%20email&expires_in=7200&token_type=Bearer&state=SsbzpPzLJ9yNJumpVGN2HKqt";
        let redirect_user: AuthRedirectUser = AuthRedirectUser::new(fragment);
        let expected_redirect_user = AuthRedirectUser {
            access_token: "eyJhb8u4prJkaXIiLCJlbmMiOiJBMjU2R0NNIiwiaXNzIjoiaHR0cHM6Ly9kZXYtdW55aGE0YWkudXMuYXV0aDAuY29tLyJ9..LDt3ZUq2Xgz7QHPD.cqz6B3ypnW_oD84ns9bjZufn5rR710hY2x7l3Oylrn5EjOYhtbQ2qU_qJvRBRmuiZO5AGUuG2px2rWRjO4Yo1LXarJd6NPjCTgc8QAUaypdeoWot14_ecDwCsPVzJwuYDVdr6V052bEjygRTzmVtCzrnIYk40VBbc3QWqxm0urzQ1Tn_6yLGaIAr5-HPrKask8ZeeATq-FlNg48gZRcXT6ckBIUZj20xD5fVTkhSFxKxb6rChkvJ5P7rNgFZLkQPG30HZoGxso3JcE6aBFy-xld5xQSIDA-4R4cYcQH4mLF6o3t7YVJK6YEROIE85DVDcV5OLZ2NI6N5XfkYpSx334n6C-EsRURVRgkAlB61CdPgwycd_EBOp9vBiPrrX5NvZeDXf7rxrTe-ucOJa79Zaf8FIRXdz27tOxDqgxU6lgsraoP5gIvlW2qaceFrx6y7lG5-NbQR4acYSH3EW-Ih03_p88NOW-tMTOFdOyd0CLdDtRFDdcHuJbGpVObvfaNVlVrT9832y1g.cLkIxwsNtzdPLb-Zyh05VA".to_owned(),
            scope: "openid%20profile%20email".to_owned(),
            expires_in: 7200,
            token_type: "Bearer".to_owned(),
            state: "SsbzpPzLJ9yNJumpVGN2HKqt".to_owned()
        };

        assert_eq!(redirect_user, expected_redirect_user);
    }
}
