use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CardRequest {
    pub list_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_date: Option<NaiveDate>,
    pub is_active: Option<bool>,
    pub due_date: Option<NaiveDate>,
    pub reminder_date: Option<NaiveDate>,
}