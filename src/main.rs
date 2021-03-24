#[macro_use]
extern crate lazy_static;

mod api;
mod models;

use std::{env, path::PathBuf};

// use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpRequest, HttpServer, Result as ActixResult};
use anyhow::{Context, Result};
use butane::db::{Connection, ConnectionSpec};

fn get_env_with_dev_default(key: &'static str, default: &'static str) -> String {
    env::var(key)
        .map_or_else(
            |_| {
                if cfg!(debug_assertions) {
                    Some(default.to_string())
                } else {
                    None
                }
            },
            Option::Some,
        )
        .expect(format!("Expected {} to be set", key).as_str())
}

lazy_static! {
    static ref FRONTEND_PATH: PathBuf = env::var("FRONTEND_PATH")
        .unwrap_or("../client/dist".to_string())
        .into();
    static ref APP_HOST: String = get_env_with_dev_default("APP_HOST", "localhost");
    static ref APP_PORT: String = get_env_with_dev_default("APP_PORT", "8080");
    static ref APP_ADDR: String = format!("{}:{}", *APP_HOST, *APP_PORT);
}

async fn index(_: HttpRequest) -> ActixResult<NamedFile> {
    Ok(NamedFile::open(FRONTEND_PATH.join("index.html"))?)
}

fn db_connect() -> Result<Connection> {
    let connection = butane::db::connect(
        &ConnectionSpec::load(".butane/connection.json")
            .context("missing database connection configuration")?,
    )
    .context("failed to connect to database")?;

    Ok(connection)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    HttpServer::new(move || {
        App::new()
            // .wrap(
            //     // TODO: Use separate CORS headers for debug/release
            //     Cors::new()
            //         .allowed_origin("http://localhost:8000")
            //         .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            //         .allowed_header(http::header::CONTENT_TYPE)
            //         .finish(),
            // )
            .data(db_connect)
            .service(web::scope("/api").configure(api::config))
            .service(
                Files::new("", FRONTEND_PATH.clone())
                    .index_file("index.html")
                    .default_handler(web::get().to(index)),
            )
    })
    .bind(APP_ADDR.clone())?
    .run()
    .await?;

    Ok(())
}
