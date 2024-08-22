-- Add migration script here
-- create user table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    ws_id BIGINT NOT NULL,
    fullname VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL,
    password_hash VARCHAR(97) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
-- workspace for users
CREATE TABLE IF NOT EXISTS workspace (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE,
    owner_id BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
BEGIN;
-- add super user 0
INSERT INTO users (id, ws_id, fullname, email, password_hash)
VALUES (0, 0, 'super user', 'super@none.org', '');
INSERT INTO workspace(id, name, owner_id)
VALUES (0, 'none', 0);
commit;
-- add foreign key constraint for ws_id for users
ALTER TABLE users
ADD CONSTRAINT users_ws_id_fkey FOREIGN KEY (ws_id) REFERENCES workspace(id);
-- create index for users for email
CREATE UNIQUE INDEX IF NOT EXISTS email_index ON users(email);
-- create chat type: single, group, private_channel, public_channel
CREATE TYPE chat_type AS ENUM (
    'single',
    'group',
    'private_channel',
    'public_channel'
);
-- create chat table
CREATE TABLE IF NOT EXISTS chats(
    id BIGSERIAL PRIMARY KEY,
    ws_id BIGINT REFERENCES workspace(id),
    name VARCHAR(64),
    type chat_type NOT NULL,
    -- user id list
    members BIGINT [] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- create message table
CREATE TABLE IF NOT EXISTS messages(
    id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL REFERENCES chats(id),
    sender_id BIGINT NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    files TEXT [] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- create index for messages for chat_id and create_at order by create_at desc
CREATE INDEX IF NOT EXISTS chat_id_created_at_idx ON messages(chat_id, created_at DESC);
-- create index for messages for sender_id
CREATE INDEX IF NOT EXISTS sender_id_idx ON messages(sender_id, created_at DESC);
