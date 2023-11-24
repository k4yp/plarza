CREATE TABLE "user" (
    "userid" SERIAL PRIMARY KEY NOT NULL,
    "username" VARCHAR(32) NOT NULL,
    "email" VARCHAR(128) NOT NULL,
    "bio" VARCHAR(256),
    "date" BIGINT NOT NULL,
    "display" VARCHAR(32),
    "media" VARCHAR(256),
    "password" VARCHAR(256)
);