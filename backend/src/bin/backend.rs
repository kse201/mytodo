#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use backend::db::{establish_connection, query_task};
use mytodo::JsonApiResponse;
use rocket_contrib::json::Json;

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: Vec::new() };

    let con = establish_connection();
    for task in query_task(&con) {
        response.data.push(task);
    }

    Json(response)
}

fn main() {
    rocket::ignite().mount("/", routes![tasks_get]).launch();
}
