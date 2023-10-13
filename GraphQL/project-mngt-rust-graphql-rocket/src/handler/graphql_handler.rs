// src/handler/graphql_handler.rs

use crate::{config::data_manager::DataManager, functions::read_write_json::update_data};
use crate::functions::functions::*;
use crate::schemas::project_schema::*;

use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};

pub struct Query;

#[Object(extends)]
impl Query {
    // Owners query
    async fn owner(&self, ctx: &Context<'_>, input: FetchOwner) -> FieldResult<Owner> {
        let data = &ctx.data::<DataManager>().unwrap();
        let owners = &data.owners;
        let owner = owners.iter().find(|o| o._id == Some(input._id));
        match owner {
            Some(owner) => Ok(owner.clone()),
            None => FieldResult::Err(format!("Owner '{}' not found", input._id).into()),
        }
    }

    async fn get_owners(&self, ctx: &Context<'_>) -> FieldResult<Vec<Owner>> {
        let data = &ctx.data::<DataManager>().unwrap();
        let owners = &data.owners;
        Ok(owners.clone())
    }

    // Projects query
    async fn project(&self, ctx: &Context<'_>, input: FetchProject) -> FieldResult<Project> {
        let data = &ctx.data::<DataManager>().unwrap();
        let projects = &data.projects;
        let project = projects.iter().find(|p| p._id == input._id.into());
        match project {
            Some(project) => Ok(project.clone()),
            None => FieldResult::Err(format!("Project '{}' not found", input._id).into()),
        }
    }

    async fn get_projects(&self, ctx: &Context<'_>) -> FieldResult<Vec<Project>> {
        let data = &ctx.data::<DataManager>().unwrap();
        let projects = &data.projects;
        Ok(projects.clone())
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    // Owner mutation
    async fn create_owner(&self, ctx: &Context<'_>, input: CreateOwner) -> FieldResult<Owner> {
        let data = &ctx.data::<DataManager>().unwrap();
        let mut all_owners = data.owners.clone();
        let new_owner = Owner {
            _id:Some(34),
            name: input.name.clone(),
            email: input.email.clone(),
            phone: input.phone.clone(),
        };
        all_owners.push(new_owner.clone());
        update_data("owners.json", &all_owners);
        Ok(new_owner)
    }

    // Project mutation
    async fn create_project(&self, ctx: &Context<'_>, input: CreateProject) -> FieldResult<Project> {
        let data = &ctx.data::<DataManager>().unwrap();
        let mut all_projects = data.projects.clone();
        let new_project = Project {
            _id:Some(23),
            owner_id:input.owner_id.clone(),
            name: input.name.clone(),
            description: input.description.clone(),
            status: input.status.clone(),
        };
        all_projects.push(new_project.clone());
        update_data("projects.json", &all_projects);
        Ok(new_project)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
