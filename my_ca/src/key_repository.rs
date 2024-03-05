use anyhow::Error;
pub mod postgres_key_repository;

pub use postgres_key_repository::PostgresKeyRepository;

use serde::{Deserialize, Serialize};
pub type KeyResult<T> = Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct KeyAndWebhook {
    guild_id: u64,
    public_key_pem: String,
    manage_webhook: String,
}

impl KeyAndWebhook {
    pub fn new(guild_id: u64, public_key_pem: &str, manage_webhook: &str) -> Self {
        let public_key_pem = public_key_pem.to_string();
        let manage_webhook = manage_webhook.to_string();
        Self {
            guild_id,
            public_key_pem,
            manage_webhook,
        }
    }
}

pub trait KeyRepository {
    async fn set_key(&self, key: KeyAndWebhook) -> KeyResult<KeyAndWebhook>;
    async fn get_key(&self, guild_id: u64) -> KeyResult<KeyAndWebhook>;
    async fn update_key(&self, key: KeyAndWebhook) -> KeyResult<KeyAndWebhook>;
    async fn upsert_key(&self, key: KeyAndWebhook) -> KeyResult<KeyAndWebhook>;
    async fn delete_key(&self, guild_id: u64) -> KeyResult<KeyAndWebhook>;
}
