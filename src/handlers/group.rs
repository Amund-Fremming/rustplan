use axum::response::IntoResponse;

pub async fn get_group() -> impl IntoResponse {
    "get"
}

pub async fn create_group() -> impl IntoResponse {
    "post"
}
