use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};

use crate::{
    group::*,
    member::{create_member, get_member},
    setup::create_middleware,
    vote::{create_vote, get_vote},
};

pub async fn create_router() {
    let group_router: Router = Router::new()
        .route("/", post(create_group))
        .route("/{id}", get(get_group));

    let member_router = Router::new()
        .route("/", post(create_member))
        .route("/{id}", get(get_member));

    let vote_router = Router::new()
        .route("/", post(create_vote))
        .route("/{id}", get(get_vote));

    let health_router = Router::new().route("/health", get(health));

    let api_routes = Router::new()
        .nest("/groups", group_router)
        .nest("/members", member_router)
        .nest("/votes", vote_router)
        .nest("/health", health_router)
        .layer(from_fn(create_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on adr: {}", listener.local_addr().unwrap());
    axum::serve(listener, api_routes).await.unwrap();
}

pub async fn health() -> &'static str {
    "OK"
}
