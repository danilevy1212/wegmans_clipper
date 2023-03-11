use wegmans_coupons_client::http_client::Client;

use anyhow::Result;
use dotenv::dotenv;
use std::env;

// NOTE  Change me!
const COUPON_ID: &str = "6451";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cookie = env::var("WEGMANS_COOKIE")?;

    let client = Client::new(&cookie)?;

    dbg!(client.clip_coupon(COUPON_ID).await?);

    Ok(())
}
