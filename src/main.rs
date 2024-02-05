use std::env;

use migration::MigratorTrait;
use rusty_board::run;

#[tokio::main]
async fn main(){
    //Database setup
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let database_connection =  sea_orm::Database::connect(db_url).await.unwrap();
    migration::Migrator::up(&database_connection, None).await.unwrap();
    // Start server
    run(database_connection).await
}