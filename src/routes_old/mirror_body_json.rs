use axum::Json;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJsonRequest{
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJsonResponse {
    message: String,
    status: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJsonRequest>) -> Json<MirrorJsonResponse>{
    Json(MirrorJsonResponse{
        message: body.message,
        status: "Received".to_owned()
    })

}