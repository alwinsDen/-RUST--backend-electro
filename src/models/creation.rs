extern crate diesel;

use crate::schema::activity_tracker::dsl::activity_tracker as tracker_insert;
use crate::schema::org_projects::dsl::org_projects as org_projects_insert;
use crate::schema::{activity_tracker, org_projects, users};
use crate::services::module_1::db_connection;
use diesel::{Insertable, RunQueryDsl};

#[derive(Insertable)]
#[table_name = "users"]
pub struct RegUsers {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "activity_tracker"]
pub struct ActivityStruct {
    pub user_id: uuid::Uuid,
    pub related_info: String,
}

impl ActivityStruct {
    pub fn save(&self) {
        diesel::insert_into(tracker_insert)
            .values(self)
            .execute(&db_connection())
            .expect("FAILED TO TRACK THE ACTIVITY");
    }
}

#[derive(Insertable)]
#[table_name = "org_projects"]
pub struct OrgProjectStruct {
    pub display_name: String,
    pub related_files: Vec<String>,
    pub user_id: uuid::Uuid,
}
impl OrgProjectStruct {
    pub fn create(&self) {
        diesel::insert_into(org_projects_insert)
            .values(self)
            .execute(&db_connection())
            .expect("ADDING TO TRACK FAIELD.");
    }
}
