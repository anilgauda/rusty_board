use crate::data::card::CreateCard::create_card;
use crate::data::card::SelectCardById::select_card_by_id;
use crate::data::card::GetAllCards::get_all_cards;
use crate::data::card::PutCard::put_card;
use crate::data::card::PatchCard::patch_card;
use crate::data::card::DeleteCard::delete_card;

use axum::{routing::{delete, get, patch, post, put}, Router};
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
    .route("/update_card/:card_id", patch(patch_card))
    .route("/delete_card/:card_id", delete(delete_card))
    .with_state(shared_data)
}