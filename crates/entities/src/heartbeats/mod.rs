use common::heartbeat::{CodeChanges, HeartbeatCategory, HeartbeatType};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "heartbeats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Foreign Key to User::id
    pub user_id: i64,
    /// The Path to the file that was being edited.
    pub entity: String,
    #[sea_orm(rename = "type")]
    pub type_: HeartbeatType,
    pub category: HeartbeatCategory,
    pub code_change: Option<CodeChanges>,
    /// Foreign Key to Project::id
    pub project: Option<i64>,
    pub branch: Option<String>,
    pub language: Option<String>,
    pub is_write: bool,
    pub editor: Option<String>,
    pub operating_system: Option<String>,
    pub machine_name_id: String,
    pub user_agent: String,
    /// When the heartbeat was started. Determined by the client.
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub start_time: DateTimeWithTimeZone,
    /// Initialized to the same value as start_time
    /// Updated when the next heartbeat with a different location is received
    /// Or when it has been at least 2 heart beat intervals since the last update(This can take time to update. But it will be set to 2 minutes after the current end_time)
    /// This can be determined by the client if provided to allow for a client to keep track of data when offline
    ///
    /// It will not span multiple days So if the start_time is 11:59 PM the next heartbeat will be a new entry
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub end_time: DateTimeWithTimeZone,
    /// Rather or not the heartbeat has been closed out.
    /// Used internally to determine if the heartbeat needs it's end_time updated
    pub closed: bool,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeWithTimeZone,
}
impl ActiveModelBehavior for ActiveModel {}

// Foreign Key account to account::id

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::users::Entity",
        from = "Column::UserId",
        to = "crate::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "crate::projects::Entity",
        from = "Column::Project",
        to = "crate::projects::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Project,
}

impl Related<crate::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<crate::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}
