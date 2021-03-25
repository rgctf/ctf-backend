use crate::models::Team;
use actix_web::{web, HttpResponse};
use butane::{db::Connection, query, DataObject};
use rand::prelude::*;
use std::ops::RangeInclusive;

const TOKEN_LEN: usize = 32;
const TOKEN_CHAR_RANGE: RangeInclusive<u8> = 0x21..=0x7e;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(create_team))
            .route(web::get().to(get_team))
    );
}

async fn create_team(name: web::Json<String>, db: web::Data<Connection>) -> HttpResponse {
    let name = name.clone();

    let test = query!(Team, name == { name }).load_first(db.as_ref()).unwrap();
    if test.is_some() {
        return HttpResponse::Conflict().finish();
    }

    let mut rng = thread_rng();
    let token: String = (0..TOKEN_LEN)
        .map(|_| -> char { rng.gen_range(TOKEN_CHAR_RANGE).into() })
        .collect();
    let mut new_team = Team {
        token,
        name,
        ..Team::default()
    };

    new_team.save(db.as_ref()).unwrap();

    let resp_team: backend_models::TeamPrivateDetails = new_team.into();
    HttpResponse::Ok().body(serde_json::to_string(&resp_team).unwrap())
}

async fn get_team(
    web::Path(id): web::Path<i64>,
    db: web::Data<Connection>,
) -> HttpResponse {
    let team = Team::get(db.as_ref(), id).unwrap();
    let resp_team: backend_models::Team = team.into();
    HttpResponse::Ok().body(serde_json::to_string(&resp_team).unwrap())
}
