use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContactBody {
    pub cf_turnstile_token: String,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContactResponse {
    pub message: String
}

impl PartialEq for ContactBody {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email
        && self.name == other.name
        && self.phone == other.phone
        && self.message == other.message
        && self.cf_turnstile_token == other.cf_turnstile_token
    }
}