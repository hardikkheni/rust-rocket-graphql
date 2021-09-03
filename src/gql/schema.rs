use super::context::GraphQLContext;
use crate::db::{models::Product, schema::products::dsl::*};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldError, FieldResult, RootNode,
};

pub struct Query;

#[graphql_object(context=GraphQLContext)]
impl Query {
    pub fn products(#[graphql(content)] ctx: &GraphQLContext) -> FieldResult<Vec<Product>> {
        let conn: &PgConnection = &ctx.pool.get().unwrap();
        let res = products.load::<Product>(conn);
        graphql_translate(res)
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}
