use wegmans_coupons_client::http_client::Client;

use anyhow::Result;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cookie = env::var("WEGMANS_COOKIE")?;

    let wegmans_http_client = Client::new(&cookie)?;

    dbg!(wegmans_http_client.get_coupons().await?);

    Ok(())
}
