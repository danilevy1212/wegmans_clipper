use super::{client_provider::provide_client, handle_error::HandleError};
use crate::constants::WEGMANS_BASE_URL;
use crate::coupons_dto::{CouponDTO, OffersDTO};
use anyhow::{Ok, Result};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Url,
};

#[derive(Clone)]
pub struct CouponClient {
    client: reqwest::Client,
    headers: reqwest::header::HeaderMap,
}

impl CouponClient {
    pub fn new(session_cookie: &str) -> Result<CouponClient> {
        // common api headers
        let mut headers = HeaderMap::new();

        headers.insert(header::COOKIE, HeaderValue::from_str(session_cookie)?);
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str("application/json")?,
        );
        Ok(CouponClient {
            client: provide_client(),
            headers,
        })
    }

    pub async fn get_coupons(&self) -> Result<OffersDTO> {
        let mut url = Url::parse(WEGMANS_BASE_URL)?.join("/api/v2/offers")?;
        url.set_query(Some("type=coupon"));

        let response = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .handle_error()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn clip_coupon(&self, id: &str) -> Result<CouponDTO> {
        let url = Url::parse(WEGMANS_BASE_URL)?.join(&format!("/api/v2/offers/{id}"))?;

        let response = self
            .client
            .put(url)
            .headers(self.headers.clone())
            .body("{\"clipped\":true}")
            .send()
            .await?
            .handle_error()
            .await?;

        Ok(response.json().await?)
    }
}
