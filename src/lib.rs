pub mod routes;
use routes::get_routes;
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    //Database connection
    let database =  Database::connect(database_uri).await;

    // Turn on the server
    let app = get_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}