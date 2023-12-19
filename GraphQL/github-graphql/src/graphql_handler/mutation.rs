use crate::{
    graphql_handler::query::read_data,
    schema::model::{CreateIssue, FetchIssue, Issue},
    *,
};

pub struct MyMutation;

#[async_graphql::Object]
impl MyMutation {
    async fn create_issue(&self, input: CreateIssue) -> FieldResult<Issue> {
        let mutation = r#"
            mutation CreateIssue($input: CreateIssueInput!) {
              createIssue(input: $input) {
                issue {
                  id
                  title
                  body
                  state
                }
              }
            }
        "#;

        let variables = json!({
            "input": {
                "repositoryId": input.repo_id,
                "title": input.title,
                "body": input.body,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = response(&request_body).await;

        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            let new_issue: Issue =
                serde_json::from_value(response_json["data"]["createIssue"]["issue"].clone())
                    .unwrap();
            //update_data(&new_issue);
            Ok(new_issue)
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to create issue: {}", error_message).into())
        }
    }

    async fn update_issue(
        &self,
        issue_id: String,
        new_title: String,
        new_body: String,
    ) -> FieldResult<Issue> {
        let mutation = r#"
                mutation UpdateIssue($input: UpdateIssueInput!) {
                  updateIssue(input: $input) {
                    issue {
                      id
                      title
                      body
                      state
                    }
                  }
                }
            "#;

        let variables = json!({
            "input": {
                "id": issue_id,
                "title": new_title,
                "body": new_body,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = response(&request_body).await;

        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            let updated_issue: Issue =
                serde_json::from_value(response_json["data"]["updateIssue"]["issue"].clone())
                    .unwrap();
            Ok(updated_issue)
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to update issue: {}", error_message).into())
        }
    }

    async fn delete_issue(&self, input: FetchIssue) -> FieldResult<String> {
        let mutation = r#"
        mutation DeleteIssue($input: DeleteIssueInput!) {
          deleteIssue(input: $input) {
            clientMutationId
          }
        }
    "#;

        let variables = json!({
            "input" : {
                "issueId" : input.issue_id,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = response(&request_body).await;

        if response.status().is_success() {
            //let response_json: serde_json::Value = response.json().await?;
            // Handle the response and return the appropriate result

            Ok("Issue deleted successfully".to_string())
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to delete issue: {}", error_message).into())
        }
    }

    async fn close_issue(&self, input: FetchIssue) -> FieldResult<String> {
        let mutation = r#"
        mutation CloseIssue($input: CloseIssueInput!) {
          closeIssue(input: $input) {
            clientMutationId
          }
        }
    "#;

        let variables = json!({
            "input" : {
                "issueId" : input.issue_id,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = response(&request_body).await;

        if response.status().is_success() {
            //let response_json: serde_json::Value = response.json().await?;
            // Handle the response and return the appropriate result
            Ok("Issue closed successfully".to_string())
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to close issue: {}", error_message).into())
        }
    }

    async fn add_label_to_issue(
        &self,
        input: FetchIssue,
        label_name: String,
    ) -> FieldResult<String> {
        let mutation = r#"
        mutation AddLabelToIssue($input: AddLabelsToLabelableInput!) {
            addLabelsToLabelable(input: $input) {
                labelable {
                    ... on Issue {
                        id
                        title
                        labels(first: 5) {
                            nodes {
                                name
                            }
                        }
                    }
                }
            }
        }
    "#;

        let labels = read_data("labels.json");
        let label_id = labels
            .iter()
            .find(|l| l.name == label_name)
            .map(|l| l.id.clone());

        if let Some(label_id) = label_id {
            let variables = json!({
                "input": {
                    "labelableId": input.issue_id,
                    "labelIds": [label_id],
                }
            });

            let request_body = json!({
                "query": mutation,
                "variables": variables,
            });

            let response = response(&request_body).await;

            if response.status().is_success() {
                Ok("Label added successfully".to_string())
            } else {
                let error_message = response.text().await?;
                Err(format!("Failed to add label to issue: {}", error_message).into())
            }
        } else {
            Err(format!("Label with name '{}' not found", label_name).into())
        }
    }

    async fn create_label(&self,repo_id:String,label_name:String,color:String) -> FieldResult<String> {
        let mutation = r#"
        mutation CreateLabel($input: CreateLabelInput!) {
            createLabel(input: $input) {
                clientMutationId
            }
        }
    "#;

        let variables = json!({
            "input": {
                "repositoryId": repo_id,
                "name": label_name,
                "color": color,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = response(&request_body).await;

        if response.status().is_success() {
            Ok("Label created successfully".to_string())
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to create label: {}", error_message).into())
        }
    }
}
