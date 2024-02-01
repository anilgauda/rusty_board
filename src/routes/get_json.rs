use axum::Json;
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
pub struct JsonResponse {
    message: String,
    count:i32,
    username: String,
}
pub async fn get_json() -> Json<JsonResponse>{
    let response = JsonResponse {
        message:"Testing Json Response".to_owned(),
        count: 15,
        username: "Test".to_owned(),
    };

    Json(response)
}
