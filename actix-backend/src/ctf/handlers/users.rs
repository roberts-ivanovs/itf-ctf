use crate::{
    api::ApiResult,
    ctf::models::users::{IUser, Register},
    ctf::models::namebuilder::{INameBuilder},
    how::Error,
    state::AppState,
};
use actix_web::{get, post};
use actix_web::{web, HttpRequest, Responder};

#[get("/email/{email}")]
async fn get_user_exists(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let email = path.into_inner().0;
    let exists = &state.user_by_email(email).await?;
    let res = ApiResult::new().with_msg("ok").with_data(exists);
    Ok(res.to_resp())
}

#[post("")]
async fn create_users(
    form: web::Json<Register>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let form = form.into_inner();
    // Generate a random name
    let random_name = state.get_unique_name().await?;
    let id = state.user_add(&form, random_name).await?;
    let res = ApiResult::new().with_msg("ok").with_data(id);
    Ok(res.to_resp())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_users);
    cfg.service(get_user_exists);
}

// #[cfg(test)]
// mod tests {
//     use crate::testing;
//     use futures::StreamExt;
//     use actix_web::{test, web, App};

//     // TODO Test all the endpoints

//     #[actix_rt::test]
//     async fn test_index_get() {
//         let spawned_app = testing::utils::spawn_app().await;
//         let mut app = test::init_service(
//             App::new()
//                 .data(spawned_app.state)
//                 .configure(crate::routes::routes),
//         )
//         .await;
//         let req = test::TestRequest::get()
//             .uri("/api/v1/ctf/user")
//             .to_request();
//         let mut resp = test::call_service(&mut app, req).await;

//         assert!(resp.status().is_success());
//         let (bytes, _) = resp.take_body().into_future().await;
//         let bytes = bytes.unwrap().unwrap();
//         let expected = web::Bytes::from_static(b"{\"code\":200,\"msg\":\"ok\",\"data\":[]}");
//         assert_eq!(expected, bytes);
//     }
// }
