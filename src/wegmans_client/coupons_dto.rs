use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct OffersDTO {
    pub items: Vec<CouponDTO>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CouponDTO {
    pub clipped: bool,
    pub id: String,
    pub name: String,
}
