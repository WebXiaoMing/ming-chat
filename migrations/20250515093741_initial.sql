-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
  id bigserial PRIMARY KEY,
  fullname VARCHAR(64) NOT NULL,
  email VARCHAR(64) NOT NULL,
  avatar VARCHAR(64),
  hashed_password VARCHAR(128) NOT NULL,
  create_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS email_index on users(email);

CREATE TYPE chat_type as ENUM ('single', 'group', 'private_channel', 'public_channel');

CREATE TABLE IF NOT EXISTS chats (
  id bigserial PRIMARY KEY,
  name VARCHAR(128) NOT NULL UNIQUE,
  type chat_type DEFAULT 'single',
  members bigint[] NOT NULL,
  create_at TIMESTAMPTZ DEFAULT NOW()
);

-- create chat table
CREATE TABLE IF NOT EXISTS messages (
  id bigserial PRIMARY KEY,
  chat_id bigint NOT NULL references chats(id),
  sender_id bigint NOT NULL references users(id),
  content TEXT NOT NULL,
  images TEXT[],
  create_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS chat_id_create_at_index ON messages(chat_id, create_at DESC);
CREATE INDEX IF NOT EXISTS sender_id_index ON messages(sender_id, create_at DESC);
