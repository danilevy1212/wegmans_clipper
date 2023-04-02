use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Response;

#[async_trait]
pub trait HandleError {
    type Output: Sized;
    async fn handle_error(self) -> Result<Self::Output>;
}

#[async_trait]
impl HandleError for Response {
    type Output = Response;
    async fn handle_error(self) -> Result<Self> {
        let status = self.status();

        if status.is_client_error() || status.is_server_error() {
            return Err(anyhow!(
                "Response contains error, status code `{status}`:\n\n{}",
                self.text().await?
            ));
        }

        return Ok(self);
    }
}
