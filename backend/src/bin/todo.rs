use backend::db::*;
use std::env;

fn help() {
    println!("subcommands:");
    println!("   new<title> : create a new task");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "delete" => delete_task(&args[2..]),
        "done" => done_task(&args[2..]),
        _ => help(),
    }
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n----");
    for task in query_task(&conn) {
        println!("{} | {} | {}", task.id, task.title, task.status);
    }
}

fn delete_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    let id = args[0].parse().expect("Unexpected arg");
    remove_task(&conn, id);
}

fn done_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    let id = args[0].parse().expect("Unexpected arg");
    complete_task(&conn, id);
}
