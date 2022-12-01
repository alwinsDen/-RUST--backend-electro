-- Your SQL goes here
create extension pgcrypto;
ALTER TABLE org_projects
    ALTER COLUMN id TYPE uuid using gen_random_uuid();