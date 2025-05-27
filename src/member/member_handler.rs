use axum::response::IntoResponse;

pub async fn get_member() -> impl IntoResponse {
    "get"
}

pub async fn create_member() -> impl IntoResponse {
    "post"
}
