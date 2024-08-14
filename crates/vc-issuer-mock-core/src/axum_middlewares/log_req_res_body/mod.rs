use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;

/// An axum middleware to log request / response.
pub async fn log_req_res_body(req: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let path = &req.uri().path().to_string();

    // log request
    let (req_parts, req_body) = req.into_parts();
    let req_body_s = body_to_string(req_body).await?;
    tracing::debug!("path: {path}, reqBody: {req_body_s}");

    // re-construct request, and await until response
    let res = next
        .run(Request::from_parts(req_parts, Body::from(req_body_s)))
        .await;

    // log response
    let (res_parts, res_body) = res.into_parts();
    let res_body_s = body_to_string(res_body).await?;
    tracing::debug!("path: {path}, resBody: {res_body_s}");

    // re-construct response
    Ok(Response::from_parts(res_parts, Body::from(res_body_s)))
}

async fn body_to_string(body: Body) -> Result<String, Response> {
    // this won't work if the body is an long running stream
    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();
    Ok(String::from_utf8(bytes.to_vec()).unwrap())
}
