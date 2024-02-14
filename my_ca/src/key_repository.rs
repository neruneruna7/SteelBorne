use anyhow::Error;
pub mod postgres_key_repository;

use serde::{Deserialize, Serialize};
pub type KeyResult<T> = Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Key {
    guild_id: u64,
    public_key: String,
}

pub trait KeyRepository {
    async fn set_key(&self, guild_id: u64, public_key: String) -> KeyResult<Key>;
    async fn get_key(&self, guild_id: u64) -> KeyResult<Key>;
    async fn update_key(&self, guild_id: u64, public_key: String) -> KeyResult<Key>;
    async fn delete_key(&self, guild_id: u64) -> KeyResult<u64>;
}
