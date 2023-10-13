// src/main.rs

mod config;
mod functions;
mod handler;
mod schemas;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema, Context,
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use config::data_manager::DataManager;
use handler::graphql_handler::{Mutation, ProjectSchema, Query};
use rocket::{response::content, routes, State};

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(query: GraphQLQuery) -> GraphQLResponse {
    let schemas = Schema::new(Query, Mutation, EmptySubscription);
    query.execute(&schemas).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_mutation(
    request: GraphQLRequest,
) -> GraphQLResponse {
        let schemas = Schema::new(Query, Mutation, EmptySubscription);

    request.execute(&schemas).await
}
#[rocket::get("/")]
    async fn graphql_playground() -> content::RawHtml<String> {
        content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    }
// fn create_schema(ctx:&Context<'_>) -> Schema<Query, Mutation, EmptySubscription> {
//     let data = &ctx.data::<DataManager>().unwrap();
//     Schema::build(Query, Mutation, EmptySubscription)
//         .data(data)
//         .finish()
// }
// #[rocket::get("/")]
// async fn graphql_playground() -> content::RawHtml<String> {
//     content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
// }

#[rocket::launch]
fn rocket() -> _ {
    let data_manager = DataManager::new();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
    .data(data_manager).finish();
    let data_manager1 = DataManager::new();

    rocket::build()
        .manage(schema).manage(data_manager1)
        .mount("/", routes![graphql_query, graphql_mutation,graphql_playground])
}
