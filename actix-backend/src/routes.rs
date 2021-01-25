
use crate::ctf;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // TODO: Healthcheck
        // .route("/health", web::get().to(health_check))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                .service( web::scope("/ctf").configure(ctf::routes))
        );
}
