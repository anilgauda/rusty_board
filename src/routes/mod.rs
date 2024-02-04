mod create_card;

use axum::{routing::{post, Route}, Router};
use create_card::create_card;
use sea_orm::DatabaseConnection;


#[derive(Clone)]
pub struct SharedData {
    database_connection: DatabaseConnection,
}

pub fn create_routes(database_connection: DatabaseConnection) -> Router{
    let shared_data = SharedData {
        database_connection: database_connection,
    };

    Router::new()
    .route("/create_card", post(create_card))
    .with_state(shared_data)
}