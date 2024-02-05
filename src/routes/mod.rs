use crate::data;

use axum::{routing::{get, post}, Router};
use data::card::create_card;
use data::card::select_card_by_id;
use sea_orm::DatabaseConnection;


#[derive(Clone)]
pub struct SharedData {
    pub database_connection: DatabaseConnection,
}

pub fn create_routes(database_connection: DatabaseConnection) -> Router{
    let shared_data = SharedData {
        database_connection: database_connection,
    };

    Router::new()
    .route("/create_card", post(create_card))
    .route("/get_card", get(select_card_by_id))
    .with_state(shared_data)
}