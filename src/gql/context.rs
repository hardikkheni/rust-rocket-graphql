/// database context for juniper graphql
use crate::db::pool::PostgresPool;

pub struct GraphQLContext {
    pub pool: PostgresPool,
}

impl juniper::Context for GraphQLContext {}
