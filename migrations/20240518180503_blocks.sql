-- Add migration script here
CREATE TABLE IF NOT EXISTS blocks (
  id serial PRIMARY KEY,
  number BIGINT UNIQUE NOT NULL,
  data JSONB NOT NULL
);

