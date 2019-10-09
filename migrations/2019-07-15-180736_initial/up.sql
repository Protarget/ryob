-- Your SQL goes here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    user_name TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
)