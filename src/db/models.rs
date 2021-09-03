use chrono::NaiveDateTime;
use diesel::Queryable;
use juniper::graphql_object;

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
    pub fn quantity(&self) -> i32 {
        self.quantity
    }
}
