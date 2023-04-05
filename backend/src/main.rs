use axum::Json;
use axum::http::{Method, StatusCode};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use std::env;
use std::error::Error;
use axum::routing::{post, get_service};
use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

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
        .route("/verify", post(verify))
        .layer(CorsLayer::permissive());

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

#[derive(Deserialize, Serialize)]
pub struct VerifyBody {
    cf_turnstile_token: String,
    email: String,
    name: String,
    phone: String,
    message: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyResponse {
    message: String
}

async fn send_contact_email(email: String, name: String, phone: String, message: String) -> Result<(), lettre::transport::smtp::Error> {
    let sender = env::var("SENDER_EMAIL").unwrap();
    let reciever = env::var("RECIEVER_EMAIL").unwrap();
    let smtp_username = env::var("SMTP_USERNAME").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();
    let smtp_provider = env::var("SMTP_PROVIDER").unwrap();

    let email_body = format!(r#"
        <h2>Contact Information</h2>
        <hr/>
        <p>
            <b>Name:</b> {}
        </p>
        <p>
            <b>Email:</b> {}
        </p>
        <p>
            <b>Phone:</b> {}
        </p>
    
        <h2>Message</h2>
        <hr/>
        <p>
            {}
        </p>
    "#, name, email, phone, message);

    let email = Message::builder()
        .from(format!("Sender <{sender}>").parse().unwrap())
        .to(format!("Receiver <{reciever}>").parse().unwrap())
        .subject(format!("Contact from {}",name))
        .body(email_body)
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&smtp_provider)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(error) => {
            // TODO: Error handle
            println!("Error with smtp transport: {:?}",error);
            Err(error)
        },
    }
}

async fn verify(Json(payload): Json<VerifyBody>,) -> (StatusCode, Json<VerifyResponse>) {
    let site_verify = cloudfare_site_check(payload.cf_turnstile_token).await;
    match site_verify {
        Ok(verified) => {
            if verified == false {
                return (StatusCode::BAD_REQUEST, Json(VerifyResponse {message: String::from("You are a robot!!")}));
            }
        },
        Err(error) => {
            // TODO: Error handle
            println!("Error with cf site verification: {:?}",error);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(VerifyResponse {message: String::from("Something went wrong")}));
        }
    }

    // from here on we know this was not a bot
    match send_contact_email(payload.email, payload.name, payload.phone, payload.message).await {
        Ok(_) => (StatusCode::OK, Json(VerifyResponse {message: String::from("Contact email sent successfully")})),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(VerifyResponse {message: String::from("There was a problem sending the email")}))
    }

}



#[derive(Deserialize, Serialize)]
pub struct CloudFareSiteVerifyBody {
    secret: String,
    response: String,
}

#[derive(Deserialize, Serialize)]
pub struct CloudFareSiteVerifyResponse {
    success: bool,
    challenge_ts: Option<String>,
    hostname: Option<String>,
    error_codes: Option<Vec<String>>,
    action: Option<String>,
    cdata: Option<String>,
  }

async fn cloudfare_site_check(cf_turnstile_token: String) -> Result<bool, reqwest::Error> {
    let secret = env::var("CLOUDFARE_SECRET_KEY").unwrap();
    let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
    let body = CloudFareSiteVerifyBody {
        secret,
        response: cf_turnstile_token
    };
    let client = reqwest::Client::new();
    let response = client.post(url).json(&body).send().await?;
    let site_verify: CloudFareSiteVerifyResponse  = response.json().await?;
    
    Ok(site_verify.success)
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
