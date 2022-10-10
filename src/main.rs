use crate::models::Item;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod models;
mod schema;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn upsert_item(conn: &mut PgConnection, name: &str) -> Option<Item> {
    use self::models::NewItem;
    use self::schema::items::dsl;

    let query = diesel::insert_into(dsl::items)
        .values(NewItem { name, time_deleted: None })
        .on_conflict(dsl::name)
        .filter_target(dsl::time_deleted.is_null())
        .do_nothing()
        .returning(Item::as_returning());

    let sql = debug_query::<Pg, _>(&query).to_string();
    println!("query to upsert {name}: {sql}");

    match query.get_result(conn) {
        Ok(item) => Some(item),
        Err(diesel::result::Error::NotFound) => None,
        Err(err) => panic!("failed to upsert {name}: {err}"),
    }
}

fn main() {
    let mut conn = establish_connection();

    for name in ["one", "two"] {
        let item = upsert_item(&mut conn, name);
        println!("result of upserting {name}: {item:?}");
    }

}
