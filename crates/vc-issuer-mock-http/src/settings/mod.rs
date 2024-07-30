use axum::http::Uri;
use vc_issuer_mock_core::VC_DEFAULT_ISSUER_ID;

const DEFAULT_PORT: u16 = 50080;

/// Defines the settings for the VC Issuer Mock HTTP server at launch time.
#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Settings {
    pub(crate) port: u16,
    pub(crate) issuer_id: Uri,
    pub(crate) oauth2_server: Uri,
}

impl Settings {
    /// Resolves the settings from the environment variables.
    ///
    /// See the [crate] document for the list of required environment variables.
    ///
    /// # Panics
    ///
    /// If any of the required environment variables is not set, or in invalid format
    pub(crate) fn new_from_env() -> Self {
        let port = std::env::var("ISSMOCK_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
        let issuer_id =
            std::env::var("ISSMOCK_ISSUER_ID").unwrap_or_else(|_| VC_DEFAULT_ISSUER_ID.to_string());
        let oauth2_server =
            std::env::var("ISSMOCK_OAUTH2_SERVER").expect("`ISSMOCK_OAUTH2_SERVER` env is not set");

        Self {
            port: port
                .parse()
                .expect(&format!("Invalid env ISSMOCK_PORT (`{port}`)")),
            issuer_id: issuer_id
                .parse()
                .expect(&format!("Invalid env ISSMOCK_ISSUER_ID (`{issuer_id}`)")),
            oauth2_server: oauth2_server.parse().expect(&format!(
                "Invalid env ISSMOCK_OAUTH2_SERVER (`{oauth2_server}`)"
            )),
        }
    }
}
