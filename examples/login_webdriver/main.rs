// ## Setup
//
// This example assumes you have geckodriver or chromedriver listening at port 4444.
//
// You can start the webdriver instance by:
//
// ### geckodriver
//
// ```text
// docker compose up geckodriver-dev -d
// ```

use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::process::Stdio;
use tokio::process::Command;
use wegmans_coupons_client::webdriver_client::WebDriverClient;

const WEBDRIVER_HOST: &str = "127.0.0.1";
const WEBDRIVER_PORT: u16 = 4444;

#[tokio::main]
async fn main() -> Result<()> {
    // Env setup
    dotenv().ok();

    // Get email and password
    let email = &env::var("WEGMANS_EMAIL")?;
    let password = &env::var("WEGMANS_PASSWORD")?;

    // Session
    let _ = Command::new("geckodriver")
        .kill_on_drop(true)
        .stdout(Stdio::null())
        .spawn()?;

    // Connect to webdriver server
    let client = WebDriverClient::new(&format!("http://{WEBDRIVER_HOST}:{WEBDRIVER_PORT}")).await?;

    // Login and return session cookie
    dbg!(client.get_session_cookie(email, password).await?);

    // Close the client
    client.close().await?;

    Ok(())
}
