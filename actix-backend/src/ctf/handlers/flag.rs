use crate::{
    api::ApiResult,
    ctf::models::{
        flag::{AnswerlessFlag, Flag, IFlag, NewFlag},
        score::IScore,
        users::IUser,
    },
    how::Error,
    state::AppState,
};
use actix_multipart::Multipart;
use actix_web::{delete, get, patch, post, web::Json};
use actix_web::{web, HttpRequest, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AnswerlessFlagComposite {
    flag: AnswerlessFlag,
    total_answers: u64,
    total_answers_of_population: f64,
}

#[get("")]
async fn get_flags(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<Json<ApiResult<Vec<AnswerlessFlagComposite>>>, Error> {
    let flags = state.flag_all_without_answer().await?;
    let total_users = state.user_all().await?.len() as u64;
    let mut result = vec![];
    for flag in flags.into_iter() {
        let total_answers = state.score_for_flag(flag.id).await?.len() as u64;
        let flag_comp = AnswerlessFlagComposite {
            flag: flag,
            total_answers_of_population: total_answers as f64 / total_users as f64,
            total_answers,
        };
        result.push(flag_comp);
    }
    let res = ApiResult::new().with_msg("ok").with_data(result);
    Ok(Json(res))
}

#[get("/single/{id}")]
async fn get_single_flag(
    path: web::Path<(u64,)>,
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let flag = &state.flag_query(path.0 .0).await?;
    let res = ApiResult::new().with_msg("ok").with_data(flag);
    Ok(res.to_resp())
}

#[get("/answers")]
async fn get_flags_with_answers(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let flags = &state.flag_all().await?;
    let res = ApiResult::new().with_msg("ok").with_data(flags);
    Ok(res.to_resp())
}

#[derive(Serialize, Deserialize, Debug)]
struct FlagCreation {
    flag: NewFlag,
    file: Option<String>,
}

async fn store_file(filename: String, file: String) -> Result<(), Error> {
    // Try saving the file
    let file: Vec<&str> = file.split("base64,").collect();
    let res = base64::decode(file[1])?;

    // filesystem operations are blocking, we have to use threadpool
    let mut f = web::block(move || std::fs::File::create(format!("/app/static/{}", filename)))
        .await
        .map_err(|_| Error::FileBlock)?;
    f.write_all(&res)?;
    Ok(())
}

// Create flag without file
#[post("")]
async fn create_flags(
    _req: HttpRequest,
    field: web::Json<FlagCreation>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    // Create path buffer
    let flag_id = state.flag_add(&field.flag).await?;
    let res = ApiResult::new().with_msg("ok").with_data(flag_id);
    if let Some(file) = field.file.clone() {
        let filename = format!("task_{}.zip", &flag_id);
        store_file(filename.clone(), file).await?;
        let updated_flag = NewFlag {
            filepath: Some(filename),
            name: field.flag.name.clone(),
            answer: field.flag.answer.clone(),
            description: field.flag.description.clone(),
        };
        state.flag_update(&updated_flag, &flag_id).await?;
    };
    Ok(res.to_resp())
}

// Update flag
#[patch("/{id}")]
async fn update_flag(
    path: web::Path<(u64,)>,
    _req: HttpRequest,
    mut field: web::Json<FlagCreation>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let filename = format!("task_{}.zip", &path.0 .0);
    if let Some(file) = field.file.clone() {
        store_file(filename.clone(), file).await?;
        field.flag.filepath = Some(filename.clone());
    }
    // else {
    //     field.flag.filepath = None;
    // }
    state.flag_update(&field.flag, &path.0 .0).await?;
    let res = ApiResult::<()>::new().with_msg("ok").code(204);
    Ok(res.to_resp())
}

// Delete flag
#[delete("/{id}")]
async fn delete_flag(
    path: web::Path<(u64,)>,
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    state.flag_delete(path.0 .0).await?;
    let res = ApiResult::<()>::new().with_msg("ok").code(204);
    Ok(res.to_resp())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_flags);
    cfg.service(get_single_flag);
    cfg.service(create_flags);
    cfg.service(get_flags_with_answers);
    cfg.service(update_flag);
    cfg.service(delete_flag);
}
