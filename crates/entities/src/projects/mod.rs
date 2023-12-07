use common::version_control_ref::VersionControlRef;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub name: String,
    /// Other Names for the Project
    #[sea_orm(default_value = "{}")]
    pub renames: Vec<String>,
    #[sea_orm(default_value = "{}")]
    pub languages: Vec<String>,
    pub color: Option<String>,
    pub version_control_ref: Option<VersionControlRef>,
    pub public: bool,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub last_heartbeat: DateTimeWithTimeZone,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub last_update: DateTimeWithTimeZone,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}

// Foreign Key account to account::id

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::heartbeats::Entity")]
    Heartbeats,
    #[sea_orm(
        belongs_to = "crate::users::Entity",
        from = "Column::UserId",
        to = "crate::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "crate::teams::Entity",
        from = "Column::TeamId",
        to = "crate::teams::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Team,
}

impl Related<crate::connections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Heartbeats.def()
    }
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
