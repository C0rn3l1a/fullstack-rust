use std::env;
use serde::{Deserialize, Serialize};
use reqwest::Client as ReqwestClient;
use tracing::{warn, debug};

#[derive(Deserialize, Serialize, Debug)]
pub struct CloudflareSiteVerifyBody {
    secret: String,
    response: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CloudflareSiteVerifyResponse {
    success: bool,
    challenge_ts: Option<String>,
    hostname: Option<String>,
    error_codes: Option<Vec<String>>,
    action: Option<String>,
    cdata: Option<String>,
  }

pub async fn cloudflare_site_check(cf_turnstile_token: String) -> Result<bool, reqwest::Error> {
    let secret = env::var("CLOUDFLARE_SECRET_KEY").unwrap();
    let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
    let body = CloudflareSiteVerifyBody {
        secret,
        response: cf_turnstile_token
    };
    
    debug!("Site Verify Request: {:?}",body);
    
    let client = ReqwestClient::builder().use_rustls_tls().build()?;
    let response = client.post(url).json(&body).send().await?;
    let site_verify: CloudflareSiteVerifyResponse  = response.json().await?;
    
    debug!("Site Verify Response: {:?}",body);
    
    if site_verify.success != true {
        warn!("Site Verify Error: {:?}",site_verify);
    }

    Ok(site_verify.success)
}