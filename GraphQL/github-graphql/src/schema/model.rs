use crate::*;


#[derive(Deserialize, SimpleObject, Serialize, Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub body: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub url: String,
}

// pub struct Issues {
//     pub issues: Vec<Issue>,
// }
#[derive(InputObject)]
pub struct CreateIssue {
    pub repo_id: String,
    pub title: String,
    pub body: String,
}

#[derive(InputObject)]
pub struct FetchIssue {
    pub issue_id: String,
}
#[derive(InputObject,Deserialize,Serialize,Debug,Clone)]
pub struct Label{
    pub id: String,
    pub name: String,
    pub description: String,
}