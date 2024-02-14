//// Postgresにはu64型を入れられないので，Stringに変換する
/// 呼び出し元が気にしないでいいように（u64として扱えるように）すること
use super::{Key, KeyRepository, KeyResult};

pub struct PostgresKeyRepository {
    pool: sqlx::PgPool,
}

impl PostgresKeyRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl KeyRepository for PostgresKeyRepository {
    async fn set_key(&self, guild_id: u64, public_key: String) -> KeyResult<Key> {
        todo!()
    }

    async fn get_key(&self, guild_id: u64) -> KeyResult<Key> {
        // Postgresにu64を入れられないので，Stringに変換する
        let guild_id = format!("{}", guild_id);
        let r = sqlx::query!(
            r#"
            SELECT guild_id, public_key
            FROM keys
            WHERE guild_id = $1
            "#,
            guild_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Key {
            guild_id: r.guild_id.parse().unwrap(),
            public_key: r.public_key,
        })
    }

    async fn update_key(&self, guild_id: u64, public_key: String) -> KeyResult<Key> {
        todo!()
    }

    async fn delete_key(&self, guild_id: u64) -> KeyResult<u64> {
        todo!()
    }
}
