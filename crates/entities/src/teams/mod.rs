pub mod team_members;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::teams::team_members::Entity")]
    TeamMembers,
    #[sea_orm(has_many = "crate::projects::Entity")]
    Projects,
    #[sea_orm(has_many = "crate::custom_languages::languages::Entity")]
    CustomLanguages,
    #[sea_orm(has_many = "crate::custom_languages::categories::Entity")]
    CustomLanguageCategories,
}

impl Related<crate::teams::team_members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamMembers.def()
    }
}

impl Related<crate::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}
impl Related<crate::custom_languages::languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomLanguages.def()
    }
}
impl Related<crate::custom_languages::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomLanguageCategories.def()
    }
}
