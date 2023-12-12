CREATE TABLE "user" (
    "userid" SERIAL PRIMARY KEY NOT NULL,
    "username" VARCHAR(32) UNIQUE NOT NULL,
    "email" VARCHAR(128) NOT NULL,
    "password" VARCHAR(256) NOT NULL,
    "bio" VARCHAR(256),
    "date" BIGINT NOT NULL,
    "display" VARCHAR(32),
    "media" VARCHAR(256)
);