-- Enable the UUID extension if it is not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the movies table if it doesn't already exist
CREATE TABLE IF NOT EXISTS movies
(
    id uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT movies_pkey PRIMARY KEY,
    title text NOT NULL,
    director text NOT NULL,
    year smallint NOT NULL,
    poster text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);

-- Create or replace the function that updates the updated_at column
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Drop the existing trigger if it exists
DROP TRIGGER IF EXISTS set_updated_at ON movies;

-- Create the trigger to update the updated_at column before each update
CREATE TRIGGER set_updated_at
BEFORE UPDATE ON movies
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
