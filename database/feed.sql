CREATE TABLE "feed" (
    "userid" SERIAL PRIMARY KEY NOT NULL,
    "username" VARCHAR(32) NOT NULL,
    "desc" VARCHAR(256),
    "display" VARCHAR(32),
    "media" VARCHAR(256),
    "followers" INT DEFAULT 0
);