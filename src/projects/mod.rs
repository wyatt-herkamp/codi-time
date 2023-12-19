use actix_web::{
    get,
    web::{self, Data, Query},
    HttpResponse,
};
use common::{PartialProjectQuery, Project, ProjectQuery};
use sea_orm::DatabaseConnection;

use crate::{error::WebsiteError, user::Authentication};
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(projects_list);
}

#[utoipa::path(get,
    impl_for=projects_list,
    path = "/api/projects/list",
    responses(
        (status = 200, description = "Projects you are have access to", body = Vec<Project>),
    ),
    security(
        ("api_key" = []),
        ("session" = [])
    )
)]
#[get("/projects/list")]
pub async fn projects_list(
    auth: Authentication,
    query: Query<PartialProjectQuery>,
    database: Data<DatabaseConnection>,
) -> Result<HttpResponse, WebsiteError> {
    let query = query.into_inner();
    let full_query = ProjectQuery::from((auth.id(), query));
    let projects =
        entities::projects::query_projects::<Project>(full_query, database.as_ref()).await?;
    if let Some(projects) = projects {
        Ok(HttpResponse::Ok().json(projects))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
