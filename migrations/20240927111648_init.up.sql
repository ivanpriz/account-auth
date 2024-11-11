-- Add up migration script here
DROP TABLE IF EXISTS "users";
CREATE TABLE "users" (
    id UUID PRIMARY KEY default gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    hashed_password VARCHAR(255) NOT NULL
);