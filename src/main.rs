#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate juniper;

mod db;
mod gql;

use gql::{
    context::GraphQLContext,
    schema::{Mutation, Query, Schema},
};
use juniper::EmptySubscription;
use rocket::{response::content, Rocket, State};

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<GraphQLContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[launch]
fn rocket() -> _ {
    let pool = db::pool::get_pool();
    Rocket::build()
        .manage(GraphQLContext { pool })
        .manage(Schema::new(
            Query,
            Mutation,
            EmptySubscription::<GraphQLContext>::new(),
        ))
        .mount("/", routes![graphiql, post_graphql_handler])
}
