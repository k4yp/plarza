CREATE TABLE "user" (
    "user_id" SERIAL PRIMARY KEY NOT NULL,
    "username" VARCHAR(32) UNIQUE NOT NULL,
    "email" VARCHAR(128) NOT NULL,
    "password" VARCHAR(256) NOT NULL,
    "bio" VARCHAR(256),
    "date" BIGINT NOT NULL,
    "display" VARCHAR(32),
    "media" VARCHAR(256)
);

CREATE TABLE "post" (
    "post_id" SERIAL PRIMARY KEY NOT NULL,
    "url_id" TEXT NOT NULL,
    "user_id" INTEGER REFERENCES "user"("user_id"),
    "source" TEXT NOT NULL,
    "date" BIGINT NOT NULL,
    "caption" TEXT NOT NULL,
    "media_path" TEXT,
    "media_link" TEXT
);

CREATE TABLE "feed" (
    "userid" SERIAL PRIMARY KEY NOT NULL,
    "username" VARCHAR(32) NOT NULL,
    "desc" VARCHAR(256),
    "display" VARCHAR(32),
    "media" VARCHAR(256),
    "followers" INT DEFAULT 0
);