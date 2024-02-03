use std::env;

use entity::SiteUser;
use rusty_board::run;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let database_connection =  sea_orm::Database::connect(db_url).await.unwrap();
    let builder = database_connection.get_database_backend();
    let schema =  Schema::new(builder);
    let statement = builder.build(&schema.create_table_from_entity(SiteUser::Entity));
    dbg!("{:?}",statement);
    run().await
}