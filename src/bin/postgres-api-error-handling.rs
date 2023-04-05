use actix_web::{web, App, HttpResponse, HttpServer, Method, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    result::Error as DieselError,
    PgConnection,
};
use std::error::Error;

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
    error: Option<String>,
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

fn get_handler(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    match diesel::sql_query("SELECT * FROM todos").load::<Todo>(&conn) {
        Ok(todos) => HttpResponse::Ok().json(ApiResponse {
            message: "GET request received.".to_string(),
            data: Some(todos),
            error: None,
        }),
        Err(e) => {
            let message = format!("Error loading todos: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                message: "Error occured.".to_string(),
                data: None,
                error: Some(message),
            })
        }
    }
}

fn post_handler(data: web::Json<CreateTodo>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    if data.name.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse {
            message: "Name is missing".to_string(),
            data: None,
            error: None,
        });
    }

    let todo = Todo {
        id: 0,
        name: data.name.to_string(),
        completed: data.completed,
    };

    match diesel::insert_into(schema::todos::table)
        .values(&todo)
        .returning(schema::todos::id)
        .get_result::<Todo>(&conn)
    {
        Ok(inserted_todo) => HttpResponse::Ok().json(ApiResponse {
            message: "POST request received.".to_string(),
            data: Some(inserted_todo),
            error: None,
        }),
        Err(e) => {
            let message = format!("Error saving new todo: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                message: "Error occured.".to_string(),
                data: None,
                error: Some(message),
            })
        }
    }
}
fn put_handler(id: web::Path<i32>, data: web::Json<Todo>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    match diesel::update(schema::todos::table.find(id.into_inner()))
        .set(&data.into_inner())
        .execute(&conn)
    {
        Ok(updated) if updated > 0 => HttpResponse::Ok().json(ApiResponse {
            message: "PUT request received.".to_string(),
            data: Some(data.into_inner()),
            error: None,
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse {
            message: "Not Found".to_string(),
            data: None,
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            message: "Error occured.".to_string(),
            data: None,
            error: Some(format!("Error updating todo: {}", e)),
        }),
    }
}
fn delete_handler(id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    match diesel::delete(schema::todos::table.find(id.into_inner()))
        .execute(&conn)
    {
        Ok(deleted) if deleted > 0 => HttpResponse::Ok().json(ApiResponse {
            message: "DELETE request received.".to_string(),
            data: None,
            error: None,
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse {
            message: "Not Found".to_string(),
            data: None,
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            message: "Error occured.".to_string(),
            data: None,
            error: Some(format!("Error deleting todo: {}", e)),
        }),
    }
}

fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/todos").route(web::get().to(get_handler)))
            .service(web::resource("/todo").route(web::post().to(post_handler)))
            .service(web::resource("/todo/{id}").route(web::put().to(put_handler)))
            .service(web::resource("/todo/{id}").route(web::delete().to(delete_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}


