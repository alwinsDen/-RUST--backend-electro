-- Your SQL goes here
alter table activity_tracker
    drop column created_at;
alter table activity_tracker
    add column created_at timestamp default current_timestamp NOT NULL;