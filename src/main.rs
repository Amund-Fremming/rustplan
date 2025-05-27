mod group;
mod member;
mod setup;
mod vote;

use setup::*;

#[tokio::main]
async fn main() {
    create_router().await;
}
