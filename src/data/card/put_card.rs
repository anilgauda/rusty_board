use axum::{extract::{Path, State}, http::StatusCode, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{entity::card, routes::SharedData, utils::app_error::AppError};

use super::card_request::CardRequest;

pub async fn put_card(
    State(state): State<SharedData>,
    Path(card_id): Path<i32>,
    Json(card_request): Json<CardRequest>,
) -> Result<(), AppError> {
    let updated_card = card::ActiveModel {
        id: Set(card_id),
        list_id: Set(card_request.list_id),
        title: Set(card_request.title),
        description: Set(card_request.description),
        created_date: Set(card_request.created_date),
        is_active: Set(card_request.is_active),
        due_date: Set(card_request.due_date),
        reminder_date: Set(card_request.reminder_date),
    };

    card::Entity::update(updated_card).filter(card::Column::Id.eq(card_id)).exec(&state.database_connection).await.map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Unable to update card".to_owned()))?;
    Ok(())
}