mod trial_askama;

use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder, Result,
};
use my_ca::{
    key::service,
    key_repository::{postgres_key_repository::PostgresKeyRepository, Key},
};
use shuttle_actix_web::ShuttleActixWeb;

use askama::Template;
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    content: String,
}

#[get("")]
async fn index() -> Result<impl Responder> {
    let index = IndexTemplate {
        title: "Steel Borne".to_string(),
        content: "neruneruna7の技術お試し用のwebサイトです".to_string(),
    };
    let renderd = index.render().unwrap();

    Ok(HttpResponse::Ok().body(renderd))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let _ = pool
        .execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)
        .unwrap();

    let key_repository = my_ca::key_repository::postgres_key_repository::PostgresKeyRepository::new(
        pool,
        Vec::new(),
    );
    key_repository.init().await.unwrap();
    let key_repository = actix_web::web::Data::new(key_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(web::scope("/").service(index));
        cfg.service(
            web::scope("/ta")
                .service(trial_askama::trial_askama)
                .service(trial_askama::trial_askama_list),
        )
        .service(
            web::scope("/ubiquitimes")
                .app_data(key_repository)
                .configure(service),
        );
    };

    Ok(config.into())
}
