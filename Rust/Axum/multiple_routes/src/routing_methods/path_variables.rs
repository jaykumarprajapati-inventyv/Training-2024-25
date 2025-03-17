use axum::extract::Path;

pub async fn get_id(Path(id): Path<i32>) -> String {
    format!("We' got id {} request", id)
}
