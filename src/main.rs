#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;

use juniper::{Context, EmptyMutation, EmptySubscription, RootNode};
use rocket::{response::content, Rocket, State};

#[derive(Clone)]
struct Episode {
    id: i32,
}

#[graphql_object]
impl Episode {
    fn id(&self) -> i32 {
        self.id
    }
}

struct Query;

#[derive(Default, Clone)]
struct Database {
    episodes: Vec<Episode>,
}

impl Context for Database {}

impl Database {
    fn new() -> Self {
        Database {
            episodes: vec![Episode { id: 1 }, Episode { id: 2 }],
        }
    }
}

#[graphql_object(context=Database)]
impl Query {
    fn episodes(#[graphql(content)] database: &Database) -> Vec<Episode> {
        database.episodes.to_vec()
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[launch]
fn rocket() -> _ {
    Rocket::build()
        .manage(Database::new())
        .manage(Schema::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        ))
        .mount("/", routes![graphiql, post_graphql_handler])
}
