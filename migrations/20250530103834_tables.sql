-- Add migration script here
DROP TABLE IF EXISTS "vote";
DROP TABLE IF EXISTS "member";
DROP TABLE IF EXISTS "group";
DROP TYPE IF EXISTS DAYOFWEEK;

CREATE TYPE DAYOFWEEK AS ENUM (
    'Monday',
    'Tuesday',
    'Wednesday',
    'Thursday',
    'Friday',
    'Saturday',
    'Sunday'
);

CREATE TABLE "group" (
    "id" UUID PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL DEFAULT '',
    "description" VARCHAR(100),
    "year" INTEGER NOT NULL DEFAULT 2025
);

CREATE TABLE "member" (
    "id" SERIAL PRIMARY KEY,
    "group_id" UUID NOT NULL,
    "name" VARCHAR(100) NOT NULL DEFAULT '',
    "locked_reply" BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE "vote" (
    "id" SERIAL PRIMARY KEY,
    "group_id" UUID NOT NULL,
    "member_id" INTEGER NOT NULL,
    "week_number" INTEGER NOT NULL,
    "day_of_week" DAYOFWEEK NOT NULL DEFAULT 'Monday'
);

ALTER TABLE "member"
ADD CONSTRAINT "group_fk"
FOREIGN KEY ("group_id")
REFERENCES "group" ("id")
ON DELETE CASCADE;

ALTER TABLE "vote"
ADD CONSTRAINT "group_fk"
FOREIGN KEY ("group_id")
REFERENCES "group" ("id")
ON DELETE CASCADE;

ALTER TABLE "vote"
ADD CONSTRAINT "member_fk"
FOREIGN KEY ("member_id")
REFERENCES "member" ("id")
ON DELETE CASCADE;