-- Add new columns user_name and password to the users table
ALTER TABLE users ADD COLUMN user_name TEXT NOT NULL;
ALTER TABLE users ADD COLUMN password TEXT NOT NULL;
