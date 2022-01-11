use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connection to {}", db))
}

pub fn create_task(con: &SqliteConnection, title: &str) {
    let task = models::NewTask { title };
    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(con)
        .expect("Error inserting new task");
}

pub fn query_task(con: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table.load(con).expect("Error loading tasks")
}

pub fn complete_task(con: &SqliteConnection, id: i32) {
    use schema::task;
    diesel::update(task::table.find(id))
        .set(task::dsl::status.eq("done"))
        .execute(con)
        .expect("Error done task");
}

pub fn remove_task(con: &SqliteConnection, id: i32) {
    use schema::task;
    diesel::delete(task::table.find(id))
        .execute(con)
        .expect("Error deleteing task");
}
