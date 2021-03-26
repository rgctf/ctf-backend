use actix_web::web;

mod error;
mod events;
mod teams;

pub use error::ApiError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .service(web::scope("/events").configure(events::config))
            .service(web::scope("/teams").configure(teams::config)),
    );
}
