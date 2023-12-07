pub mod pub_connection;

use sea_orm::{entity::prelude::*, JsonValue};
use sea_orm_exports::SeaORMExports;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    DeriveActiveEnum,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Deserialize,
    Serialize,
    EnumString,
    Display,
    EnumIter,
)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Application {
    #[sea_orm(string_value = "Github")]
    Github,
    #[sea_orm(string_value = "WakaTime")]
    WakaTime,
}
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SeaORMExports)]
#[sea_orm(table_name = "connections")]
#[exports(Connection, has_relation)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// Has One relation to users::id
    pub user_id: i64,
    /// Used to store other data provided by the application
    /// Could Contain
    /// - Username
    pub other_data: Option<JsonValue>,
    /// Used to store other data provided by the application that should not be public
    pub other_data_private: Option<JsonValue>,
    pub application: Application,
    /// For Apps other than WakAPI this is the refresh token
    pub token: String,
    /// Not Available for WakAPI
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub expires_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created: DateTimeWithTimeZone,
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
}

impl Related<crate::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
