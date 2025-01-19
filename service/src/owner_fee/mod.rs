use crate::owner_fee::value_object::StreamAddVal;
use bigdecimal::{BigDecimal, Zero};
use common::data_result::{AppError, AppResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::{db_get_connection, Conn};
use common::error::{BUSINESS_ERROR, DATA_NOT_EXIST};
use common::tools::lock::LOCK_OWNER_FEE;
use common::tools::transaction::TryTransaction;
use diesel::SaveChangesDsl;
use itertools::Itertools;
use log::{debug, info};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use repository::owner_fee::{create_new_owner_fee_detail_stream, AllRelativeStream, DetailType, OwnerFeeDetailPo, OwnerFeeDetailRecordPo, OwnerFeeDetailUpdatePo};
use repository::owner_info::OwnerBasicInfoPo;
use repository::property_fee::PropertyFeeDetailPo;
use repository::tool_table::CountType;
use repository::{owner_fee, owner_info};
use std::collections::HashMap;

pub mod value_object;

///
/// 每次修改时要验证价格
///
pub fn put_data(po: OwnerFeeDetailUpdatePo) -> AppResult<OwnerFeeDetailPo> {
    let result = po.update_time()
        .save_changes(&mut db_get_connection())?;
    Ok(result)
}

///
/// 初始化一个新的流水
///
pub fn new_data(value: StreamAddVal) -> AppResult<OwnerFeeDetailPo> {
    new_data_with_conn(value, Some(&mut db_get_connection()))
}
pub fn new_data_with_conn(mut value: StreamAddVal, conn: Option<&mut Conn>) -> AppResult<OwnerFeeDetailPo> {
    info!("添加流水数据: {:?}", value);
    let p_room_number = &value.room_number.clone();

    let result = conn.try_transaction::<_, AppError, _>(|conn| {
        //查询业主信息
        let mut basic_info = OwnerBasicInfoPo::by_room_number(p_room_number, conn)?;
        let _guard = LOCK_OWNER_FEE.try_lock(basic_info.room_number.as_str())?;
        let result ;
        if value.stream_type == DetailType::ManagementFee {
            result = create_management_fee(&value, &basic_info, conn)?;
        } else {
            //生成一个单号
            let new_stream_order_number = &repository::tool_table::current_date_count_with_conn(CountType::OwnerFeeSeqNumber, conn)?;
            //计算余额
            let new_amount_balance = value.calculate(&mut basic_info.amount_balance);
            info!("room_number = {:?} stream_type = {:?} stream_amount = {:?} before_amount = {:?} after_amount= {:?}",  value.room_number,value.stream_type,value.amount, basic_info.amount_balance, &new_amount_balance);
            //更新记录表，新增流水数据（非业务逻辑)
            let record = owner_fee::try_record_data(&new_amount_balance, p_room_number, conn)?;
            //开始创建新流水
            result = create_new_owner_fee_detail_stream(
                new_stream_order_number,
                p_room_number,
                basic_info.owner_name.as_deref(),
                &value.stream_type,
                &value.amount.clone().ok_or(BUSINESS_ERROR("amount is required", 1001))?,
                &record.record_id,
                value.relative_order_number.as_str(),
                conn,
            )?;
            //更新余额
            owner_info::update_amount(basic_info.id, &new_amount_balance, conn)?;
        }
        //如果是结算流水，就更新要结算的流水信息
        if value.stream_type == DetailType::SettlementFee {
            OwnerFeeDetailUpdatePo::settle_post_processer(&value.relative_order_number, &result.stream_id, conn)?;
        }
        Ok(result)
    })?;
    Ok(result)
}
///
/// 自动结算
///
fn create_management_fee(value: &StreamAddVal, basic_info: &OwnerBasicInfoPo, conn: &mut Conn) -> AppResult<OwnerFeeDetailPo> {
    let mut amount = value.amount.clone().ok_or(BUSINESS_ERROR("amount is required", 1001))?;
    let mut pre_store_amount = None;
    let basic_origin_amount = value.amount.clone().expect("amount is required");
    let pre_calc_amount_balance = &basic_info.amount_balance + &basic_origin_amount;
    if pre_calc_amount_balance <= BigDecimal::zero() {
        //结余小于0说明足额抵扣
        pre_store_amount = Some(amount.clone());
        amount = BigDecimal::zero();
    } else if pre_calc_amount_balance > BigDecimal::zero() && pre_calc_amount_balance < basic_origin_amount {
        //结余大于=0,但是小于物业费说明部分抵扣
        amount = &basic_info.amount_balance + &basic_origin_amount;
        pre_store_amount = Some(&basic_origin_amount - &amount);
    }
    info!("create management stream  room ={:?} pre_store_amount= {:?},amount= {:?}", value.room_number,pre_store_amount, &amount);
    //创建预存抵扣
    if let Some(pre_store_amount) = pre_store_amount{
        new_data_with_conn(StreamAddVal {
            stream_type: DetailType::PreStoreDeduction,
            room_number: value.room_number.clone(),
            amount: Some(pre_store_amount),
            //手动添添加没有单号，先固定-
            relative_order_number: "--".to_string(),
        }, Some(conn))?;
    }


    //生成一个单号
    let new_stream_order_number = &repository::tool_table::current_date_count_with_conn(CountType::OwnerFeeSeqNumber, conn)?;
    //real添加物业费流水
    let basic_info = OwnerBasicInfoPo::by_room_number(&value.room_number, conn)?;
    let amount_balance_after_pre_deduction = &basic_info.amount_balance + &amount;
    owner_info::update_amount(basic_info.id, &amount_balance_after_pre_deduction, conn)?;
    let record = owner_fee::try_record_data(&amount_balance_after_pre_deduction, &value.room_number, conn)?;
    info!("room_number = {:?} stream_type = {:?} stream_amount = {:?} before_amount = {:?} after_amount= {:?}",  value.room_number,value.stream_type, &amount, basic_info.amount_balance, &amount_balance_after_pre_deduction);
    //开始创建新流水
    let result = create_new_owner_fee_detail_stream(
        new_stream_order_number,
        value.room_number.as_str(),
        basic_info.owner_name.as_deref(),
        &value.stream_type,
        &amount,
        &record.record_id,
        value.relative_order_number.as_str(),
        conn,
    )?;
    //如果全额抵扣，则自动0元结算
    if amount == BigDecimal::zero() {
        new_data_with_conn(StreamAddVal {
            stream_type: DetailType::SettlementFee,
            room_number: value.room_number.clone(),
            amount: Some(BigDecimal::zero()),
            //手动添添加没有单号，先固定-
            relative_order_number: result.stream_id.clone(),
        }, Some(conn))?;
    }

    Ok(result)
}

///
/// 如果有预存提前生成预存单
///
pub fn try_pre_store_deduction(fee: &PropertyFeeDetailPo) ->AppResult<()> {
    let room_number = fee.room_number.clone().expect("data_not_exist");
    let relate_version = fee.record_version.as_deref().expect("data_not_exit");
    //先判断是否已有预存单生成，因为没有涉及事务

    let conn = &mut db_get_connection();
    let exist_owner_fees = OwnerFeeDetailPo::by_room_number_and_relative_order_numbers(&vec![(room_number.as_str(), relate_version)], conn)?
        .into_iter()
        .filter(|e| e.detail_type == DetailType::PreStoreFee).collect::<Vec<_>>();
    if exist_owner_fees.len() > 0 {
        return Ok(());
    }

    if let Some(pre_store) = &fee.pre_store_fee {
        new_data(StreamAddVal {
            stream_type: DetailType::PreStoreFee,
            room_number,
            amount: Some(pre_store.clone()),
            //手动添添加没有单号，先固定-
            relative_order_number: relate_version.to_string(),
        })?;
    }
    Ok(())

}


///
/// 根据stream_id重新从数据库取出相关的流水数据计算余额
///
pub async fn re_calculate_amount_balance(stream_record_id: &Vec<&str>) -> AppResult<HashMap<String, BigDecimal>> {
    debug!("re_calculate_amount_balance: {:?}", stream_record_id);
    let conn = &mut db_get_connection();

    let all_relative_stream = OwnerFeeDetailPo::get_by_stream_record_id_list(stream_record_id, conn)?;
    let all_relative_stream_map = all_relative_stream.into_iter().map(|e| (e.record_id.clone(), e)).into_group_map();

    let all_relative_record = OwnerFeeDetailRecordPo::by_record_id_list(stream_record_id, conn)?;
    let record_amount_map = &all_relative_record.into_iter().map(|e| (e.record_id, e.amount_balance)).collect::<HashMap<String, BigDecimal>>();
    debug!("record_amount_map: {:?}", record_amount_map);
    let result = all_relative_stream_map
        .into_par_iter()
        .map(|(k, e)| {
            calculate(e, record_amount_map.get(k.as_str()).expect("record_id not found").clone())
        })
        .reduce(
            || HashMap::new()
            , |mut a, b| {
                a.extend(b);
                a
            });
    Ok(result)
}


fn calculate(mut relative_stream: Vec<OwnerFeeDetailPo>, p_amount: BigDecimal) -> HashMap<String, BigDecimal> {
    debug!("relative_stream: {:?} record_amount: {:?}", relative_stream , p_amount);
    let mut p_amount_balance = p_amount;
    assert!(relative_stream.len() > 0);
    let mut map = HashMap::new();
    //根据时间倒序排序
    relative_stream.sort_by(|a, b| b.stream_id.cmp(&a.stream_id));
    for detail_po in relative_stream {
        map.insert(detail_po.stream_id.clone(), p_amount_balance.clone());
        if detail_po.detail_type == DetailType::SettlementFee || detail_po.detail_type == DetailType::PreStoreFee || DetailType::AdjustOrder == detail_po.detail_type {
            p_amount_balance = p_amount_balance + detail_po.amount;
        } else {
            p_amount_balance = p_amount_balance - detail_po.amount;
        }
    }
    map
}

///
/// 根据房间号和版本生成指定的物业费
///
pub fn add_data(param_room_number:&str, param_version:&str) ->AppResult<OwnerFeeDetailPo>{
    debug!("add_data: {},{}", param_room_number, param_version);
    let conn = &mut db_get_connection();
    //查询是否有物业费
    let property_fee = PropertyFeeDetailPo::by_room_number_and_version(param_room_number, param_version, conn)?;
    let exist_owner_fees = OwnerFeeDetailPo::by_room_number_and_relative_order_numbers(&vec![(param_room_number, param_version)], conn)?;
    debug!("exist_owner_fees: {:?}", exist_owner_fees);
    //有就直接返回
    if exist_owner_fees.len() > 0 {
        return Ok(exist_owner_fees.into_iter().next().unwrap());
    }
    //预存扣除
    let _ = self::try_pre_store_deduction(&property_fee);

    self::new_data(StreamAddVal {
        stream_type: DetailType::ManagementFee,
        room_number: property_fee.room_number.ok_or(DATA_NOT_EXIST())?,
        amount: property_fee.total_fee,
        relative_order_number: property_fee.record_version.ok_or(DATA_NOT_EXIST())?,
    })
}
///
/// 基于版本号生成数据
///

pub fn add_datas(param_version:&str) ->AppResult<()>{
    debug!("add_datas: {}", param_version);
    let conn = &mut db_get_connection();
    //查询是否有物业费
    let property_fee = PropertyFeeDetailPo::by_version(param_version,conn)?;
    let condition_par = property_fee.iter().map(|e| (e.room_number.as_deref().expect("data_not_exist"), e.record_version.as_deref().expect("data_not_exit"))).collect();
    let exist_owner_fees = OwnerFeeDetailPo::by_room_number_and_relative_order_numbers(&condition_par,conn)?;
    //过滤掉已生成的流水
    tokio::spawn(async move {
        let need_create =
            property_fee.into_iter()
                .filter(|e|!exist_owner_fees.contains(e)).collect::<Vec<_>>();


        need_create.into_iter()
            .for_each(|each_fee|{
                //预存扣除
                let _ = self::try_pre_store_deduction(&each_fee);
                let _ = self::new_data(StreamAddVal {
                    stream_type: DetailType::ManagementFee,
                    room_number: each_fee.room_number.ok_or(DATA_NOT_EXIST()).unwrap(),
                    amount: each_fee.total_fee,
                    relative_order_number: each_fee.record_version.ok_or(DATA_NOT_EXIST()).unwrap(),
                });
            });
    });

    Ok(())
}

trait  Contains{
    fn contains(&self,_: &PropertyFeeDetailPo)->bool;
}
impl Contains for Vec<OwnerFeeDetailPo>  {
    fn contains(&self ,po: &PropertyFeeDetailPo) -> bool {
        self.iter().any(|e| {
            po.room_number.as_deref() == Some(&e.room_number) &&
                po.record_version.as_deref() == Some(&e.related_order_number)
        })
    }
}

///
/// 手动添加数据，预存
///
pub fn manually_add_data(amount:BigDecimal,room_number:String) -> AppResult<OwnerFeeDetailPo> {
    debug!("manually_add_data: {}", amount);
    let detail_type = DetailType::AdjustOrder;
    //没有就抛出错
    OwnerBasicInfoPo::by_room_number(&room_number, &mut db_get_connection())?;

    let result = new_data(StreamAddVal {
        stream_type: detail_type,
        room_number,
        amount: Some(amount),
        //手动添添加没有单号，先固定-
        relative_order_number: "-".to_string(),
    })?;
    Ok(result)
}

///
/// 手动结算，生成指定流水的负向流水
///
pub fn manually_add_settle_data(p_stream_id:String) -> AppResult<OwnerFeeDetailPo> {
    debug!("manually_add_settle_data: {}", p_stream_id);
    let detail_type = DetailType::SettlementFee;
    //查询要结算的流水判断状态是否可以结算(目前只有物业费和滞纳金可以结算)
    let detail_types = vec![DetailType::ManagementFee, DetailType::LiquidatedDamages];
    let AllRelativeStream{common_stream, deduction_streams } = OwnerFeeDetailPo::all_relative_stream_by_stream_id(p_stream_id, detail_types, &mut db_get_connection())?;
    if !deduction_streams.is_empty() {
        return Err(BUSINESS_ERROR("已有结算流水", 1001));
    }
    let result = new_data(StreamAddVal {
        stream_type: detail_type,
        room_number: common_stream.room_number,
        amount: Some(common_stream.amount),
        relative_order_number: common_stream.stream_id,
    })?;
    Ok(result)
}