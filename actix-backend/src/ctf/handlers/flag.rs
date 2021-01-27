use crate::{
    api::ApiResult,
    ctf::models::flag::{Flag, IFlag, NewFlag},
    how::Error,
    state::AppState,
};
use actix_multipart::Multipart;
use actix_web::{get, patch, post};
use actix_web::{web, HttpRequest, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

#[get("")]
async fn get_flags(_req: HttpRequest, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let users = &state.flag_all_without_answer().await?;
    let res = ApiResult::new().with_msg("ok").with_data(users);
    Ok(res.to_resp())
}

#[get("/single/{id}")]
async fn get_single_flag(
    path: web::Path<(u64,)>,
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let users = &state.flag_query(path.0 .0).await?;
    let res = ApiResult::new().with_msg("ok").with_data(users);
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

// Add file to a flag
// #[patch("/file/{id}")]
// async fn save_file(
//     mut payload: Multipart,
//     path: web::Path<(u64,)>,
//     state: web::Data<AppState>,
// ) -> Result<impl Responder, Error> {
//     // iterate over multipart stream
//     while let Ok(Some(mut field)) = payload.try_next().await {
//         let content_type = field.content_disposition().unwrap();
//         let filename = content_type.get_filename().unwrap();
//         let filepath = format!("/src/static/{}", &filename);

//         // File::create is blocking operation, use threadpool
//         let mut f = web::block(|| std::fs::File::create(filepath))
//             .await
//             .unwrap();

//         // Field in turn is stream of *Bytes* object
//         while let Some(chunk) = field.next().await {
//             let data = chunk.unwrap();
//             // filesystem operations are blocking, we have to use threadpool
//             f = web::block(move || f.write_all(&data).map(|_| f))
//                 .await
//                 .map_err(|_| Error::FileBlock)?;
//         }

//         // If file created succesffully -- Update the flag entry in the db
//         let old_flag = state.flag_query(path.0 .0).await?;
//         let old_flag = NewFlag::from_flag(old_flag);
//         state.flag_update(&old_flag, &path.0 .0).await?;
//     }
//     // TODO Update filename
//     let res = ApiResult::<()>::new().with_msg("ok").code(204);
//     Ok(res.to_resp())
// }

// NOTE: Flags are pre-created

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_flags);
    cfg.service(get_single_flag);
    cfg.service(get_flags_with_answers);
    cfg.service(update_flag);
    cfg.service(create_flags);
    // cfg.service(save_file);
}
