use std::os::linux::raw::stat;

use crate::{entity::card, routes::SharedData, utils::app_error::AppError};
use axum::{
    extract::{path, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CardRequestPartialUpdate {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    list_id: Option<Option<i32>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    title: Option<Option<String>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    description: Option<Option<String>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    created_date: Option<Option<NaiveDate>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    is_active: Option<Option<bool>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    due_date: Option<Option<NaiveDate>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    reminder_date: Option<Option<NaiveDate>>,
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

pub async fn patch_card(State(state): State<SharedData>,Path(card_id): Path<i32>, Json(card_request): Json<CardRequestPartialUpdate>) -> Result<(),StatusCode> {
    let db_connection = state.database_connection;

    let mut card_by_id =  if let Some(card) = card::Entity::find_by_id(card_id).one(&db_connection).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
        card.into_active_model()
    }else {
        return Err(StatusCode::NOT_FOUND);
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

    card::Entity::update(card_by_id).filter(card::Column::Id.eq(card_id)).exec(&db_connection).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}
