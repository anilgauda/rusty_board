use crate::data::{self, card::{create_card, get_all_cards, put_card, select_card_by_id}};

use axum::{routing::{get, post, put}, Router};
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
    .route("/get_card/:card_id", get(select_card_by_id))
    .route("/cards",get(get_all_cards))
    .route("/update_card/:card_id",put(put_card))
    .with_state(shared_data)
}