use std::env;
use serde::{Deserialize, Serialize};
use reqwest::Client as ReqwestClient;

#[derive(Deserialize, Serialize)]
pub struct CloudFareSiteVerifyBody {
    secret: String,
    response: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CloudFareSiteVerifyResponse {
    success: bool,
    challenge_ts: Option<String>,
    hostname: Option<String>,
    error_codes: Option<Vec<String>>,
    action: Option<String>,
    cdata: Option<String>,
  }

pub async fn cloudfare_site_check(cf_turnstile_token: String) -> Result<bool, reqwest::Error> {
    let secret = env::var("CLOUDFARE_SECRET_KEY").unwrap();
    let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
    let body = CloudFareSiteVerifyBody {
        secret,
        response: cf_turnstile_token
    };
    let client = ReqwestClient::builder().use_rustls_tls().build()?;
    let response = client.post(url).json(&body).send().await?;
    let site_verify: CloudFareSiteVerifyResponse  = response.json().await?;
    if site_verify.success != true {
        println!("Site Verify Error: {:?}",site_verify);
    }

    Ok(site_verify.success)
}