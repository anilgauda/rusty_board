use std::os::linux::raw::stat;

use crate::{entity::card, routes::SharedData, utils::app_error::AppError};
use axum::{
    extract::{path, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CardRequest {
    list_id: Option<i32>,
    title: Option<String>,
    description: Option<String>,
    created_date: Option<NaiveDate>,
    is_active: Option<bool>,
    due_date: Option<NaiveDate>,
    reminder_date: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardResponse {
    id: i32,
    list_id: Option<i32>,
    title: Option<String>,
    description: Option<String>,
    created_date: Option<NaiveDate>,
    is_active: Option<bool>,
    due_date: Option<NaiveDate>,
    reminder_date: Option<NaiveDate>,
}

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
        .map_err(|err| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error Occured while Inserting",
            )
        });
    Ok(())
}

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

pub async fn get_all_cards(
    State(state): State<SharedData>,
) -> Result<Json<Vec<CardResponse>>, AppError> {
    let cards = card::Entity::find()
        .all(&state.database_connection)
        .await
        .map_err(|err| {
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

pub async fn put_card(
    State(state): State<SharedData>,
    Path(card_id): Path<i32>,
    Json(card_request): Json<CardRequest>,
) -> Result<(), StatusCode> {
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

    card::Entity::update(updated_card).filter(card::Column::Id.eq(card_id)).exec(&state.database_connection).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}
