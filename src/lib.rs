pub mod routes;
use routes::get_routes;
pub async fn run() {
    // Turn on the server
    let app = get_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}