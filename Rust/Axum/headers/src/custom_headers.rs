use http::HeaderMap;

/*
(1)http::HeaderMap:- It stores HTTP headers as key-value pairs.
Used to extract custom headers from the request.

(2)headers: HeaderMap:- Captures all headers from the incoming request.
Allows accessing a specific header using .get("x-message").

(3)to_owned():-Converts &str (borrowed) to String (owned).
*/

pub async fn custom_header(headers: HeaderMap) -> String {
    let message_val = headers.get("x-message").unwrap();
    let message = message_val.to_str().unwrap().to_owned();
    message
}
