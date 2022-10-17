#[macro_use]
extern crate diesel;
use diesel::{
    helper_types::{IntoBoxed, LeftJoin},
    pg::Pg,
    query_builder::QueryFragment,
    AppearsOnTable, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
mod schema;
use schema::*;
fn main() {
    // creating sample data
    let mut conn = PgConnection::establish("postgresql://postgres:123@localhost:5423/col")
        .unwrap_or_else(|_| panic!("Error connecting"));
    let new_size = InsertSize {
        value: String::from("size 1"),
    };
    let size = diesel::insert_into(sizes::table)
        .values(&new_size)
        .returning(sizes::id)
        .get_result::<i32>(&mut conn)
        .unwrap();

    let new_factory = InsertFactory {
        name: String::from("factory 1"),
    };
    let factory = diesel::insert_into(factories::table)
        .values(&new_factory)
        .returning(factories::id)
        .get_result::<i32>(&mut conn)
        .unwrap();

    let new_weight = InsertWeight { value: 10 };
    let weight = diesel::insert_into(weights::table)
        .values(&new_weight)
        .returning(weights::id)
        .get_result::<i32>(&mut conn)
        .unwrap();

    let new_product = InsertProduct {
        name: String::from("product 1"),
        name_mobile: Some(String::from("mobile 1")),
        factory_id: Some(factory),
        size_id: Some(size),
        weight_id: Some(weight),
    };
    let _product = diesel::insert_into(products::table)
        .values(&new_product)
        .returning(products::id)
        .get_result::<i32>(&mut conn)
        .unwrap();

    println!("creating sample data finished");

    // actual problem :

    let entry_col = products::name;

    let mut query = products::table
        .left_join(sizes::table)
        .left_join(weights::table)
        .left_join(factories::table)
        .into_boxed();

    let mut query = sort(query, entry_col);
}

fn sort<U: 'static>(query: BoxedQuery<'static>, column: U) -> BoxedQuery<'static>
where
    U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<BoxedTable> + Send,
{
    query.order_by(column.asc())
}

// models
#[derive(Debug, Insertable)]
#[diesel(table_name = products)]
struct InsertProduct {
    name: String,
    name_mobile: Option<String>,
    size_id: Option<i32>,
    weight_id: Option<i32>,
    factory_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sizes)]
struct InsertSize {
    value: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = weights)]
struct InsertWeight {
    value: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = factories)]
struct InsertFactory {
    name: String,
}

type BoxedTable =
    LeftJoin<LeftJoin<LeftJoin<products::table, sizes::table>, weights::table>, factories::table>;

type BoxedQuery<'a> = IntoBoxed<'a, BoxedTable, Pg>;
