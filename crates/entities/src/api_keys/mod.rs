use common::{APITokenPermissions, user_types::api_token::FromCLI};
use sea_orm::entity::prelude::*;
pub mod utils;
use sea_orm_exports::SeaORMExports;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SeaORMExports)]
#[sea_orm(table_name = "api_keys")]
#[exports(APIKey, has_relation)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// Has One relation to users::id
    pub user_id: i64,
    /// API Key
    pub token: String,
    pub permissions: Vec<APITokenPermissions>,
    pub from_cli: Option<FromCLI>,
    /// Key is invalid. Invalid keys are kept for a short period for warning and logging purposes.
    pub revoked: Option<DateTimeWithTimeZone>,
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
