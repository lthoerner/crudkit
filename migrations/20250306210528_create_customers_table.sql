CREATE SCHEMA main;

CREATE TABLE main.customers (
    id serial PRIMARY KEY,
    name text NOT NULL,
    email_address text,
    phone_number text,
    street_address text
);
