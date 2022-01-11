#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use backend::db::{establish_connection, query_task};
use mytodo::JsonApiResponse;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rocket_contrib::json::Json;

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: Vec::new() };

    let con = establish_connection();
    for task in query_task(&con) {
        let api_task = mytodo::Task{
            id: task.id,
            title: task.title,
            status: task.status
        };
        response.data.push(api_task);
    }

    Json(response)
}

fn main() ->Result<(), Error>{
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![tasks_get])
        .attach(cors)
        .launch();

    Ok(())
}
