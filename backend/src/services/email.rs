use lettre::transport::smtp::Error as LettreError;
use lettre::{
    Message, SmtpTransport, Transport, message::{header,SinglePart},
    transport::smtp::authentication::Credentials
};
use std::env;

pub async fn send_contact_email(email: String, name: String, phone: String, message: String) -> Result<(), LettreError> {
    let sender = env::var("SENDER_EMAIL").unwrap();
    let receiver = env::var("RECEIVER_EMAIL").unwrap();
    let smtp_username = env::var("SMTP_USERNAME").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();
    let smtp_provider = env::var("SMTP_PROVIDER").unwrap();

    let email_body = build_contact_email_body(email, name.clone(), phone, message);

    let email = Message::builder()
        .from(format!("Contact Form <{sender}>").parse().unwrap())
        .to(format!("Receiver <{receiver}>").parse().unwrap())
        .subject(format!("Contact from {}",name))
        .singlepart(SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(email_body))
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

fn build_contact_email_body(name: String, email: String, phone: String, message: String) -> String {
    format!(r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Hello from Lettre!</title>
    </head>
    <body>
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
    </body>
    </html>"#, name, email, phone, message)
}