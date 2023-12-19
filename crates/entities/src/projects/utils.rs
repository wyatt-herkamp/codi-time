use common::{
    project::{PartialProjectQuery, ProjectQuery, UserOrTeam},
    Project, ProjectSortBy,
};
use sea_orm::{
    entity::prelude::*, ConnectionTrait, FromQueryResult, Order, QueryOrder, QuerySelect,
};

use crate::{
    users::UserEntity, ProjectColumn, ProjectEntity, TeamEntity, TeamMemberColumn, TeamMemberEntity,
};

pub async fn get_projects_user_has_access_to(
    user: i64,
    database: &impl ConnectionTrait,
) -> Result<Vec<Project>, DbErr> {
    let teams: Vec<i64> = TeamMemberEntity::find()
        .select_only()
        .column(TeamMemberColumn::TeamId)
        .filter(TeamMemberColumn::UserId.eq(user))
        .into_tuple()
        .all(database)
        .await?;

    let query = if teams.is_empty() {
        ProjectColumn::UserId.eq(user.clone())
    } else {
        ProjectColumn::UserId
            .eq(user.clone())
            .or(ProjectColumn::TeamId.is_in(teams))
    };

    let projects: Vec<Project> = ProjectEntity::find()
        .filter(query)
        .into_model()
        .all(database)
        .await?;
    Ok(projects)
}

pub async fn query_projects<M: FromQueryResult>(
    query: ProjectQuery,
    database: &impl ConnectionTrait,
) -> Result<Option<Vec<M>>, DbErr> {
    let ProjectQuery {
        owned_by,
        query_params,
    } = query;
    let base_query = match owned_by {
        UserOrTeam::User {
            id_or_name,
            check_teams,
        } => {
            let Some(id) = id_or_name.get_id::<UserEntity>(database).await? else {
                return Ok(None);
            };
            if check_teams {
                let teams: Vec<i64> = TeamMemberEntity::find()
                    .select_only()
                    .column(TeamMemberColumn::TeamId)
                    .filter(TeamMemberColumn::UserId.eq(id))
                    .into_tuple()
                    .all(database)
                    .await?;
                // TODO condense this into one query
                if teams.is_empty() {
                    ProjectColumn::UserId.eq(id)
                } else {
                    ProjectColumn::UserId
                        .eq(id)
                        .or(ProjectColumn::TeamId.is_in(teams))
                }
            } else {
                ProjectColumn::UserId.eq(id)
            }
        }
        UserOrTeam::Team(id_or_name) => {
            let Some(id) = id_or_name.get_id::<TeamEntity>(database).await? else {
                return Ok(None);
            };
            ProjectColumn::TeamId.eq(id)
        }
    };

    let PartialProjectQuery {
        language,
        name,
        sort_by,
    } = query_params;
    let sql_query = if let Some(language) = language {
        base_query.and(ProjectColumn::Languages.contains(language))
    } else {
        base_query
    };
    let (sort_column, sort_order) = if let Some(sort_by) = sort_by {
        match sort_by {
            ProjectSortBy::Name(order) => (ProjectColumn::Name, order.unwrap_or_default().into()),
            ProjectSortBy::LastUpdate(order) => {
                (ProjectColumn::LastUpdate, order.unwrap_or_default().into())
            }
            ProjectSortBy::LastHeartbeat(order) => (
                ProjectColumn::LastHeartbeat,
                order.unwrap_or_default().into(),
            ),
        }
    } else {
        (ProjectColumn::Name, Order::Asc)
    };
    let sql_query = if let Some(name) = name {
        sql_query.and(
            ProjectColumn::Name
                .eq(name.clone())
                .or(ProjectColumn::Renames.contains(name)),
        )
    } else {
        sql_query
    };

    let projects: Vec<M> = ProjectEntity::find()
        .order_by(sort_column, sort_order)
        .filter(sql_query)
        .into_model()
        .all(database)
        .await?;
    Ok(Some(projects))
}
