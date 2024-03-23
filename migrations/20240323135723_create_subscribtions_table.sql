-- Add migration script here
CREATE TABLE SUBSCRIPTIONS (
  "id" UUID PRIMARY KEY NOT NULL,
  "email" TEXT NOT NULL UNIQUE,
  "name" TEXT NOT NULL,
  "subscribed_at" TIMESTAMPTZ NOT NULL
);
