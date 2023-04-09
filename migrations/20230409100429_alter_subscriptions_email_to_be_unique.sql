-- Add migration script here
-- Alter email column to be unique
ALTER TABLE subscriptions
    ADD CONSTRAINT UQ_subscriptions_email UNIQUE (email);
