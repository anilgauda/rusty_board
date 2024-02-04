mod hello_world;
mod mirror_body;
mod mirror_body_json;
mod path_variables;
mod query_parameter;
mod mirror_user_agent;
mod middleware_message;
mod middleware_custom_header;
mod always_errors;
mod returns_201;
mod get_json;
mod custom_json_extractor;

use axum::{http::Method, middleware, routing::{get, post}, Router};
use hello_world::hello_world;
use mirror_body::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use query_parameter::query_params;
use mirror_user_agent::user_agent;
use middleware_message::middleware_message;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any,CorsLayer};
use middleware_custom_header::middleware_custom_header;
use always_errors::always_errors;
use returns_201::returns_201;
use get_json::get_json;
use custom_json_extractor::custom_json_extractor;

#[derive(Clone)]
pub struct SharedData{
    message:String,
    connection: DatabaseConnection,
}

pub fn get_routes(database_connection: DatabaseConnection) -> Router {

    let cors = CorsLayer::new().allow_methods([Method::GET,Method::POST]).allow_origin(Any);
    let shared_data = SharedData {
        message:"This is shared data".to_string(),
        connection: database_connection,
    };

    Router::new()
    .route("/", get(hello_world))
    .route("/mirror_body_string",post(mirror_body_string))
    .route("/mirror_body_json",post(mirror_body_json))
    .route("/path_variables/:id",get(path_variables))
    .route("/query_parameter",get(query_params))
    .route("/mirror_user_agent", get(user_agent))
    .route("/middleware_message", get(middleware_message))
    .route("/returns_201", post(returns_201))
    .route("/get_json",get(get_json))
    .route("/custom_json_extractor",post(custom_json_extractor))
    .with_state(shared_data)
    .layer(cors)
    .route_layer(middleware::from_fn(middleware_custom_header)) // Adding layer for token validation using middleware
    .route("/always_errors",get(always_errors))
}
