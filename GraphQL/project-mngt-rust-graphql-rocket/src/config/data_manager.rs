use crate::{schemas::project_schema::{Owner, Project}, functions::read_write_json::read_data};



pub struct DataManager{
    pub owners:Vec<Owner>,
    pub projects:Vec<Project>
}

impl DataManager{
    pub fn new() -> DataManager{
        DataManager{
            owners:read_data("owners.json").unwrap(),
            projects:read_data("projects.json").unwrap()
        }
    }
}