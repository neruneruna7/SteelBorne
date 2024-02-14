use actix_web::{get, HttpResponse, Responder, Result};

pub mod key_repository;

#[get("oreoreca/get")]
async fn get_publickey() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("ok"))
}
