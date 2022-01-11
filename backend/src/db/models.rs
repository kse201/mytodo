use super::schema::task;
use serde::Serialize;

#[derive(Debug, Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub status: String,
}
