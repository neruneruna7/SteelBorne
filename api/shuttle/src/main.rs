use actix_web::{get, web::ServiceConfig, HttpResponse, Responder, Result};
use shuttle_actix_web::ShuttleActixWeb;


use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let index = IndexTemplate {
        content: "neruneruna7の技術お試し用のwebサイトです".to_string(),
    };
    let renderd = index.render().unwrap();

    Ok(HttpResponse::Ok().body(renderd))
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index);
    };

    Ok(config.into())
}
