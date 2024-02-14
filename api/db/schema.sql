-- テーブルを削除する
DROP TABLE IF EXISTS keys;

CREATE TABLE IF NOT EXISTS keys
(
    guild_id text NOT NULL PRIMARY KEY,
    public_key text NOT NULL,
    manage_webhook text NOT NULL,
    created_at timestamp with time zone default CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);