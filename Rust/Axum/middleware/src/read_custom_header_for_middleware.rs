use axum::Extension;

#[derive(Clone)]

pub struct MyHeaderMsg(pub String);

pub async fn read_custom_header(Extension(headermsg): Extension<MyHeaderMsg>) -> String {
    headermsg.0
}
