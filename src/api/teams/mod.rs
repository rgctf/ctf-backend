use std::ops::RangeInclusive;

use actix_web::{web, HttpResponse, Responder};
use butane::{db::Connection, query, DataObject};
use rand::prelude::*;

use crate::{api::ApiError, models::Team};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(get_team))
            .route(web::post().to(create_team)),
    );
}

fn generate_token<R: Rng + CryptoRng + ?Sized>(rng: &mut R) -> String {
    const TOKEN_LEN: usize = 32;
    const TOKEN_CHAR_RANGE: RangeInclusive<u8> = 0x21..=0x7e;

    (0..TOKEN_LEN)
        .map(|_| rng.gen_range(TOKEN_CHAR_RANGE))
        .map(char::from)
        .collect()
}

async fn get_team(
    web::Path(id): web::Path<i64>,
    db: web::Data<Connection>,
) -> Result<impl Responder, ApiError> {
    let team = Team::get(db.as_ref(), id)?;

    let resp_team: backend_models::Team = team.into();
    Ok(HttpResponse::Ok().json(&resp_team))
}

async fn create_team(
    name: web::Json<String>,
    db: web::Data<Connection>,
) -> Result<impl Responder, ApiError> {
    let name = name.clone();

    let existing_team = query!(Team, name == { name }).load_first(db.as_ref())?;

    if existing_team.is_some() {
        return Ok(HttpResponse::Conflict().finish());
    }

    let mut rng = thread_rng();
    let token = generate_token(&mut rng);

    let mut new_team = Team {
        token,
        name,
        ..Team::default()
    };

    new_team.save(db.as_ref())?;

    let resp_team: backend_models::TeamPrivateDetails = new_team.into();
    Ok(HttpResponse::Ok().json(&resp_team))
}
