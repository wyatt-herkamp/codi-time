use common::IdOrName;
use sea_orm::{
    entity::prelude::*, sea_query::SimpleExpr, ActiveValue, IntoActiveModel, QuerySelect,
};

use crate::{TeamColumn, TeamEntity};
