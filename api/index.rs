use http::{StatusCode, uri::PathAndQuery};
use vercel_lambda::{lambda, error::VercelError, IntoResponse, Request, Response};
use std::error::Error;

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let path_and_query = request.uri().path_and_query().ok_or("Invalid URI")?;
    let path_segments = path_and_query.path().split('/').collect::<Vec<_>>();
    let query_string = path_and_query.query().unwrap_or("");

    if path_segments.len() != 2 {
        return Err("Invalid path".into());
    }

    let id = path_segments[1];
    let query_params = query_string.split('&').collect::<Vec<_>>();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(format!("id: {}, query_params: {:?}", id, query_params))
        .expect("Internal Server Error");

    Ok(response)
}