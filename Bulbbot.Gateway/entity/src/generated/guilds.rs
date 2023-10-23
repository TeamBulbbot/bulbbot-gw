//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "guilds")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub guild_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::guild_configurations::Entity")]
    GuildConfigurations,
    #[sea_orm(has_many = "super::guild_loggings::Entity")]
    GuildLoggings,
    #[sea_orm(has_many = "super::messages::Entity")]
    Messages,
}

impl Related<super::guild_configurations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuildConfigurations.def()
    }
}

impl Related<super::guild_loggings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuildLoggings.def()
    }
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
