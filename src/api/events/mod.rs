use actix_web::{web, HttpResponse, Responder};
use butane::{db::Connection, query, DataObject};

use crate::{
    api::ApiError,
    models::{Event, EventChallenge},
};

mod challenges;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::put().to(put)))
        .service(
            web::scope("/{event_id}")
                .service(web::resource("/").route(web::get().to(get)))
                .service(web::scope("/challenges").configure(challenges::config)),
        );
}

async fn get(
    web::Path(event_id): web::Path<String>,
    db: web::Data<Connection>,
) -> Result<impl Responder, ApiError> {
    let event = Event::get(db.as_ref(), &event_id)?;

    let resp_event: backend_models::Event = event.into();
    Ok(HttpResponse::Ok().json(&resp_event))
}

// FIXME: This should be a private route.
async fn put(
    details: web::Json<backend_models::Event>,
    db: web::Data<Connection>,
) -> Result<impl Responder, ApiError> {
    let mut event: Event = details.into_inner().into();

    if let Some((start, end)) = event.start_timestamp.zip(event.end_timestamp) {
        if start > end {
            return Ok(HttpResponse::BadRequest().finish());
        }
    }

    event.save(db.as_ref())?;

    Ok(HttpResponse::NoContent().finish())
}
