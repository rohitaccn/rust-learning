use tokio::prelude::*;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn handle_get(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    // Handle the GET request here
    Ok(Response::new(Body::empty()))
}

async fn handle_post(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    // Handle the POST request here
    Ok(Response::new(Body::empty()))
}

async fn handle_connection(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => handle_get(req).await,
        (&hyper::Method::POST, "/") => handle_post(req).await,
        _ => Ok(Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr)?;
    println!("Server running on {}", addr);

    let service = make_service_fn(|_| async {
        Ok::<_, Box<dyn std::error::Error>>(service_fn(handle_connection))
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
