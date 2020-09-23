use actix_web::web;

// mod endpoint;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            // .service(web::scope("/endpoint").configure(endpoint::config)),
    );
}
