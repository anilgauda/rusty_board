use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create().table(Card::Table).if_not_exists()
        .col(ColumnDef::new(Card::Id).integer().not_null().auto_increment().primary_key(),)
        .col(ColumnDef::new(Card::ListId).integer())
        .col(ColumnDef::new(Card::Title).string())
        .col(ColumnDef::new(Card::Description).string())
        .col(ColumnDef::new(Card::CreatedDate).date())
        .col(ColumnDef::new(Card::IsActive).boolean())
        .col(ColumnDef::new(Card::DueDate).date())
        .col(ColumnDef::new(Card::ReminderDate).date())
        .to_owned(),
        
    ).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Card::Table).to_owned()).await

    }
}

#[derive(DeriveIden)]
enum Card {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "list_id")]
    ListId,
    #[sea_orm(iden = "title")]
    Title,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "is_active")]
    IsActive,
    #[sea_orm(iden = "due_date")]
    DueDate,
    #[sea_orm(iden = "reminder_date")]
    ReminderDate,
}
