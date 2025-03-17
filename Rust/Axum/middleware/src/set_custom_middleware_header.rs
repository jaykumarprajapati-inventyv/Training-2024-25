/*In thunderclient/headers, if we're not sendig request with "message" key with its value then it'll throw "400 bad request error".*/

use axum::{body::Body, middleware::Next, response::Response};
use http::{Request, StatusCode};

use super::read_custom_header_for_middleware::MyHeaderMsg;

pub async fn my_custom_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers(); //Getting custom headers

    // Try to get the "message" header, return 400 if missing or invalid
    let message = headers
        .get("message") // Get the "message" key with its value
        .and_then(|value| value.to_str().ok()) // Convert to string, return None if invalid
        .ok_or_else(|| StatusCode::BAD_REQUEST)? // Return 400 Bad Request if missing or invalid
        .to_owned(); // Convert to owned String

    request.extensions_mut().insert(MyHeaderMsg(message));

    Ok(next.run(request).await)
}
