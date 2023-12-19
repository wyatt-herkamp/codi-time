use sea_orm::{prelude::*, sea_query::SimpleExpr, EntityTrait, QuerySelect};

use crate::IdOrName;
pub trait BasicTableTrait: EntityTrait {
    const ID_COLUMN: Self::Column;
    const CREATED_COLUMN: Self::Column;
}
pub trait HasNameColumn: BasicTableTrait {
    const NAME_COLUMN: Self::Column;
}

impl IdOrName {
    /// Query based on the type of IdOrName
    #[inline(always)]
    pub fn query<T: HasNameColumn>(self) -> SimpleExpr {
        match self {
            Self::Id(id) => T::ID_COLUMN.eq(id),
            Self::Name(name) => T::NAME_COLUMN.eq(name),
        }
    }

    /// Convert IdOrName to an id.
    ///
    /// If the type is already an id, it will return the id.
    ///
    /// If the type is a name, it will generate a query to find the Id.
    #[inline(always)]
    pub async fn get_id<T: HasNameColumn>(
        self,
        database: &impl ConnectionTrait,
    ) -> Result<Option<i64>, DbErr> {
        match self {
            Self::Id(id) => Ok(Some(id)),
            Self::Name(name) => {
                let user = T::find()
                    .select_only()
                    .column(T::ID_COLUMN)
                    .filter(T::NAME_COLUMN.eq(name))
                    .into_tuple()
                    .one(database)
                    .await?;
                Ok(user)
            }
        }
    }
}
