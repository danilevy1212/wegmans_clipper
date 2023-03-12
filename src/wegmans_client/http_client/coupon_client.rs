use crate::constants::WEGMANS_BASE_URL;
use anyhow::{Ok, Result};

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Url,
};

use crate::coupons_dto::{Coupon, OffersDTO};

use super::{handle_error::HandleError, client_provider::provide_client};

#[derive(Clone)]
pub struct CouponClient {
    client: reqwest::Client,
    api_base: Url,
    headers: reqwest::header::HeaderMap,
}

impl CouponClient {
    pub fn new(session_cookie: &str) -> Result<CouponClient> {
        let api_base = Url::parse(WEGMANS_BASE_URL)?.join("/api/v2")?;

        // common api headers
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, HeaderValue::from_str(session_cookie)?);
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str("application/json")?,
        );
        Ok(CouponClient {
            api_base,
            client: provide_client(),
            headers,
        })
    }

    pub async fn get_coupons(&self) -> Result<OffersDTO> {
        let mut url = self.api_base.clone().join("offers")?;
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

    pub async fn clip_coupon(&self, id: &str) -> Result<Coupon> {
        let url = self.api_base.clone().join(&format!("offers/{id}"))?;

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
