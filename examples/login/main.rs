//! ## Setup
//!
//! This example assumes you have geckodriver or chromedriver listening at port 4444.
//!
//! You can start the webdriver instance by:
//!
//! ### geckodriver
//!
//! ```text
//! docker compose up -d
//! ```

use dotenv::dotenv;
use fantoccini::{ClientBuilder, Locator::Css};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Env setup
    dotenv().ok();

    // Get email and password
    let email = env::var("WEGMANS_EMAIL")?;
    let password = env::var("WEGMANS_PASSWORD")?;

    // Connect to webdriver instance that is listening on port 4444
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    // Go to the login website.
    client.goto("https://shop.wegmans.com/login").await?;

    // Wait to be redirected
    client.wait().for_element(Css("#next")).await?;

    // Enter email, password and submit
    client
        .form(Css("#localAccountForm"))
        .await?
        .set(Css("#signInName"), &email)
        .await?
        .set(Css("#password"), &password)
        .await?
        .submit_with(Css("#next"))
        .await?;

    // Wait to be redirected again
    client
        .wait()
        .for_element(Css("#sticky-react-header"))
        .await?;

    // Make sure to have the most recent cookie, 10 seconds is overkill but that is OK
    sleep(Duration::from_secs(10)).await;

    println!(
        "The session cookie: {:#?}",
        client.get_named_cookie("session-prd-weg").await?.value()
    );

    client.close().await?;

    Ok(())
}
