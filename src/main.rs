mod db;
mod handlers;
mod models;
mod wrappers;

use axum::routing::*;
use handlers::*;
use models::AppState;
use std::{env, sync::Arc};
use tracing::error;

use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber log level.");

    let connection_string = env::var("DATABASE_URL")
        .map_err(|_| {
            error!("Failed to get connection string.");
            std::process::exit(1);
        })
        .unwrap();

    let app_state = Arc::new(
        AppState::connect_pool(&connection_string)
            .await
            .map_err(|_| {
                error!("Failed to connect to database pool.");
                std::process::exit(1);
            })
            .unwrap(),
    );

    let group_router: Router = Router::new()
        .route("/", post(create_group))
        .route("/join", post(add_member_to_group))
        .route("/{id}", get(get_group_with_relations))
        .route(
            "/remove/{group_id}/{member_id}",
            delete(remove_member_from_group),
        )
        .with_state(app_state.clone());

    let member_router = Router::new()
        .route("/{name}", get(get_member))
        .with_state(app_state.clone());

    let vote_router = Router::new()
        .route("/", post(create_vote))
        .with_state(app_state.clone());

    let api_routes = Router::new()
        .nest("/groups", group_router)
        .nest("/members", member_router)
        .nest("/votes", vote_router)
        .route("/health", get(async || "OK"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on adr: {}", listener.local_addr().unwrap());
    axum::serve(listener, api_routes).await.unwrap();
}
