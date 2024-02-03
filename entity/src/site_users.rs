use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name= "site_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email_address: String,
    pub password: String,
    pub signup_date: ChronoDate,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}
