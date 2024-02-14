use crate::key_repository::{KeyRepository, PostgresKeyRepository};
use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder, Result,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/oreoreca").service(get));
}

#[get("/get/{guild_id}")]
async fn get(
    guild_id: web::Path<u64>,
    repo: web::Data<PostgresKeyRepository>,
) -> Result<impl Responder> {
    let guild_id = guild_id.into_inner();
    let result = repo.get_key(guild_id).await;

    match result {
        Ok(key) => Ok(HttpResponse::Ok().json(key)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}
