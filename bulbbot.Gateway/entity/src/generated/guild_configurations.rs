//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "guild_configurations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub guild_id: String,
    pub language_iso_code: String,
    pub has_premium: bool,
    pub auto_role: Option<String>,
    pub actions_on_info: bool,
    pub roles_on_leave: bool,
    pub quick_reasons: Option<Vec<String>>,
    pub manual_nick_name_infs: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::guilds::Entity",
        from = "Column::GuildId",
        to = "super::guilds::Column::GuildId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Guilds,
}

impl Related<super::guilds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guilds.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
