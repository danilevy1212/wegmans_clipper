use wegmans_coupons_client::http_client::coupon_client::CouponClient;

use anyhow::Result;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cookie = env::var("WEGMANS_COOKIE")?;

    let wegmans_http_client = CouponClient::new(&cookie)?;

    dbg!(wegmans_http_client.get_coupons().await?);

    Ok(())
}
