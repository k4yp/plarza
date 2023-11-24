CREATE TABLE "post" (
    "postid" SERIAL PRIMARY KEY NOT NULL,
    "link" TEXT NOT NULL,
    "user" TEXT NOT NULL,
    "source" TEXT NOT NULL,
    "date" BIGINT NOT NULL,
    "caption" TEXT NOT NULL,
    "media_path" TEXT,
    "media_link" TEXT
);