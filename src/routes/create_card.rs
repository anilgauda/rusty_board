use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, Set};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::entity::card;

use super::SharedData;

#[derive(Serialize,Deserialize,Debug)]
pub struct CardRequest {
    list_id: Option<i32>,
    title: Option<String>,
    description: Option<String>,
    created_date: Option<NaiveDate>,
    is_active: Option<bool>,
    due_date: Option<NaiveDate>,
    reminder_date: Option<NaiveDate>,
}
pub async fn create_card(State(state): State<SharedData>, Json(card_request): Json<CardRequest>){
    let new_card =  card::ActiveModel{ 
        title: Set(card_request.title),
        description: Set(card_request.description),
        created_date: Set(card_request.created_date),
        is_active: Set(card_request.is_active),
        ..Default::default()
    };

    let response = new_card.save(&state.database_connection).await.unwrap();
    dbg!(response);

}