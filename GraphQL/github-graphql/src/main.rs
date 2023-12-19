use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    FieldResult, InputObject, Object, Schema, SimpleObject,
};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use graphql_handler::{query::Query, mutation::MyMutation};
use reqwest::{self, Response};
use rocket::{post, response::content, routes, State};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
mod graphql_handler;
mod schema;
use std::env;
use dotenv::dotenv;

pub async fn response(body:&Value) -> Response{
    dotenv().ok();
    let token = env::var("TOKEN").unwrap();

    let response = reqwest::Client::new()
            .post("https://api.github.com/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "graphql-rust-client")
            .json(&body)
            .send()
            .await.unwrap();
    response
}


#[rocket::get("/")]
pub async fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[rocket::main]
async fn main() {
    let schema = Schema::build(Query, MyMutation, async_graphql::EmptySubscription).finish();

    rocket::build()
        .mount("/", routes![graphql_handler_main, graphql_playground])
        .manage(schema)
        .launch()
        .await
        .expect("Rocket failed to launch");
}

#[post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_handler_main(
    request: GraphQLRequest,
    schema: &State<Schema<Query, MyMutation, async_graphql::EmptySubscription>>,
) -> GraphQLResponse {
    request.execute(schema.inner()).await.into()
}

// pub fn update_data(data: &Issue) {
//     let mut all_data = read_data().unwrap();
//     all_data.push(data.clone());
//     let json = serde_json::to_string_pretty(&all_data).unwrap();
//     let mut file =
//         fs::File::create("he.json").expect("Failed to create or open the file for writing");

//     if file.write_all(json.as_bytes()).is_ok() {
//         println!("Data updated successfully in json.");
//     }
// }

// pub fn read_data() -> Result<Vec<Issue>, Error> {
//     let data = fs::read_to_string("he.json").unwrap();
//     let issues = serde_json::from_str(&data).unwrap();
//     Ok(issues)
// }