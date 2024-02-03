use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // let builder = manager.get_database_backend();
        // let schema =  Schema::new(builder);
        // let statement = builder.build(&schema.create_table_from_entity(SiteUser::Entity));
        // dbg!("{:?}",statement);

        let builder=  manager.get_database_backend();
        let schema =  Schema::new(builder);
        let statement = builder.build(&schema.create_table_from_entity(SiteUser::Entity));

        manager.exec_stmt(statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
