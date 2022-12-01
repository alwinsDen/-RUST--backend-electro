-- Your SQL goes here
ALTER TABLE org_projects
    ALTER COLUMN id set default gen_random_uuid();