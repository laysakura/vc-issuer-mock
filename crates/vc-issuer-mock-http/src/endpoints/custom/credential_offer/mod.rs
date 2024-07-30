use axum::{response::Html, Extension};
use base64::{engine::general_purpose, Engine as _};
use image::{ImageFormat, Luma};
use qrcode::QrCode;
use std::io::Cursor;
use tera::{Context, Tera};
use vc_issuer_mock_core::endpoints::oid4vci::CredentialOffer;

/// An HTML page to show the Credential Offer Endpoint URLs (in a QR code and a hyperlink).
#[axum::debug_handler]
pub(crate) async fn show(
    Extension(credential_offer): Extension<CredentialOffer>,
    Extension(templates): Extension<Tera>,
) -> Html<String> {
    let url = credential_offer.to_url_by_value();
    let qr_base64 = to_qr_base64(&url);

    let mut context = Context::new();
    context.insert("url", &url);
    context.insert("qr_code", &qr_base64);

    let rendered = templates.render("index.html", &context).unwrap();
    Html(rendered)
}

fn to_qr_base64(url: &str) -> String {
    let qr = QrCode::new(url).unwrap();
    let qr_image = qr.render::<Luma<u8>>().build();
    let mut qr_buffer = Cursor::new(Vec::new());
    qr_image.write_to(&mut qr_buffer, ImageFormat::Png).unwrap();
    general_purpose::STANDARD.encode(qr_buffer.get_ref())
}

#[cfg(test)]
mod tests {

    use std::io::Write;
    use std::str::FromStr;

    use axum::http::Uri;
    use tempfile::Builder;

    use crate::{templates::init_templates, test_tracing::init_tracing};

    use super::*;

    #[tokio::test]
    async fn test_show() {
        init_tracing();

        let templates = init_templates();
        let credential_offer = CredentialOffer::new(&Uri::from_str("https://example.com").unwrap());

        let res = show(Extension(credential_offer), Extension(templates)).await;
        let html = res.0;

        // Save HTML to tmp file
        let mut tmp_file = Builder::new()
            .prefix("index-")
            .suffix(".html")
            .rand_bytes(5)
            .tempfile()
            .unwrap();
        write!(tmp_file, "{}", html).unwrap();
        let tmp_path = tmp_file.into_temp_path();
        let path = tmp_path.keep().unwrap();

        // Log the HTML file path
        tracing::debug!("credential-offer page saved to file://{}", path.display());

        assert!(html.contains("openid-credential-offer://"));
        assert!(html.contains("data:image/png;base64,"));
    }
}
