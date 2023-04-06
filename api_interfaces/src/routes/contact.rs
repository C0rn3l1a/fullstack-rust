use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ContactBody {
    pub cf_turnstile_token: String,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct ContactResponse {
    pub message: String
}
