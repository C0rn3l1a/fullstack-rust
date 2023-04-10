
use api_interfaces::routes::contact::{ContactBody, ContactResponse};
use axum::Json;
use reqwest::StatusCode;
use tracing::error;

use crate::services::{email::send_contact_email, cf_site_verify::cloudflare_site_check};

pub async fn contact(Json(payload): Json<ContactBody>,) -> (StatusCode, Json<ContactResponse>) {
    let site_verify = cloudflare_site_check(payload.cf_turnstile_token).await;
    match site_verify {
        Ok(verified) => {
            if verified == false {
                return (StatusCode::BAD_REQUEST, Json(ContactResponse {message: String::from("You are a robot!!")}));
            }
        },
        Err(error) => {
            // TODO: Error handle
            error!("Error with cf site verification: {:?}",error);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ContactResponse {message: String::from("Something went wrong")}));
        }
    }

    // from here on we know this was not a bot
    match send_contact_email(payload.email, payload.name, payload.phone, payload.message).await {
        Ok(_) => (StatusCode::OK, Json(ContactResponse {message: String::from("Contact email sent successfully")})),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ContactResponse {message: String::from("There was a problem sending the email")}))
    }

}