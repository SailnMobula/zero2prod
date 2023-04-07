-- Add migration script here
-- create table subscriptions
CREATE TABLE subscriptions
(
    id            uuid        NOT NULL,
    PRIMARY KEY (id),
    email         text        NOT NULL,
    name          text        NOT NULL,
    subscribed_at timestamptz NOT NULL
)