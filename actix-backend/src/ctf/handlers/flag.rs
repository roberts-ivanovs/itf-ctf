use crate::{
    api::ApiResult,
    how::Error,
    state::AppState,
    ctf::models::flag::IFlag,
};
use actix_web::{get};
use actix_web::{web, HttpRequest, Responder};

#[get("")]
async fn get_flags(_req: HttpRequest, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let users = &state.flag_all().await?;
    let res = ApiResult::new().with_msg("ok").with_data(users);
    Ok(res.to_resp())
}

// This one will also return the correct answer for a task
#[get("/admin/{id}")]
async fn get_users_flag_admin(
    path: web::Path<(u64,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner().0;
    let users = &state.flag_query(id).await?;
    let res = ApiResult::new().with_msg("ok").with_data(users);
    Ok(res.to_resp())
}

// This one will return only the question ID and meta-info without the answer
#[get("/{id}")]
async fn get_users_flag(
    path: web::Path<(u64,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner().0;
    let users = &state.flag_query_without_answer(id).await?;
    let res = ApiResult::new().with_msg("ok").with_data(users);
    Ok(res.to_resp())
}



// NOTE: Flags are pre-created

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_flags);
    cfg.service(get_users_flag);
}
