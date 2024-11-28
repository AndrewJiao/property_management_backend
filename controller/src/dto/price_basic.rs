use crate::dto::ToUpdatePO;
use bigdecimal::BigDecimal;
use repository::price_basic::UpdatePriceBasicPo;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceBasicUpdateDto {
    #[validate(length(min = 0, max = 100))]
    pub name: Option<String>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub basic_number: Option<BigDecimal>,
    #[validate(length(min = 0, max = 1000))]
    pub comment: Option<String>,
}

impl ToUpdatePO for PriceBasicUpdateDto {
    type PO<'a> = UpdatePriceBasicPo<'a>;

    fn to_update_po(&self, id: i64) -> Self::PO<'_> {
        UpdatePriceBasicPo {
            id,
            name: self.name.as_deref(),
            basic_number: self.basic_number.as_ref(),
            update_time: None,
            comment: self.comment.as_deref(),
        }
    }
}
