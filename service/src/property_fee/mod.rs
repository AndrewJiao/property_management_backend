use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::AppResult;
use diesel::SaveChangesDsl;
use repository::component::operation_trait::FeeCalculator;
use repository::property_fee::{PropertyFeeDetailPo, PropertyFeeDetailUpdatePo};

///
/// 编辑的过程中尝试重新计算
///
pub fn do_edit_update(mut po: PropertyFeeDetailUpdatePo) -> AppResult<()> {
    po.fee_calculate();
    po.update_time()
        .save_changes::<PropertyFeeDetailPo>(&mut db_get_connection())?;
    Ok(())
}

///
/// 依据基础数据 + 用户数据 > 计算出费用
///
pub fn init_data() -> AppResult<()> {
    // let basic_price;
    // {
    //     use repository::schema::basic::t_price_basic::*;
    //     basic_price = table.select(all_columns).get_results::<PriceBasicPo>()?
    //         .into_iter().map(|x| (x., x)).collect::<std::collections::HashMap<i64, PriceBasicPo>>();
    //     ;
    // }

    todo!();
}