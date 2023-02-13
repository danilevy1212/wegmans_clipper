use dotenv::dotenv;
use reqwest::{header, Client, Url};
use serde_json::json;
use std::env;

// NOTE  Change me!
const COUPON_ID: u32 = 6451;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut url = Url::parse(&env::var("WEGMANS_BASE_URL")?)?;
    let cookie = env::var("WEGMANS_COOKIE")?;

    let client_builder = Client::builder();
    let mut headers = header::HeaderMap::new();

    headers.append(header::COOKIE, cookie.parse()?);

    dbg!(&headers);

    url.set_path(format!("/api/v2/offers/{COUPON_ID}").as_str());

    let payload = json!({
        "clipped": true
    })
    .to_string();

    let req = client_builder
        .default_headers(headers)
        .build()?
        .put(url)
        // NOTE  content-type is need or I will get blocked by cloud-flare
        .header(header::CONTENT_TYPE, "application/json")
        .body(payload);

    dbg!(&req);

    let resp = req.send().await?.text().await?;

    dbg!(&resp);

    Ok(())
}
