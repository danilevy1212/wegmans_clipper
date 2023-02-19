use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OffersDTO {
    pub items: Vec<Coupon>,
}

#[derive(Deserialize, Debug)]
pub struct Coupon {
    pub clipped: bool,
    pub id: String,
}
