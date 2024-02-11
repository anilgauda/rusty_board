use axum::{extract::{Path, State}, http::StatusCode, Json};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::{entity::card, routes::SharedData, utils::app_error::AppError};

use super::card_update_request::CardUpdateRequest;

pub async fn patch_card(State(state): State<SharedData>,Path(card_id): Path<i32>, Json(card_request): Json<CardUpdateRequest>) -> Result<(),AppError> {
    let db_connection = state.database_connection;

    let mut card_by_id =  if let Some(card) = card::Entity::find_by_id(card_id).one(&db_connection).await.map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error Occured while Updating card".to_owned()))?{
        card.into_active_model()
    }else {
        return Err(AppError::new(StatusCode::NOT_FOUND,"Card with given Id not found".to_owned()));
    };

    if let Some(list_id) = card_request.list_id {
        card_by_id.list_id= Set(list_id)
    }

    if let Some(title) = card_request.title {
        card_by_id.title= Set(title)
    }

    if let Some(description) = card_request.description {
        card_by_id.description= Set(description)
    }

    if let Some(created_date) = card_request.created_date {
        card_by_id.created_date= Set(created_date)
    }

    if let Some(is_active) = card_request.is_active {
        card_by_id.is_active= Set(is_active)
    }

    if let Some(due_date) = card_request.due_date {
        card_by_id.due_date= Set(due_date)
    }

    if let Some(reminder_date) = card_request.reminder_date {
        card_by_id.reminder_date= Set(reminder_date)
    }

    card::Entity::update(card_by_id).filter(card::Column::Id.eq(card_id)).exec(&db_connection).await.map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Failed to update the card".to_owned()))?;
    Ok(())
}