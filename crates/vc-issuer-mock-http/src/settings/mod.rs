use axum::http::Uri;

const DEFAULT_PORT: u16 = 50080;

/// Defines the settings for the VC Issuer Mock HTTP server at launch time.
#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Settings {
    pub(crate) port: u16,
    issuer_id: Uri,
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
        let port: u16 = port
            .parse()
            .expect(&format!("Invalid env ISSMOCK_PORT (`{port}`)"));

        let issuer_id = std::env::var("ISSMOCK_ISSUER_ID")
            .unwrap_or_else(|_| Self::issuer_id(port).to_string());

        let oauth2_server =
            std::env::var("ISSMOCK_OAUTH2_SERVER").expect("`ISSMOCK_OAUTH2_SERVER` env is not set");

        Self {
            port,
            issuer_id: issuer_id
                .parse()
                .expect(&format!("Invalid env ISSMOCK_ISSUER_ID (`{issuer_id}`)")),
            oauth2_server: oauth2_server.parse().expect(&format!(
                "Invalid env ISSMOCK_OAUTH2_SERVER (`{oauth2_server}`)"
            )),
        }
    }

    pub(crate) fn to_issuer_root_url(&self) -> Uri {
        Self::issuer_id(self.port)
    }

    pub(crate) fn to_issuer_oid4vci_base_url(&self) -> Uri {
        format!("{}oid4vci", self.to_issuer_root_url())
            .parse()
            .expect("Failed to parse the OID4VCI base URL")
    }

    fn issuer_id(port: u16) -> Uri {
        format!("http://localhost:{}/", port)
            .parse()
            .expect("Failed to parse the issuer_id")
    }
}
