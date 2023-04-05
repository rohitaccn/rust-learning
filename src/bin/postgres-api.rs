use actix_web::{web, App, HttpResponse, HttpServer, Method, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use diesel::{prelude::*, r2d2::ConnectionManager, PgConnection};

// Database connection pool
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct CreateTodo {
    name: String,
    completed: bool,
}

#[derive(Serialize)]
struct Todo {
    id: i32,
    name: String,
    completed: bool,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    data: Option<Todo>,
}

//establish connection
fn establish_connection() -> Pool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

//handling GET request
fn get_handler(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let todos = diesel::sql_query("SELECT * FROM todos")
        .load::<Todo>(&conn)
        .expect("Error loading todos");
    HttpResponse::Ok().json(ApiResponse {
        message: "GET request received.".to_string(),
        data: Some(todos),
    })
}

//handling POST request
fn post_handler(data: web::Json<CreateTodo>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // Validate the request data
    if data.name.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse {
            message: "Name is missing".to_string(),
            data: None,
        });
    }

    let todo = Todo {
        id: 0,
        name: data.name.to_string(),
        completed: data.completed,
    };

    // Insert the todo into the database
    let insert_query = diesel::insert_into(schema::todos::table)
        .values(&todo)
        .returning(schema::todos::id);
    let inserted_todo = insert_query.get_result::<Todo>(&conn).expect("Error saving new todo");
    HttpResponse::Ok().json(ApiResponse {
        message: "POST request received.".to_string(),
        data: Some(inserted_todo),
    })
}

