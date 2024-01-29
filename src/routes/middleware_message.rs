use axum::extract::State;

use super::SharedData;

pub async fn middleware_message(State(state): State<SharedData>) -> String{
    state.message

}