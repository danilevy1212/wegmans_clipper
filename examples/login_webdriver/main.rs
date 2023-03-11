// ## Setup
//
// This example assumes you have geckodriver or chromedriver listening at port 4444.
//
// You can start the webdriver instance by:
//
// ### geckodriver
//
// ```text
// docker compose up -d
// ```

use dotenv::dotenv;
use wegmans_coupons_client::webdriver_client::WebDriverClient;
use std::env;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Env setup
    dotenv().ok();

    // Get email and password
    let email = &env::var("WEGMANS_EMAIL")?;
    let password = &env::var("WEGMANS_PASSWORD")?;
    let web_driver_url = &env::var("WEB_DRIVER_HOST_URL")?;

    // Connect to webdriver server
    let client = WebDriverClient::new(web_driver_url).await?;

    // Login and return session cookie
    dbg!(client.get_session_cookie(email, password).await?);

    // Close the client
    client.close().await?;

    Ok(())
}
