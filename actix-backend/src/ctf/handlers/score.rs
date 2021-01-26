use std::{collections::HashMap, vec};

use crate::{
    api::ApiResult,
    ctf::models::{
        flag::{Flag, IFlag},
        score::{IScore, NewScore},
        users::{IUser, User},
    },
    how::Error,
    state::AppState,
};
use actix_web::{get, post};
use actix_web::{web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ScoreWithObjects {
    user: User,
    falgs: Vec<Flag>,
    score: f64,
}

#[get("")]
async fn get_score(_req: HttpRequest, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    // ---------- STAGE 1 ---------- //
    // Get all flags
    let flags = &state.flag_all_without_answer().await?;
    // Get all scores for every flag
    let mut score_mapping = HashMap::new();
    for f in flags.iter() {
        let scores = &state.score_for_flag(f.id).await?;
        // Calculate the 1/n score for every flag based on the score count (create a hash map)
        score_mapping.insert(f.id, scores.len());
    }

    // ---------- STAGE 2 ---------- //
    // Get all users
    let users = &state.user_all().await?;
    // Get scores for every user
    let mut result: Vec<ScoreWithObjects> = Vec::new();
    for u in users.iter() {
        let scores = &state.score_for_user(u.id).await?;

        let mut user_score = 0.0;
        for item in scores {
            let flag = &state.flag_query(item.flag_id).await?;
            user_score += score_mapping.get(&flag.id).unwrap_or(0);
        }

        // let accumulated_flags: Vec<Flag> = scores
        //     .iter()
        //     .map(|e| async { &state.flag_query(e.flag_id).await? }).collect();
        for item in accumulated_flags {

        }
            // .fold(0, |acc, item| {
            //     let resp = item.await?;
            //     acc += score_mapping.get(res)
            // });
        // Create the response ovject
        // result.append(ScoreWithObjects {
        //     user: u,
        //     flags:
        // })
    }

    let res = ApiResult::new().with_msg("ok").with_data(result);
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
        false => ApiResult::new().with_msg("Incorrect answer!").code(400),
    };
    Ok(res.to_resp())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_score);
    cfg.service(create_score_entry);
}
