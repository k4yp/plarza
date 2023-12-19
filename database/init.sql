CREATE TABLE "user" (
    "user_id" SERIAL PRIMARY KEY NOT NULL,
    "username" VARCHAR(32) UNIQUE NOT NULL,
    "email" VARCHAR(128) NOT NULL,
    "password" VARCHAR(256) NOT NULL,
    "salt" VARCHAR(256) NOT NULL,
    "bio" VARCHAR(256),
    "date" BIGINT NOT NULL,
    "display" VARCHAR(32),
    "pfp" VARCHAR(256)
);

CREATE TABLE "feed" (
    "feed_id" SERIAL PRIMARY KEY NOT NULL,
    "username" VARCHAR(32) NOT NULL,
    "bio" VARCHAR(256),
    "display" VARCHAR(32),
    "pfp" VARCHAR(256),
    "date" BIGINT NOT NULL
);

CREATE TABLE "post" (
    "post_id" SERIAL PRIMARY KEY NOT NULL,
    "url_id" TEXT NOT NULL,
    "user_id" INTEGER REFERENCES "user"("user_id"),
    "feed_id" INTEGER REFERENCES "feed"("feed_id"),
    "source" VARCHAR(32) NOT NULL,
    "date" BIGINT NOT NULL,
    "caption" TEXT NOT NULL,
    "media_url" TEXT
);

CREATE TABLE "connections" (
    "connection_id" SERIAL PRIMARY KEY NOT NULL,
    "user_id" INTEGER REFERENCES "user"("user_id"),
    "feed_id" INTEGER REFERENCES "feed"("feed_id"),
    "source" VARCHAR(32) NOT NULL,
    "date" BIGINT NOT NULL
);

INSERT INTO "user" ("username", "email", "password", "salt", "date")
VALUES ('test', 'test@plarza.com', '$argon2id$v=19$m=19456,t=2,p=1$gkc15RVOcVHKXDYGvZMcww$d+V7MsxUnGNXzMzcAK/vEhqXKkLJjrmsn/cxWR2sJ3g', 'gkc15RVOcVHKXDYGvZMcww', 1700000000);