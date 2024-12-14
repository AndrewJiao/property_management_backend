use crate::owner_fee::value_object::StreamAddVal;
use bigdecimal::BigDecimal;
use common::data_result::{AppError, AppResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::{BUSINESS_ERROR, DATA_NOT_EXIST};
use common::tools::lock::LOCK_OWNER_FEE;
use diesel::{Connection, SaveChangesDsl};
use itertools::Itertools;
use log::{debug, info };
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
pub fn new_data(mut value: StreamAddVal) -> AppResult<OwnerFeeDetailPo> {
    info!("添加流水数据: {:?}", value);
    let p_room_number = &value.room_number.clone();
    //生成一个单号
    let conn = &mut db_get_connection();
    //查询业主信息
    let mut basic_info = OwnerBasicInfoPo::by_room_number(p_room_number, conn)?;

    let _guard = LOCK_OWNER_FEE.try_lock(basic_info.room_number.as_str())?;
    let result = conn.transaction::<_, AppError, _>(|conn| {
        //计算
        let new_amount_balance = value.calculate(&mut basic_info.amount_balance);
        //更新结余，更新记录表，新增流水数据
        let record = owner_fee::try_record_data(&new_amount_balance, p_room_number, conn)?;
        owner_info::update_amount(basic_info.id, &new_amount_balance, conn)?;

        let result = create_new_owner_fee_detail_stream(
            &repository::tool_table::current_date_count_with_conn(CountType::OwnerFeeSeqNumber, conn)?,
            p_room_number,
            basic_info.owner_name.as_deref(),
            &value.stream_type,
            &value.amount.ok_or(BUSINESS_ERROR("amount is required", 1001))?,
            &record.record_id,
            value.relative_order_number.as_str(),
            conn,
        )?;
        Ok(result)
    })?;
    Ok(result)
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
        if detail_po.detail_type == DetailType::SettlementFee || detail_po.detail_type == DetailType::PreStoreFee {
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
    let property_fee = PropertyFeeDetailPo::by_room_number_and_version(param_room_number, param_version,conn)?;
    let exist_owner_fees = OwnerFeeDetailPo::by_room_number_and_relative_order_numbers(&vec![(param_room_number, param_version)],conn)?;
    debug!("exist_owner_fees: {:?}", exist_owner_fees);
    //有就直接返回
    if exist_owner_fees.len() > 0 {
        return Ok(exist_owner_fees.into_iter().next().unwrap());
    }
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
    //过滤掉已有的
    tokio::spawn(async move {
        let need_create =
            property_fee.into_iter()
                .filter(|e|!exist_owner_fees.contains(e)).collect::<Vec<_>>();

        need_create.into_iter()
            .for_each(|e|{
                let _ = self::new_data(StreamAddVal {
                    stream_type: DetailType::ManagementFee,
                    room_number: e.room_number.ok_or(DATA_NOT_EXIST()).unwrap(),
                    amount: e.total_fee,
                    relative_order_number: e.record_version.ok_or(DATA_NOT_EXIST()).unwrap(),
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
    let detail_type = DetailType::PreStoreFee;
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
   if !deduction_streams.is_empty(){
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