use std::borrow::Borrow;

use sqlx::Row;

//// Postgresにはu64型を入れられないので，Stringに変換する
/// 呼び出し元が気にしないでいいように（u64として扱えるように）すること
use super::{Key, KeyRepository, KeyResult};

/// Postgresにはu64型を入れられないので，Stringに変換する
/// 変換するときに使う構造体
#[derive(sqlx::FromRow)]
struct PostgresKey {
    guild_id: String,
    public_key: String,
    manage_webhook: String,
}

// Fromトレイトを実装して，Keyに変換できるようにする
impl From<PostgresKey> for Key {
    fn from(p: PostgresKey) -> Self {
        Key {
            guild_id: p.guild_id.parse().unwrap(),
            public_key: p.public_key,
            manage_webhook: p.manage_webhook,
        }
    }
}

pub struct PostgresKeyRepository {
    pool: sqlx::PgPool,
    pub(crate) secret: Vec<Key>,
}

impl PostgresKeyRepository {
    pub fn new(pool: sqlx::PgPool, secret: Vec<Key>) -> Self {
        Self { pool, secret }
    }
}

impl KeyRepository for PostgresKeyRepository {
    /// 公開鍵を格納する
    ///
    /// しかるべき者以外は操作してはならない
    async fn set_key(&self, key: Key) -> KeyResult<Key> {
        let guild_id = format!("{}", key.guild_id);
        let r = sqlx::query_as::<_, PostgresKey>(
            r#"
            INSERT INTO keys (guild_id, public_key, manage_webhook)
            VALUES ($1, $2, $3)
            RETURNING guild_id, public_key, manage_webhook
            "#,
        )
        .bind(guild_id)
        .bind(key.public_key)
        .bind(key.manage_webhook)
        .fetch_one(&self.pool)
        .await?
        .into();
        Ok(r)
    }

    /// 公開鍵を取得する
    ///
    /// 誰が呼び出してもいい
    async fn get_key(&self, guild_id: u64) -> KeyResult<Key> {
        // Postgresにu64を入れられないので，Stringに変換する
        let guild_id = format!("{}", guild_id);
        let r = sqlx::query_as::<_, PostgresKey>(
            r#"
            SELECT guild_id, public_key, manage_webhook
            FROM keys
            WHERE guild_id = $1
            "#,
        )
        .bind(guild_id)
        .fetch_one(&self.pool)
        .await?
        .into();

        Ok(r)
    }

    /// 公開鍵を更新する
    ///
    /// しかるべき者以外は操作してはならない
    async fn update_key(&self, key: Key) -> KeyResult<Key> {
        let guild_id = format!("{}", key.guild_id);
        let r = sqlx::query_as::<_, PostgresKey>(
            r#"
            UPDATE keys
            SET public_key = $2, manage_webhook = $3
            WHERE guild_id = $1
            RETURNING guild_id, public_key, manage_webhook
            "#,
        )
        .bind(key.public_key)
        .bind(guild_id)
        .bind(key.manage_webhook)
        .fetch_one(&self.pool)
        .await?
        .into();

        Ok(r)
    }

    /// 公開鍵を削除する
    ///
    /// しかるべき者以外は操作してはならない
    async fn delete_key(&self, guild_id: u64) -> KeyResult<Key> {
        let guild_id = format!("{}", guild_id);
        let r = sqlx::query_as::<_, PostgresKey>(
            r#"
            DELETE FROM keys
            WHERE guild_id = $1
            RETURNING guild_id
            "#,
        )
        .bind(guild_id)
        .fetch_one(&self.pool)
        .await?
        .into();

        Ok(r)
    }

    async fn upsert_key(&self, key: Key) -> KeyResult<Key> {
        let guild_id = format!("{}", key.guild_id);
        let r = sqlx::query_as::<_, PostgresKey>(
            r#"
            INSERT INTO keys (guild_id, public_key)
            VALUES ($1, $2, $3)
            ON CONFLICT (guild_id) DO UPDATE
            SET public_key = $2, manage_webhook = $3
            RETURNING guild_id, public_key, manage_webhook
            "#,
        )
        .bind(guild_id)
        .bind(key.public_key)
        .bind(key.manage_webhook)
        .fetch_one(&self.pool)
        .await?
        .into();

        Ok(r)
    }
}
