use actix_web::{web, HttpResponse, Responder};
use butane::{db::Connection, query, DataObject};

use crate::{
    api::ApiError,
    models::{Event, EventChallenge},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(get)));
}

async fn get(
    web::Path(id): web::Path<String>,
    db: web::Data<Connection>,
) -> Result<impl Responder, ApiError> {
    let event = Event::get(db.as_ref(), &id)?;

    // FIXME: Ensure that the requester has permission to view this event's challenges.

    // TODO: These should map to an actual challenge model rather than just names.
    let challenges = query!(EventChallenge, event == { id })
        .load(db.as_ref())?
        .iter()
        .map(|challenge| challenge.id.clone())
        .collect();

    let listing = backend_models::ChallengeListing { challenges };

    Ok(HttpResponse::Ok().json(&listing))
}
