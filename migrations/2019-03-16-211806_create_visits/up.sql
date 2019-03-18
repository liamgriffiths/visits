-- Your SQL goes here
-- Adds extension to let us add "exclusion" constraints.
CREATE EXTENSION IF NOT EXISTS btree_gist;

-- Create our users table :)
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT username_is_not_too_long CHECK (length(username) <= 64)
);

-- Use diesel-helper to auto-set updated_at.
SELECT diesel_manage_updated_at('users');

-- Create our visits table :)
CREATE TABLE visits (
  id SERIAL PRIMARY KEY,
  user_id  INTEGER REFERENCES users(id) NOT NULL,
  enter_at DATE NOT NULL,
  exit_at DATE NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  -- Verify that the entry date comes before the exit date.
  CONSTRAINT entry_is_before_exit CHECK (enter_at <= exit_at),

  -- Verify that a user does not have overlapping visits (ex. an entry before an exit happened).
  -- From https://dba.stackexchange.com/questions/110582/uniqueness-constraint-with-date-range
  CONSTRAINT no_overlapping_visits EXCLUDE USING gist (
    user_id WITH =,
    daterange(enter_at, exit_at, '[]') WITH &&
  )
);

-- Add an index on user_id lookups.
CREATE INDEX visits_user_id ON visits (user_id);

-- Use diesel-helper to auto-set updated_at.
SELECT diesel_manage_updated_at('visits');
