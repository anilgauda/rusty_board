use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, Set};

use crate::{entity::card, routes::SharedData, utils::app_error::AppError};
use super::card_request::CardRequest;

pub async fn create_card(
    State(state): State<SharedData>,
    Json(card_request): Json<CardRequest>,
) -> Result<(), AppError> {
    let new_card = card::ActiveModel {
        title: Set(card_request.title),
        description: Set(card_request.description),
        created_date: Set(card_request.created_date),
        is_active: Set(card_request.is_active),
        ..Default::default()
    };

    new_card
        .save(&state.database_connection)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error Occured while Inserting",
            )
        })?;
    Ok(())
}
