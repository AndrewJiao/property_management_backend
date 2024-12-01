use bigdecimal::BigDecimal;
use common::const_value::SETTINGS;
use repository::owner_fee::DetailType;

pub struct StreamAddVal {
    pub stream_type: DetailType,
    pub room_number: String,
    pub amount: Option<BigDecimal>,

}
impl StreamAddVal {
    pub fn calculate(&mut self, amount_balance: &BigDecimal) -> BigDecimal {
        match &self.stream_type {
            DetailType::ManagementFee => {
                amount_balance + self.amount.as_ref().expect("need amount")
            }
            DetailType::LiquidatedDamages => {
                let amount =  amount_balance * &SETTINGS.app_config.liquidated_damages_rate;
                self.amount = Some(amount.clone());
                amount_balance + &amount

            }
            DetailType::PreStoreFee => {
                amount_balance + self.amount.as_ref().expect("need amount")
            }
            DetailType::SettlementFee => {
                amount_balance + self.amount.as_ref().expect("need amount")
            }
        }
    }
}