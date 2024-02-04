use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,Validate,Debug)]
pub struct User {
    #[validate(email)]
    username: String,
    #[validate(length(min =1))]
    password: String,
}

pub async fn custom_json_extractor(user:Json<User>) -> Result<Json<User>, (StatusCode, String)>{
    if let Err(errors) = user.validate() {
        dbg!(&errors);
        return Err((StatusCode::BAD_REQUEST,errors.to_string()));
    }

    Ok(user)
}