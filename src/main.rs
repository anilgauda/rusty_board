use rusty_board::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    dotenvy::dotenv().unwrap();

    let database_uri = dotenv!("DATABASE_URL");
    run(database_uri).await
}
