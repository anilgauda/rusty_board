use axum::{extract::{Path, State}, http::StatusCode};
use sea_orm::EntityTrait;

use crate::{entity::card, routes::SharedData, utils::app_error::AppError};

pub async fn delete_card(State(state): State<SharedData>,Path(card_id): Path<i32>) -> Result<(),AppError> {
    // card::Entity::delete_many().filter(card::Column::Id.eq(card_id)).exec(&state.database_connection).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    card::Entity::delete_by_id(card_id).exec(&state.database_connection).await.map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Unable to Delete the card".to_owned()))?;
    Ok(())
}