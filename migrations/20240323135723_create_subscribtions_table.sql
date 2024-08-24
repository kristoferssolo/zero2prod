-- Add migration script here
CREATE TABLE SUBSCRIPTIONS (
    "id" uuid PRIMARY KEY NOT NULL,
    "email" text NOT NULL UNIQUE,
    "name" text NOT NULL,
    "subscribed_at" timestamptz NOT NULL
);

