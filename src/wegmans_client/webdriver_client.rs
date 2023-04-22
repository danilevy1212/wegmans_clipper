use crate::constants::{WEGMANS_BASE_URL, WEGMANS_SESSION_COOKIE_NAME};

use anyhow::{Context, Result};
use fantoccini::{
    Client, ClientBuilder,
    Locator::{Css, XPath},
};
use log::info;
use reqwest::Url;

pub struct WebDriverClient(Client);

impl WebDriverClient {
    pub async fn new(url: &str) -> Result<WebDriverClient> {
        Ok(WebDriverClient(
            ClientBuilder::native()
                .connect(url)
                .await
                .context(format!("Failed to connect with url=\"{url}\""))?,
        ))
    }

    pub async fn get_session_cookie(&self, email: &str, password: &str) -> Result<String> {
        let client = &self.0;
        let url = Url::parse(WEGMANS_BASE_URL)?.join("/login")?.to_string();

        // Go to the login website.
        client.goto(&url).await?;

        // Wait to be redirected
        client
            .wait()
            .for_element(Css("#next"))
            .await
            .context("Failed to enter login page")?;

        info!("Login with email: \"{email}\" password: \"{password}\"");

        // Enter email, password and submit
        client
            .form(Css("#localAccountForm"))
            .await
            .context("Failed to find user login form")?
            .set(Css("#signInName"), email)
            .await
            .context("Failed to add email")?
            .set(Css("#password"), password)
            .await
            .context("Failed to add password")?
            .submit_with(Css("#next"))
            .await
            .context("Failed to submit login form")?;

        // Wait to be redirected again
        client
            .wait()
            .for_element(XPath("//*[@id='wegmansLoginCompLabel']/span"))
            .await
            .context("Failed to login or could not find login navigation bar")?;

        let cookie = client
            .get_named_cookie(WEGMANS_SESSION_COOKIE_NAME)
            .await
            .context("Couldn't retrieve session cookie")?;
        let (cookie_name, cookie_value) = cookie.name_value();

        Ok(format!("{cookie_name}={cookie_value}"))
    }

    pub async fn close(self) -> Result<()> {
        self.0
            .close()
            .await
            .context("Failed to close connection with webdriver")
    }
}
