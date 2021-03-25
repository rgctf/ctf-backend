use actix_web::web;

mod teams;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").service(web::scope("/teams").configure(teams::config)));
}
