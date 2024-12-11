use bigdecimal::BigDecimal;
use common::const_value::SETTINGS;
use repository::owner_fee::DetailType;

#[derive(Debug)]
pub struct StreamAddVal {
    pub stream_type: DetailType,
    pub room_number: String,
    pub amount: Option<BigDecimal>,
    pub relative_order_number: String,

}
impl StreamAddVal {
    pub fn calculate(&mut self, amount_balance: &BigDecimal) -> BigDecimal {
        match &self.stream_type {
            DetailType::LiquidatedDamages => {
                let amount = amount_balance * &SETTINGS.app_config.liquidated_damages_rate;
                self.amount = Some(amount.clone());
                amount_balance + &amount
            }
            DetailType::ManagementFee | DetailType::SettlementFee | DetailType::PreStoreFee => {
                amount_balance + self.amount.as_ref().expect("need amount")
            }
        }
    }
}