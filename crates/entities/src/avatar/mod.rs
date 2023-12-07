use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use sea_orm_exports::SeaORMExports;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::gravatar::GravatarData;

#[derive(FromJsonQueryResult, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Source {
    Local { file: String },
    Gravatar { entry: crate::gravatar::Entry },
}
impl TryFrom<GravatarData> for Source {
    type Error = ();

    fn try_from(mut value: GravatarData) -> Result<Self, Self::Error> {
        if value.entry.len() > 0 {
            Ok(Source::Gravatar {
                entry: value.entry.remove(0),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, SeaORMExports)]
#[exports(Avatar, has_relation)]
#[sea_orm(table_name = "avatars")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// Has One relation to users::id
    pub user_id: i64,
    pub hash: String,
    pub source: Source,
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
