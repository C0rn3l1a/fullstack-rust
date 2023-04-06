use axum::{
    Router, http::{StatusCode},
    routing::{post, get_service},
};
use std::{net::SocketAddr, env};
use tower_http::{cors::{CorsLayer}, services::ServeDir};
use dotenv::dotenv;

mod services;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    check_environment();

    // build our application with a route
    let app = Router::new()
        // this line here serves the yew app
        .nest_service("/", 
            get_service(ServeDir::new("dist")).handle_error(|_| async { (StatusCode::NOT_FOUND, "File not found") }),
        )
        .route("/contact", post(routes::contact::contact))
        .layer(CorsLayer::permissive());

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn check_environment() {
    if let Err(_) = env::var("SMTP_USERNAME") {
        panic!("env var `SMTP_USERNAME` needs to be set")
    }
    if let Err(_) = env::var("SMTP_PASSWORD") {
        panic!("env var `SMTP_PASSWORD` needs to be set")
    }
    if let Err(_) = env::var("SMTP_PROVIDER") {
        panic!("env var `SMTP_PROVIDER` needs to be set")
    }
    if let Err(_) = env::var("RECEIVER_EMAIL") {
        panic!("env var `RECEIVER_EMAIL` needs to be set")
    }
    if let Err(_) = env::var("SENDER_EMAIL") {
        panic!("env var `SENDER_EMAIL` needs to be set")
    }
}
