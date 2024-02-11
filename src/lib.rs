pub mod routes;
pub mod entity;
pub mod utils;
pub mod data;
use routes::create_routes;
use sea_orm::DatabaseConnection;

pub async fn run(database_connection: DatabaseConnection) {

    // Turn on the server
    let app = create_routes(database_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}