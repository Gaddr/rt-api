CREATE SCHEMA "gaddrdb";

CREATE TABLE "gaddrdb"."document" (
  "id" uuid PRIMARY KEY,
  "name" varchar UNIQUE,
  "created_at" timestamptz,
  "modified_at" timestamptz
);
