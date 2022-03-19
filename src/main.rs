#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::FromForm;
use rocket::request::Form;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;
extern crate dotenv;

use diesel::{prelude::*, Queryable};
use diesel::mysql::MysqlConnection;

use dotenv::dotenv;
use std::env;
use schema::tasks;

mod schema;


#[derive(FromForm, 
    Serialize, 
    Deserialize,
    Queryable,
    Clone, 
    Default, 
    QueryId, 
    SqlType
)]
struct Task {
    id: i32,
    description: String,
    done: i32
}

#[derive(Insertable, FromForm)]
#[table_name="tasks"]
struct iTask {
    description: String,
    done: i32
}



#[derive(Insertable, FromForm, Clone)]
#[table_name="tasks"]
struct dTask {
    id: i32,
    description: String,
    done: i32
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[get("/")]
fn index() -> &'static str {
    establish_connection();
    "Hello, world"
}

#[post("/createTodo", data = "<task>")]
fn createTodo(task: Form<iTask>) -> String {

    let connection = establish_connection();
    let new_task = iTask {
        description: task.description.to_string(),
        done: task.done,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&connection)
        .expect("Error saving new post");

        format!("ok")
}


#[get("/getTodos")]
fn getTodos() -> Json<Vec<Task>> {

    // let mut todoVec: Vec<Task> = Vec::new();

    // let task1 = Task {
    //     description: "do something today".to_string(),
    //     done: false,
    // };

    // let task2 = Task {
    //     description: "did something yesterday".to_string(),
    //     done: true,
    // };

    // todoVec.push(task1);
    // todoVec.push(task2);

    // Json(todoVec)

        // use mod schema;
    //  use schema::task::dsl::*;
    // use rustrocket::schema::tasks::dsl::*;`
    use schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks
    // .select(Task.description)
    // .filter("")
    // .limit(2)
    .load::<Task>(&connection)
    .expect("Error loading tasks");

    Json(results)
    
}

#[post("/deleteTodo/<task_id>")]
fn deleteTodo(task_id: i32) -> String {

    use schema::tasks::dsl::*;

    let connection = establish_connection();
    let num_deleted = diesel::delete(tasks.filter(id.eq(task_id)))
        .execute(&connection)
        .expect("Error deleting tasks");

        format!("ok")
}

#[post("/updateTodo"/*,format = 
"application/x-www-form-urlencoded"*/, data = "<task>")]
fn updateTodo(task: Form<dTask>) -> String {

    use schema::tasks::dsl::*;
    let connection = establish_connection();

    diesel::update(tasks.filter(id.eq(task.id)))
    .set((
        description.eq(task.description.clone()), 
        done.eq(task.done)
    ))
    .execute(&connection);

    format!("ok")
}


fn main() {
    rocket::ignite()
    .mount("/", 
    routes![
        index,
        createTodo,
        getTodos,
        deleteTodo,
        updateTodo
    ])
    .launch();
}
