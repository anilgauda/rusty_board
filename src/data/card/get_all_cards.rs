use axum::{extract::State, http::StatusCode, Json};
use sea_orm::EntityTrait;

use crate::{entity::card, routes::SharedData, utils::app_error::AppError};

use super::card_response::CardResponse;

pub async fn get_all_cards(
    State(state): State<SharedData>,
) -> Result<Json<Vec<CardResponse>>, AppError> {
    let cards = card::Entity::find()
        .all(&state.database_connection)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "No Element in Database".to_owned(),
            )
        })?
        .into_iter()
        .map(|card| CardResponse {
            id: card.id,
            list_id: card.list_id,
            description: card.description,
            created_date: card.created_date,
            title: card.title,
            is_active: card.is_active,
            due_date: card.due_date,
            reminder_date: card.reminder_date,
        })
        .collect();

    Ok(Json(cards))
}