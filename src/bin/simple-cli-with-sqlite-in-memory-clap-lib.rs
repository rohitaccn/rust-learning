use sqlx::{prelude::*, sqlite::SqlitePool};
use std::env;
use tokio::{prelude::*, task};

#[derive(sqlx::FromRow)]
struct Todo {
    id: i32,
    name: String,
    completed: bool,
}

async fn create_todo(pool: &SqlitePool, name: &str, completed: bool) -> Result<Todo, sqlx::Error> {
    let todo = sqlx::query("INSERT INTO todos (name, completed) VALUES ($1, $2)")
        .bind(name)
        .bind(completed)
        .execute(pool)
        .await?;
    let id = todo.last_insert_rowid();
    sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

async fn get_todo(pool: &SqlitePool, id: i32) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(todo)
}

async fn update_todo(pool: &SqlitePool, id: i32, name: &str, completed: bool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE todos SET name = $1, completed = $2 WHERE id = $3")
        .bind(name)
        .bind(completed)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn get_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(pool)
        .await?;
    Ok(todos)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::builder().build(":memory:").await?;
    sqlx::query(
        r#"
    CREATE TABLE todos (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        completed BOOLEAN NOT NULL
    )"#,
    )
    .execute(&pool)
    .await?;

    // Create a new todo
    let new_todo = task::spawn(create_todo(&pool, "Buy milk", false))
        .await??;
    println!("Created new todo: {:?}", new_todo);

    // Get a todo by id
    let todo = task::spawn(get_todo(&pool, new_todo.id)).await??;
    match todo {
        Some(t) => println!("Retrieved todo: {:?}", t),
        None => println!("No todo found with id {}", new_todo.id),
    }

    // Update a todo
    task::spawn(update_todo(&pool, new_todo.id, "Buy eggs", true)).await??;
    let updated_todo = task::spawn(get_todo(&pool, new_todo.id)).await??.unwrap();
    println!("Updated todo: {:?}", updated_todo);

    // Get all todos
    let todos = task::spawn(get_todos(&pool)).await??;
    println!("Retrieved all todos: {:?}", todos);

    Ok(())
}


use clap::{App, Arg, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("todo-cli")
        .version("1.0")
        .author("Your Name")
        .about("Manage a list of todos")
        .arg(Arg::with_name("name")
            .help("Name of the todo")
            .index(1)
            .required(true))
        .arg(Arg::with_name("completed")
            .help("Whether the todo is completed or not")
            .index(2)
            .required(false))
        .arg(Arg::with_name("id")
            .help("ID of the todo")
            .index(1)
            .required(false))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new todo")
                .arg(Arg::with_name("name").required(true))
                .arg(Arg::with_name("completed").required(false)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get a todo by ID")
                .arg(Arg::with_name("id").required(true)),
        )
        .subcommand(
            SubCommand::with_name("update")
               


/* 
use clap::{App, Arg, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("todo-cli")
        .version("1.0")
        .author("Your Name")
        .about("Manage a list of todos")
        .arg(Arg::with_name("name")
            .help("Name of the todo")
            .index(1)
            .required(true))
        .arg(Arg::with_name("completed")
            .help("Whether the todo is completed or not")
            .index(2)
            .required(false))
        .arg(Arg::with_name("id")
            .help("ID of the todo")
            .index(1)
            .required(false))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new todo")
                .arg(Arg::with_name("name").required

*/

