use crate::constants::WEGMANS_BASE_URL;
use anyhow::{Context, Result};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Url,
};

use crate::coupons_dto::{Coupon, OffersDTO};

#[derive(Clone)]
pub struct Client {
    api_base: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new(session_cookie: &str) -> Result<Client> {
        let api_base = Url::parse(WEGMANS_BASE_URL)?.join("/api/v2")?;

        // common api headers
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, HeaderValue::from_str(session_cookie)?);

        let error_msg = format!("Unbuildable client, {:#?}", &headers);

        // common client settings for REST endpoint reqwests
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .with_context(|| error_msg)?;

        Ok(Client { api_base, client })
    }
    pub async fn get_coupons(&self) -> Result<OffersDTO> {
        let mut url = self.api_base.clone().join("offers")?;
        url.set_query(Some("type=coupon"));

        let response = self.client.get(url).send().await?.text().await?;

        Ok(serde_json::from_str(&response)?)
    }

    pub async fn clip_coupon(&self, id: &str) -> Result<Coupon> {
        let url = self.api_base.clone().join(&format!("offers/{id}"))?;

        let response = self
            .client
            .put(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body("{\"clipped\":true}")
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str(&response)?)
    }
}
