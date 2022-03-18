#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::Form;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

#[derive(FromForm, Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool
}

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/createTodo", data = "<task>")]
fn createTodo(task: Form<Task>) -> Json<Task> {
    // format!("description, '{}', done is {}",task.description, task.done)
    let newTask: Task = Task {
        description: task.description.to_string(),
        done: task.done,
    };
    Json(newTask)
}


#[get("/getTodos")]
fn getTodos() -> Json<Vec<Task>> {

    let mut todoVec: Vec<Task> = Vec::new();

    let task1 = Task {
        description: "do something today".to_string(),
        done: false,
    };

    let task2 = Task {
        description: "did something yesterday".to_string(),
        done: true,
    };

    todoVec.push(task1);
    todoVec.push(task2);

    Json(todoVec)
}

fn main() {
    rocket::ignite()
    .mount("/", 
    routes![
        index,
        createTodo,
        getTodos
    ])
    .launch();
}
