mod hello_world;
mod mirror_body;
mod mirror_body_json;
mod path_variables;
mod query_parameter;
mod mirror_user_agent;
use axum::{routing::{get, post}, Router};
use hello_world::hello_world;
use mirror_body::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use query_parameter::query_params;
use mirror_user_agent::user_agent;

pub fn get_routes() -> Router {
    Router::new()
    .route("/", get(hello_world))
    .route("/mirror_body_string",post(mirror_body_string))
    .route("/mirror_body_json",post(mirror_body_json))
    .route("/path_variables/:id",get(path_variables))
    .route("/query_parameter",get(query_params))
    .route("/mirror_user_agent", get(user_agent))
}
