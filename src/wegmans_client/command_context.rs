use crate::webdriver_client::WebDriverClient;
use anyhow::{Result, Context};
use std::future::Future;
use std::process::Stdio;
use tokio::process::Command;

pub struct WebDriverContext<'a> {
    webdriver_url: &'a str,
    webdriver_command: &'a str,
    webdriver_args: Vec<String>,
    email: &'a str,
    password: &'a str,
}

impl<'a> WebDriverContext<'a> {
    pub fn new(
        webdriver_url: &'a str,
        webdriver_command: &'a str,
        webdriver_args: Vec<String>,
        email: &'a str,
        password: &'a str,
    ) -> Self {
        WebDriverContext {
            email,
            password,
            webdriver_url,
            webdriver_args,
            webdriver_command,
        }
    }

    /// Run a closure in the context of a connection with a webdriver server.
    ///
    /// The closure will recieve the `session_cookie` needed for all futher calls
    /// to the HTTP API.
    ///
    /// Helper to automate opening and closing connection to the webdriver server.
    pub async fn with_web_driver_session<F, Fut>(self, f: F) -> Result<()>
    where
        F: Fn(String) -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        let mut gecko = Command::new(self.webdriver_command)
            .args(&self.webdriver_args)
            .kill_on_drop(true)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context(format!("{0} not in path", self.webdriver_command))?;

        let client = WebDriverClient::new(self.webdriver_url).await?;

        let session_cookie = client.get_session_cookie(self.email, self.password).await;

        let result: Result<()> = if session_cookie.is_err() {
            Err(session_cookie.err().unwrap())
        } else {
            f(session_cookie.ok().unwrap()).await
        };

        client.close().await?;
        gecko.kill().await?;

        result
    }
}
