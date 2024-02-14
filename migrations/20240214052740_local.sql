-- Add migration script here
-- テーブルを削除する
DROP TABLE IF EXISTS keys;

CREATE TABLE IF NOT EXISTS keys
(
    guild_id INT NOT NULL PRIMARY KEY,
    public_key text NOT NULL,
    created_at timestamp with time zone default CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);