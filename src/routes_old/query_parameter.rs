use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Debug)]
pub struct QueryParamsRequest {
    id:i32,
    message: String,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct QueryParamsResponse{
    message: String,
}

pub async fn query_params(Query(query): Query<QueryParamsRequest>) ->  Json<QueryParamsResponse>{

    Json(QueryParamsResponse{
        message: query.message
    })

} 