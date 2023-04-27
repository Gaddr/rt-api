CREATE SCHEMA "grt";

CREATE TABLE "grt"."document" (
  "id" uuid PRIMARY KEY,
  "name" varchar UNIQUE,
  "created_at" timestamptz,
  "modified_at" timestamptz
);
