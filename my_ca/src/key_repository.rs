use anyhow::Error;
pub mod postgres_key_repository;

pub use postgres_key_repository::PostgresKeyRepository;

use serde::{Deserialize, Serialize};
pub type KeyResult<T> = Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Key {
    guild_id: u64,
    public_key: String,
}

impl Key {
    pub fn new(guild_id: u64, public_key: &str) -> Self {
        let public_key = public_key.to_string();
        Self {
            guild_id,
            public_key,
        }
    }
}

pub trait KeyRepository {
    async fn set_key(&self, key: Key) -> KeyResult<Key>;
    async fn get_key(&self, guild_id: u64) -> KeyResult<Key>;
    async fn update_key(&self, key: Key) -> KeyResult<Key>;
    async fn upsert_key(&self, key: Key) -> KeyResult<Key>;
    async fn delete_key(&self, guild_id: u64) -> KeyResult<u64>;
}
