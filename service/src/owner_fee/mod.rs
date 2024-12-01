use crate::owner_fee::value_object::StreamAddVal;
use common::data_result::{AppError, AppResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::BUSINESS_ERROR;
use common::tools::lock::LOCK_OWNER_FEE;
use diesel::{Connection, SaveChangesDsl};
use repository::owner_fee::{create_new_owner_fee_detail_stream, OwnerFeeDetailPo, OwnerFeeDetailUpdatePo};
use repository::owner_info::OwnerBasicInfoPo;
use repository::tool_table::CountType;
use repository::{owner_fee, owner_info};
use std::thread;
use std::time::Duration;

pub mod value_object;

///
/// 每次修改时要验证价格
///
pub fn put_data(po: OwnerFeeDetailUpdatePo) -> AppResult<OwnerFeeDetailPo> {
    //是否有修改价格
    // po.amount.map(|new_amount| {
    //     let old_owner_fee = OwnerFeeDetailPo::get_by_id(po.id)?;
    //
    //     Ok(())
    // })
    let result = po.update_time()
        .save_changes(&mut db_get_connection())?;
    Ok(result)
}

///
/// 初始化一个新的流水
///
pub fn new_data(mut value: StreamAddVal) -> AppResult<()> {
    let room_number = &value.room_number.clone();
    //生成一个单号
    let conn = &mut db_get_connection();
    //查询业主信息
    let mut basic_info = OwnerBasicInfoPo::by_room_number(room_number, conn)?;

    let _guard = LOCK_OWNER_FEE.try_lock(basic_info.room_number.as_str())?;
    conn.transaction::<_, AppError, _>(|conn| {
        //计算
        thread::sleep(Duration::from_secs(10));

        let new_amount_balance = value.calculate(&mut basic_info.amount_balance);
        //更新结余，更新记录表，新增流水数据
        let record = owner_fee::try_record_data(&new_amount_balance, room_number, conn)?;
        owner_info::update_amount(basic_info.id, &new_amount_balance, conn)?;

        let _ = create_new_owner_fee_detail_stream(
            &repository::tool_table::current_date_count_with_conn(CountType::OwnerFeeSeqNumber, conn)?,
            room_number,
            basic_info.owner_name.as_deref(),
            &value.stream_type,
            &value.amount.ok_or(BUSINESS_ERROR("amount is required", 1001))?,
            &record.record_id,
            conn,
        )?;
        Ok(())
    })?;
    Ok(())
}
