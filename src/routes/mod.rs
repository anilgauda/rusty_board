mod hello_world;
mod mirror_body;
mod mirror_body_json;
mod path_variables;
mod query_parameter;
mod mirror_user_agent;
mod middleware_message;
mod middleware_custom_header;
mod always_errors;
use axum::{http::Method, middleware, routing::{get, post}, Router};
use hello_world::hello_world;
use mirror_body::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use query_parameter::query_params;
use mirror_user_agent::user_agent;
use middleware_message::middleware_message;
use tower_http::cors::{Any,CorsLayer};
use middleware_custom_header::middleware_custom_header;
use always_errors::always_errors;

#[derive(Clone)]
pub struct SharedData{
    message:String,
}

pub fn get_routes() -> Router {

    let cors = CorsLayer::new().allow_methods([Method::GET,Method::POST]).allow_origin(Any);
    let shared_data = SharedData {
        message:"This is shared data".to_string(),
    };

    Router::new()
    .route("/", get(hello_world))
    .route("/mirror_body_string",post(mirror_body_string))
    .route("/mirror_body_json",post(mirror_body_json))
    .route("/path_variables/:id",get(path_variables))
    .route("/query_parameter",get(query_params))
    .route("/mirror_user_agent", get(user_agent))
    .route("/middleware_message", get(middleware_message))
    .with_state(shared_data)
    .layer(cors)
    .route_layer(middleware::from_fn(middleware_custom_header)) // Adding layer for token validation using middleware
    .route("/always_errors",get(always_errors))
}
