use hyper::{Body, Response};

pub(crate) fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("404 Not Found"));
    *response.status_mut() = hyper::StatusCode::NOT_FOUND;
    Ok(response)
}

pub(crate) fn not_implemented() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("501 Not Implemented"));
    *response.status_mut() = hyper::StatusCode::NOT_IMPLEMENTED;
    Ok(response)
}

pub(crate) fn internal_server_error(
    error: Option<hyper::http::Error>,
) -> Result<Response<Body>, hyper::Error> {
    let err_msg = match error {
        Some(e) => format!("500 Internal Server Error: {}", e),
        None => "500 Internal Server Error".to_string(),
    };

    let mut response = Response::new(Body::from(err_msg));
    *response.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
    Ok(response)
}
