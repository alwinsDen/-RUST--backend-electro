-- Your SQL goes here
ALTER TABLE org_projects
ALTER COLUMN id TYPE uuid using gen_random_uuid();