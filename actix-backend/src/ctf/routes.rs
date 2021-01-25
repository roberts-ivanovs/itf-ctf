use actix_web::web;

use super::handlers::users;
use super::handlers::flag;
use super::handlers::score;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").configure(users::init));
    cfg.service(web::scope("/flag").configure(flag::init));
    cfg.service(web::scope("/score").configure(score::init));
}
