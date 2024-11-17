use bigdecimal::BigDecimal;
use repository::price_basic::UpdatePriceBasicPo;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PriceBasic {
    pub name: Option<String>,
    pub basic_number: Option<BigDecimal>,
}

impl PriceBasic {
    pub fn to_update_po(&self, id: i64) -> UpdatePriceBasicPo {
        UpdatePriceBasicPo {
            id,
            name: self.name.as_deref(),
            basic_number: self.basic_number.as_ref(),
            update_time: Some(chrono::Utc::now().naive_utc()),
        }
    }
}
