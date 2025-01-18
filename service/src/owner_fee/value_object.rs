use bigdecimal::{BigDecimal, FromPrimitive};
use repository::owner_fee::DetailType;
use repository::price_basic::{BasicPriceType, PriceBasicPo};

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
                //暂时不会走这个逻辑
                let liquidated_damages_rate = PriceBasicPo::with_price_type(BasicPriceType::LiquidateFee).unwrap();
                let amount = amount_balance * (liquidated_damages_rate.basic_number.unwrap() * BigDecimal::from_f64(0.01).unwrap());
                self.amount = Some(amount.clone());
                amount_balance + &amount
            }
            DetailType::ManagementFee | DetailType::PreStoreDeduction => {
                amount_balance + self.amount.as_ref().expect("need amount")
            }
            DetailType::PreStoreFee | DetailType::SettlementFee => {
                amount_balance - self.amount.as_ref().expect("need amount")
            }
        }
    }
}