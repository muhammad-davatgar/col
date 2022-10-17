// @generated automatically by Diesel CLI.

diesel::table! {
    factories (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        name_mobile -> Nullable<Varchar>,
        size_id -> Nullable<Int4>,
        weight_id -> Nullable<Int4>,
        factory_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sizes (id) {
        id -> Int4,
        value -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    weights (id) {
        id -> Int4,
        value -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(products -> factories (factory_id));
diesel::joinable!(products -> sizes (size_id));
diesel::joinable!(products -> weights (weight_id));

diesel::allow_tables_to_appear_in_same_query!(
    factories,
    products,
    sizes,
    weights,
);
