use crate::constants::{WEGMANS_BASE_URL, WEGMANS_SESSION_COOKIE_NAME};

use anyhow::{Context, Result};
use fantoccini::{Client, ClientBuilder, Locator::Css};
use reqwest::Url;
use std::time::Duration;
use tokio::time::sleep;

pub struct WebDriverClient(Client);

impl WebDriverClient {
    pub async fn new(url: &str) -> Result<WebDriverClient> {
        Ok(WebDriverClient(
            ClientBuilder::native()
                .connect(url)
                .await
                .with_context(|| format!("Could not connect with url={url}"))?,
        ))
    }

    pub async fn get_session_cookie(&self, email: &str, password: &str) -> Result<String> {
        let client = &self.0;
        let url = Url::parse(WEGMANS_BASE_URL)?.join("/login")?.to_string();

        // Go to the login website.
        client.goto(&url).await?;

        // Wait to be redirected
        client.wait().for_element(Css("#next")).await?;

        // Enter email, password and submit
        client
            .form(Css("#localAccountForm"))
            .await?
            .set(Css("#signInName"), email)
            .await?
            .set(Css("#password"), password)
            .await?
            .submit_with(Css("#next"))
            .await?;

        // Wait to be redirected again
        client
            .wait()
            .for_element(Css("#sticky-react-header"))
            .await?;

        // Make sure to have the most recent cookie, 10 seconds is overkill but that is OK
        sleep(Duration::from_secs(5)).await;

        let cookie = client.get_named_cookie(WEGMANS_SESSION_COOKIE_NAME).await?;
        let (cookie_name, cookie_value) = cookie.name_value();

        Ok(format!("{cookie_name}={cookie_value}"))
    }

    pub async fn close(self) -> Result<()> {
        self.0
            .close()
            .await
            .with_context(|| "Error while closing connection")
    }
}
