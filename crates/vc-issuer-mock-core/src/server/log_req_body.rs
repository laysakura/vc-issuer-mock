use axum::{
    async_trait,
    body::{Body, Bytes},
    extract::{FromRequest, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;

/// middleware that shows how to consume the request body upfront
///
/// See: <https://github.com/tokio-rs/axum/blob/50274725cb2b865f2aaaa799ff81f44e6a587fd5/examples/consume-body-in-extractor-or-middleware/src/main.rs>
pub async fn log_req_body(request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

// the trick is to take the request apart, buffer the body, do what you need to do, then put
// the request back together
async fn buffer_request_body(request: Request) -> Result<Request, Response> {
    let (parts, body) = request.into_parts();

    // this won't work if the body is an long running stream
    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    do_thing_with_request_body(bytes.clone());

    Ok(Request::from_parts(parts, Body::from(bytes)))
}

fn do_thing_with_request_body(bytes: Bytes) {
    let bytes_str = String::from_utf8_lossy(&bytes);
    tracing::debug!("reqBody: {}", bytes_str);
}

// extractor that shows how to consume the request body upfront
struct BufferRequestBody;

// we must implement `FromRequest` (and not `FromRequestParts`) to consume the body
#[async_trait]
impl<S> FromRequest<S> for BufferRequestBody
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;

        do_thing_with_request_body(body.clone());

        Ok(Self)
    }
}
