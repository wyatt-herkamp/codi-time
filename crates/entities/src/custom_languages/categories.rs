use sea_orm::entity::prelude::*;

use super::Source;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    #[sea_orm(default_value = "FromAdmin")]
    pub source: Source,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::teams::Entity",
        from = "Column::TeamId",
        to = "crate::teams::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Team,
    #[sea_orm(
        belongs_to = "crate::users::Entity",
        from = "Column::UserId",
        to = "crate::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<crate::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<crate::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}
