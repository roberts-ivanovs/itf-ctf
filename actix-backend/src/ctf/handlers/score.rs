use crate::{
    api::ApiResult,
    ctf::models::{
        flag::IFlag,
        score::{IScore, NewScore},
    },
    how::Error,
    state::AppState,
};
use actix_web::{get, post};
use actix_web::{web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};

#[get("")]
async fn get_score(_req: HttpRequest, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let users = &state.score_all().await?;
    let res = ApiResult::new().with_msg("ok").with_data(users);
    Ok(res.to_resp())
}

#[derive(Serialize, Deserialize, Debug)]
struct NewScoreWithAnswer {
    metadata: NewScore,
    answer: String,
}

#[post("")]
async fn create_score_entry(
    form: web::Json<NewScoreWithAnswer>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    // Fetch the flag
    let flag = &state.flag_query(form.metadata.flag_id).await?;
    // Compare the answers
    let res = match flag.answer == form.answer {
        true => {
            // Save answer
            &state.score_add(&form.metadata).await?;
            ApiResult::new().with_msg("ok").with_data("Correct!")
        }
        false => ApiResult::new()
            .with_msg("Incorrect answer!")
            .code(400),
    };
    Ok(res.to_resp())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_score);
    cfg.service(create_score_entry);
}
