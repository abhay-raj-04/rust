use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

//owner schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Owner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<i32>,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(InputObject)]
pub struct CreateOwner {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(InputObject)]
pub struct FetchOwner {
    pub _id: i32,
}

//project schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<i32>,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: String,
}

#[derive(InputObject)]
pub struct CreateProject {
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: String,
}

#[derive(InputObject)]
pub struct FetchProject {
    pub _id: i32,
}
