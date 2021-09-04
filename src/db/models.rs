use super::schema::products;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use juniper::{graphql_object, GraphQLInputObject};

#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub product_id: String,
    pub garment_style: String,
    pub garment_color: String,
    pub garment_size: String,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[graphql_object]
impl Product {
    fn id(&self) -> i32 {
        self.id
    }
    #[graphql(name = "product_id")]
    pub fn product_id(&self) -> &str {
        self.product_id.as_str()
    }
    #[graphql(name = "garment_style")]
    pub fn garment_style(&self) -> &str {
        self.garment_style.as_str()
    }
    #[graphql(name = "garment_color")]
    pub fn garment_color(&self) -> &str {
        self.garment_style.as_str()
    }
    #[graphql(name = "garment_size")]
    pub fn garment_size(&self) -> &str {
        self.garment_size.as_str()
    }
    pub fn quantity(&self) -> i32 {
        self.quantity
    }
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "products"]
pub struct InsertProductInput {
    #[graphql(name = "product_id")]
    pub product_id: String,
    #[graphql(name = "garment_style")]
    pub garment_style: String,
    #[graphql(name = "garment_color")]
    pub garment_color: String,
    #[graphql(name = "garment_size")]
    pub garment_size: String,
}
