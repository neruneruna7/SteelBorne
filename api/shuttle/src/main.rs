mod trial_askama;

use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder, Result,
};
use my_ca::key_repository::{postgres_key_repository::PostgresKeyRepository, Key};
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

fn secret_store_read(secret_sotore: SecretStore) -> Result<Vec<Key>> {
    let row_num = if let Some(row_num) = secret_sotore.get("ROW_NUM") {
        row_num.parse::<usize>().unwrap()
    } else {
        panic!("ROW_NUM is not found");
    };

    let mut keys = Vec::new();
    for i in 0..row_num {
        if let Some(key) = secret_sotore.get(&format!("K{}", i)) {
            keys.push(serde_json::from_str::<Key>(&key).unwrap());
            // dbに格納する
        } else {
            panic!("KEY_{} is not found", i);
        }
    }

    Ok(keys)
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let secret_keys = secret_store_read(secret_store).unwrap();

    let _ = pool
        .execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)
        .unwrap();

    let key_repository = my_ca::key_repository::postgres_key_repository::PostgresKeyRepository::new(
        pool,
        secret_keys,
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
        .service(web::scope("/ubiquitimes").app_data(key_repository));
    };

    Ok(config.into())
}
