use axum::http::{Method, StatusCode};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use std::env;
use axum::routing::{post, get_service};
use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

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
        .route("/hello-world", get(handler))
        .route("/email", post(send_email))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST]),
        );

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn send_email() {
    let sender = env::var("SENDER_EMAIL").unwrap();
    let reciever = env::var("RECIEVER_EMAIL").unwrap();
    let smtp_username = env::var("SMTP_USERNAME").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();
    let smtp_provider = env::var("SMTP_PROVIDER").unwrap();

    let email = Message::builder()
        .from(format!("Sender <{sender}>").parse().unwrap())
        .to(format!("Receiver <{reciever}>").parse().unwrap())
        .subject("Sending email with Rust")
        .body(String::from("This is my first email"))
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&smtp_provider)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
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
    if let Err(_) = env::var("RECIEVER_EMAIL") {
        panic!("env var `RECIEVER_EMAIL` needs to be set")
    }
    if let Err(_) = env::var("SENDER_EMAIL") {
        panic!("env var `SENDER_EMAIL` needs to be set")
    }
}
