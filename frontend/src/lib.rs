use mytodo::{JsonApiResponse, Task};
use seed::fetch;
use seed::{prelude::*, *};

/// Init
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!(url);
    orders.perform_cmd(async { Msg::FetchedTasks(fetch_drills().await) });
    Model {
        tasks: vec![],
        new_todo_title: String::new(),
    }
}

// Model

struct Model {
    tasks: Vec<Task>,
    new_todo_title: String,
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

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_header(&model.new_todo_title),
        IF!(not (model.tasks.is_empty() )
            => vec![
            view_main(model.tasks.to_owned()),
            view_footer(model.tasks.to_owned()),
        ])
    ]
}

fn view_header(new_todo_title: &str) -> Node<Msg> {
    header![C!["header"], h1!["todos"],]
}

fn view_main(tasks: Vec<Task>) -> Node<Msg> {
    // C!["main", view_toggle_all(), view_todo_list()]
    section![C!["main"], view_task_list(tasks)]
}

// fn view_toggle_all() -> Node<Msg> {}
fn view_task_list(tasks: Vec<Task>) -> Node<Msg> {
    ul![
        C!["task-list"],
        tasks.iter().map(|task| { li![div![label![&task.title,]]] })
    ]
}
fn view_footer(tasks: Vec<Task>) -> Node<Msg> {
    let completed_count = tasks.iter().filter(|task| task.is_completed()).count();
    let active_count = tasks.len() - completed_count;
    footer![
        C!["footer"],
        span![C!["task-count"], format!("iten {} left", active_count)],
        view_filters(),
    ]
}
fn view_filters() -> Node<Msg> {
    ul![C!["filters"]]
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
