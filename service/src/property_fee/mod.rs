use crate::room_info;
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::AppResult;
use common::CURRENT_USE;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use repository::component::operation_trait::FeeCalculator;
use repository::owner_info::OwnerBasicInfoPo;
use repository::price_basic::{BasicPriceType, PriceBasicConfigGet, PriceBasicPo};
use repository::property_fee::{PropertyFeeDetailInsertPo, PropertyFeeDetailPo, PropertyFeeDetailUpdatePo};
use repository::room_info::RoomInfoDetailPo;
use std::collections::{HashMap, HashSet};
use diesel::dsl::insert_into;

///
/// 编辑的过程中尝试重新计算
///
pub fn do_edit_update(mut po: PropertyFeeDetailUpdatePo) -> AppResult<PropertyFeeDetailPo> {
    po.fee_calculate();
    let result  = po.update_time()
        .save_changes::<PropertyFeeDetailPo>(&mut db_get_connection())?;
    Ok(result)
}

///
/// 依据基础数据 + 用户数据 > 计算出费用
///
pub fn init_data(version: Option<&str>) -> AppResult<()> {
    let basic_price_config;
    {
        use repository::schema::basic::t_price_basic::*;
        basic_price_config =
            table.select(all_columns)
                .get_results::<PriceBasicPo>(&mut db_get_connection())?
                .to_price_type_map();
    }
    let room_info;

    let temp = room_info::init_current_month_version(chrono::Local::now());
    let n_month_version = version
        .unwrap_or(temp.as_str());
    {
        use repository::schema::basic::t_room_info_detail::*;

        room_info = table
            .filter(month_version.eq(n_month_version))
            .select(all_columns).get_results::<RoomInfoDetailPo>(&mut db_get_connection())?;
    }
    let owner_info_config;
    {
        use repository::schema::basic::t_owner_basic_info::*;
        owner_info_config = table
            .select(all_columns).get_results::<OwnerBasicInfoPo>(&mut db_get_connection())?
            .into_iter()
            .map(|info| (info.room_number.clone(), info)
            ).collect::<HashMap<String, OwnerBasicInfoPo>>();
    }
    //排除掉已有数据
    let exist_room_number;
    {
        use repository::schema::basic::t_property_fee_detail::*;
        exist_room_number = table
            .filter(record_version.eq(n_month_version))
            .filter(is_delete.eq(false))
            .select(room_number)
            .get_results::<Option<String>>(&mut db_get_connection())?
            .into_iter().flatten().collect::<HashSet<String>>();
    }


    //基于room_info生成费用数据
    let data_insert = room_info.iter()
        .filter(|info| !exist_room_number.contains(info.room_number.as_deref().unwrap()))
        .flat_map(|info| {
            let default = String::new();
            let room_num_ref = info.room_number.as_ref().unwrap_or(&default);
            match owner_info_config.get(room_num_ref) {
                None => { None }
                Some(owner_info) => {
                    let machine_room_fee = basic_price_config.get(&BasicPriceType::MachineRoomRenovationFee).map(|info| info.basic_number.clone()).flatten();
                    let (ele_total, ele_share) = info.calculate_electric(&basic_price_config).unzip();
                    let (water_total, water_share) = info.calculate_water(&basic_price_config).unzip();
                    Some(PropertyFeeDetailInsertPo {
                        room_number: info.room_number.as_deref().unwrap(),
                        room_owner_name: owner_info.owner_name.as_deref(),
                        management_fee: owner_info.calculate_management_fee(&basic_price_config),
                        part_fee: owner_info.calculate_part_fee(&basic_price_config),
                        machine_room_renovation_fee: machine_room_fee,
                        electric_fee: ele_total,
                        electric_share_fee: ele_share,
                        water_fee: water_total,
                        water_share_fee: water_share,
                        liquidate_fee: None,
                        pre_store_fee: None,
                        record_version: n_month_version.as_ref(),
                        create_by: CURRENT_USE,
                        update_by: CURRENT_USE,
                        create_time: None,
                        update_time: None,
                        is_delete: false,
                        delete_at: None,
                        comment: None,
                        total_fee: None,
                    })
                }
            }
        }).map(|mut po| {
        po.fee_calculate();
        po.create_time()
    }).collect::<Vec<PropertyFeeDetailInsertPo>>();
    if data_insert.is_empty() {
        return Ok(());
    }
    {
        use repository::schema::basic::t_property_fee_detail::table;
        let statement = insert_into(table).values(data_insert);
        println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&statement));
        statement.execute(&mut db_get_connection())?;
    }
    Ok(())
}