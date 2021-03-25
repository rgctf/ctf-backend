use std::ops::RangeInclusive;

use actix_web::{HttpResponse, Responder, web};
use butane::{db::Connection, query, DataObject};
use rand::prelude::*;

use crate::models::Team;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(get_team))
            .route(web::post().to(create_team))
    );
}

fn generate_token<R: Rng + CryptoRng + ?Sized>(rng: &mut R) -> String {
    const TOKEN_LEN: usize = 32;
    const TOKEN_CHAR_RANGE: RangeInclusive<u8> = 0x21..=0x7e;

    (0..TOKEN_LEN)
        .map(|_| char::from(rng.gen_range(TOKEN_CHAR_RANGE)))
        .collect()
}

async fn get_team(web::Path(id): web::Path<i64>, db: web::Data<Connection>) -> impl Responder {
    let team = Team::get(db.as_ref(), id).unwrap();

    let resp_team: backend_models::Team = team.into();
    HttpResponse::Ok().json(&resp_team)
}

async fn create_team(name: web::Json<String>, db: web::Data<Connection>) -> impl Responder {
    let name = name.clone();

    let existing_team = query!(Team, name == { name })
        .load_first(db.as_ref()).unwrap();

    if existing_team.is_some() {
        return HttpResponse::Conflict().finish();
    }

    let mut rng = thread_rng();
    let token = generate_token(&mut rng);

    let mut new_team = Team {
        token,
        name,
        ..Team::default()
    };

    new_team.save(db.as_ref()).unwrap();

    let resp_team: backend_models::TeamPrivateDetails = new_team.into();
    HttpResponse::Ok().json(&resp_team)
}
