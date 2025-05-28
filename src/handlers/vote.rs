use axum::response::IntoResponse;

pub async fn get_vote() -> impl IntoResponse {
    "get"
}

pub async fn create_vote() -> impl IntoResponse {
    "post"
}
