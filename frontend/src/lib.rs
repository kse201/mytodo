use mytodo::{JsonApiResponse, Task};
use seed::fetch;
use seed::{prelude::*, *};

/// Init
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!(url);
    orders.perform_cmd(async { Msg::FetchedTasks(fetch_drills().await) });
    Model { tasks: vec![] }
}

// Model

struct Model {
    tasks: Vec<Task>,
}

// Msg

enum Msg {
    FetchedTasks(fetch::Result<JsonApiResponse>),
}

// Update

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        }
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("error fetching: {:?}", reason));
        }
    }
}

// View

fn view(model: &Model) -> impl View<Msg> {
    let tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .map(|t| li![{ t.title.clone() }])
        .collect();

    h1![{ "tasks" }, ul![tasks,],]
}

async fn fetch_drills() -> fetch::Result<JsonApiResponse> {
    fetch("http://localhost:8000/tasks/")
        .await?
        .check_status()?
        .json()
        .await
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    let root_element = document()
        .get_elements_by_class_name("taskapp")
        .item(0)
        .expect("element with the class taskapp");
    App::start(root_element, init, update, view);
}
