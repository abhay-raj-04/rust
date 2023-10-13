use async_graphql::Error;
use serde::Serialize;

use crate::schemas::project_schema::{CreateOwner, CreateProject, Owner, Project, FetchOwner, FetchProject};

use super::read_write_json::{read_data, update_data};
pub struct Owners{
    pub owners:Vec<Owner>
}
impl Owners {
    

pub fn get_all_owners() -> Vec<Owner> {
    let owners: Vec<Owner> = read_data("owners.json").unwrap();
    owners
}
pub fn get_all_projects(&self) -> Vec<Project> {
    let projects: Vec<Project> = read_data("projects.json").unwrap();
    projects
}
pub fn create_owner(&self,new_owner: Owner) -> Owner {
    let mut owners: Vec<Owner> = read_data("owners.json").unwrap();

    let owner = Owner {
        _id: auto_generate_id(),
        name: new_owner.name.clone(),
        email: new_owner.email.clone(),
        phone: new_owner.phone.clone(),
    };
    owners.push(owner.clone());
    update_data("owner.json", &owners);

    owner
}
}
pub fn create_project(new_project: Project) -> Project {
    let mut projects: Vec<Project> = read_data("projects.json").unwrap();

    let project = Project {
        _id: auto_generate_projectid(),
        name: new_project.name.clone(),
        owner_id: new_project.owner_id.clone(),
        description: new_project.description.clone(),
        status: new_project.status.clone(),
    };
    projects.push(project.clone());
    update_data("projects.json", &projects);

    project
}
pub fn get_single_owner(id:FetchOwner) -> Result<Owner,String>{
    let owners: Vec<Owner> = read_data("owners.json").unwrap();

    for owner in owners{
        if owner._id == Some(id._id){
            return Ok(owner);
        }
    }
    Err("NotFound".to_string())
}
pub fn get_single_project(id:FetchProject) -> Result<Project,String>{
    let projects: Vec<Project> = read_data("projects.json").unwrap();

    for project in projects{
        if project._id == Some(id._id){
            return Ok(project);
        }
    }
    Err("NotFound".to_string())
}


fn auto_generate_id() -> Option<i32> {
    let owners: Vec<Owner> = read_data("owners.json").unwrap();

    let mut a = 202301;
    loop {
        for o in owners.clone() {
            if o._id == Some(a) {
                a += 1;
            }
        }
        break;
    }
    Some(a)
}
fn auto_generate_projectid() -> Option<i32> {
    let projects: Vec<Project> = read_data("projects.json").unwrap();

    let mut a = 2001;
    loop {
        for o in projects.clone() {
            if o._id == Some(a) {
                a += 1;
            }
        }
        break;
    }
    Some(a)
}
