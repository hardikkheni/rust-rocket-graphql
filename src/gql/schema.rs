use super::context::GraphQLContext;
use crate::db::{
    models::{InsertProductInput, Product},
    schema::products,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::{graphql_object, EmptySubscription, FieldError, FieldResult, RootNode};

pub struct Query;

#[graphql_object(context=GraphQLContext)]
impl Query {
    pub fn products(#[graphql(content)] ctx: &GraphQLContext) -> FieldResult<Vec<Product>> {
        let conn: &PgConnection = &ctx.pool.get().unwrap();
        let res = products::dsl::products.load::<Product>(conn);
        graphql_translate(res)
    }
}

pub struct Mutation;

#[graphql_object(context=GraphQLContext)]
impl Mutation {
    #[graphql(name = "insertProduct")]
    pub fn insert_product(ctx: &GraphQLContext, input: InsertProductInput) -> FieldResult<Product> {
        let conn: &PgConnection = &ctx.pool.get().unwrap();
        let res = diesel::insert_into(products::table)
            .values(&input)
            .get_result(conn);
        graphql_translate(res)
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}
