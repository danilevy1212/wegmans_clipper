use dotenv::dotenv;
use reqwest::{header, Client, Url};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut url = Url::parse(&env::var("WEGMANS_BASE_URL")?)?;
    let cookie = env::var("WEGMANS_COOKIE")?;

    let client_builder = Client::builder();
    let mut headers = header::HeaderMap::new();

    headers.append(header::COOKIE, cookie.parse()?);
    url.set_path("api/v2/offers");
    url.set_query(Some("type=coupon"));

    dbg!(&url);

    let resp = client_builder
        .default_headers(headers)
        .build()?
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    dbg!(&resp);

    Ok(())
}
