use axum::{extract::{Path, State}, http::StatusCode, Json};
use sea_orm::EntityTrait;

use crate::{entity::card, routes::SharedData, utils::app_error::AppError};

use super::card_response::CardResponse;

pub async fn select_card_by_id(
    State(state): State<SharedData>,
    Path(card_id): Path<i32>,
) -> Result<Json<CardResponse>, AppError> {
    let selected_card = card::Entity::find_by_id(card_id)
        .one(&state.database_connection)
        .await
        .unwrap();

    if let Some(selected_card) = selected_card {
        Ok(Json(CardResponse {
            id: selected_card.id,
            list_id: selected_card.list_id,
            description: selected_card.description,
            created_date: selected_card.created_date,
            title: selected_card.title,
            is_active: selected_card.is_active,
            due_date: selected_card.due_date,
            reminder_date: selected_card.reminder_date,
        }))
    } else {
        Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Given Id not found".to_owned(),
        ))
    }
}