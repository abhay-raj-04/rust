use std::{fs::File, io::Write};

use serde_json::Value;

use crate::{
    schema::model::Label,
    *,
};

pub struct Query;

#[Object(extends)]
impl Query {
    async fn get_all_issues(&self,owner:String,repo_name: String) -> FieldResult<Result<Vec<Value>,String>>{
        let query = r#"
        query GetIssues($owner: String!, $repoName: String!) {
            repository(owner: $owner, name: $repoName) {
                issues(last: 10) {
                    nodes {
                        id
                        title
                        body
                        state
                        createdAt
                        updatedAt
                        closedAt
                        url
                    }
                }
            }
        }"#;
        let variables = json!({
            "owner": owner,
            "repoName": repo_name,
        });
        let request_body = json!({
            "query": query,
            "variables": variables,
        });

        let response = response(&request_body).await;
        if response.status().is_success() {
            let response_json: Value = response.json().await?;
            let issues = response_json["data"]["repository"]["issues"]["nodes"]
                .as_array()
                .unwrap();
            
            Ok(Ok(issues.clone()))
        
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    
    }
    async fn get_labels(
        &self,
        owner: String,
        repo_name: String,
    ) -> Result<Vec<Value>, reqwest::Error> {
        let query = r#"
        query GetLabels($owner: String!, $repoName: String!) {
          repository(owner: $owner, name: $repoName) {
            labels(first: 20) {
              nodes {
                id
                name
                description
              }
            }
          }
        }
    "#;

        let variables = json!({
            "owner": owner,
            "repoName": repo_name,
        });

        let request_body = json!({
            "query": query,
            "variables": variables,
        });

        let response = response(&request_body).await;

        if response.status().is_success() {
            let response_json: Value = response.json().await?;
            let labels = response_json["data"]["repository"]["labels"]["nodes"]
                .as_array()
                .unwrap();
            update_data("labels.json", &labels);
            Ok(labels.clone())
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }

}

pub fn update_data(filename: &str, data: &Vec<Value>) {
    //convert &Vec<Value> to &Vec<Label>
    let mut labels: Vec<Label> = Vec::new();
    for value in data {
        let label = serde_json::from_value(value.clone()).unwrap();
        labels.push(label);
    }

    let mut file = File::create(filename).unwrap();
    file.write_all(serde_json::to_string_pretty(data).unwrap().as_bytes())
        .unwrap();
}
pub fn read_data(filename: &str) -> Vec<Label> {
    let file = File::open(filename).unwrap();
    let data: Vec<Label> = serde_json::from_reader(file).unwrap();
    data
}
