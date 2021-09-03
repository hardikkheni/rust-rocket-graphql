table! {
    products (id) {
        id -> Int4,
        product_id -> Varchar,
        garment_style -> Varchar,
        garment_color -> Varchar,
        garment_size -> Varchar,
        quantity -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}
