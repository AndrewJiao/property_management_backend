use crate::room_info;
use bigdecimal::{BigDecimal, FromPrimitive };
use common::data_result::AppResult;
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::{BUSINESS_ERROR_OWNER_FEE_DETAIL_EXIST, DATA_NOT_FOUND};
use common::CURRENT_USE;
use diesel::dsl::insert_into;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use itertools::Itertools;
use log::info;
use repository::component::operation_trait::FeeCalculator;
use repository::owner_fee::OwnerFeeDetailPo;
use repository::owner_info::OwnerBasicInfoPo;
use repository::price_basic::{BasicPriceType, PriceBasicConfigGet, PriceBasicPo};
use repository::property_fee::{PropertyFeeDetailInsertPo, PropertyFeeDetailPo, PropertyFeeDetailUpdatePo};
use repository::room_info::RoomInfoDetailPo;
use std::collections::{HashMap, HashSet};

///
/// 编辑的过程中尝试重新计算
///
pub fn do_edit_update(update_po: PropertyFeeDetailUpdatePo) -> AppResult<PropertyFeeDetailPo>
{
    let conn = &mut db_get_connection();
    //校验是否已生成费用明细
    let binding = &PropertyFeeDetailPo::by_id(update_po.id)
        .get_result::<PropertyFeeDetailPo>(conn)?;
    let record = &vec![(binding.room_number.as_deref().ok_or(DATA_NOT_FOUND())?, binding.record_version.as_deref().ok_or(DATA_NOT_FOUND())?)];
    let has_owner_fee_data = OwnerFeeDetailPo::by_room_number_and_relative_order_numbers(record, conn)?.is_empty();
    if !has_owner_fee_data {
        return Err(BUSINESS_ERROR_OWNER_FEE_DETAIL_EXIST());
    }

    let mut exist_po: PropertyFeeDetailUpdatePo = binding.into();
    exist_po.update(update_po);
    exist_po.fee_calculate();
    //交给外部去修改
    let result = exist_po
        .update_time()
        .save_changes::<PropertyFeeDetailPo>(conn)?;
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
            .filter(is_delete.eq(false))
            .select(all_columns).get_results::<RoomInfoDetailPo>(&mut db_get_connection())?;
    }
    let owner_info_config;
    {
        use repository::schema::basic::t_owner_basic_info::*;
        owner_info_config = table
            .filter(is_delete.eq(false))
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

    //找到有会产生违约金的数据
    let un_payment_streams = try_get_un_payment_stream_data()?;

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

                    let (ele_total, ele_share) = info.calculate_electric(&basic_price_config, owner_info.room_square.as_ref(), owner_info.room_type);
                    let (water_total, water_share) = info.calculate_water(&basic_price_config, owner_info.room_type).unzip();

                    let lift_fee_basic = basic_price_config.get(&BasicPriceType::LiftFeeBasic).map(|info| info.basic_number.clone()).flatten();
                    let lift_fee_plus = basic_price_config.get(&BasicPriceType::LiftFeePlus).map(|info| info.basic_number.clone()).flatten();
                    let lift_fee: Option<BigDecimal> = info.calculate_lift(lift_fee_basic, lift_fee_plus);
                    //计算违约金
                    let liquidate_fee = un_payment_streams.get(room_num_ref).map(|stream_vec| {
                        stream_vec.iter().map(|e| e.amount.clone()).reduce(|a, b| a + b)
                    //如果还有余额，把它计入预存

                    }).flatten().map(|arrears| calculate_liquidate_fee(arrears, &basic_price_config));
                    Some(PropertyFeeDetailInsertPo {
                        room_number: info.room_number.as_deref().unwrap(),
                        room_owner_name: owner_info.owner_name.as_deref(),
                        management_fee: owner_info.calculate_management_fee(&basic_price_config),
                        part_fee: owner_info.calculate_part_fee(&basic_price_config),
                        lift_fee,
                        machine_room_renovation_fee: machine_room_fee,
                        electric_fee: ele_total,
                        electric_share_fee: ele_share,
                        water_fee: water_total,
                        water_share_fee: water_share,
                        liquidate_fee,
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
                        total_charge: None,
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


///
/// 找到有欠款的用户，获取三个月前的未缴费数据
///
pub fn try_get_un_payment_stream_data()->AppResult<HashMap<String,Vec<OwnerFeeDetailPo>>> {
    let room_infos = OwnerBasicInfoPo::by_all_un_payment()?
        .into_iter().map(|info| info.room_number).collect::<Vec<String>>();
    let result = OwnerFeeDetailPo::by_all_un_payment_stream(&room_infos)?
        //分个组
        .into_iter().into_group_map_by(|info| info.room_number.clone())
        //累计了2个月才计算
        .into_iter().filter(|(_, v)| v.len() >= 2)
        .collect::<HashMap<String, Vec<OwnerFeeDetailPo>>>();
    info!("un_payment_stream_data:{:?}",result.iter().map(|(e,v)|(e.clone(),v.iter().map(|x|x.stream_id.clone()).join(","))).collect::<HashMap<String,String>>());
    Ok(result)
}

pub fn calculate_liquidate_fee(arrears: BigDecimal, price_config: &HashMap<BasicPriceType, PriceBasicPo>) -> BigDecimal {
    let liquidate_rate = price_config.get(&BasicPriceType::LiquidateFee).map(|info| info.basic_number.as_ref()).flatten().ok_or(DATA_NOT_FOUND()).expect("违约金费率未配置");
    //违约金 = 欠费 * 0.01
    arrears * liquidate_rate * BigDecimal::from_f64(0.01).expect("liquidate_rate is not a number")
}

pub mod excel{
    use bigdecimal::ToPrimitive;
    use common::const_value::SETTINGS;
    use common::data_result::AppResult;
    use itertools::Itertools;
    use lazy_static::lazy_static;
    use log::debug;
    use repository::property_fee::PropertyFeeDetailPo;
    use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, Workbook, Worksheet};

    pub fn build_work_book (all_result:Vec<PropertyFeeDetailPo>) ->AppResult<Vec<u8>>{
        debug!("all_result.len:{}",all_result.len());
        let mut workbook = Workbook::new();
        //设置字体
        let sheet = workbook.add_worksheet();
        sheet.set_name("物业费用")?;
        //分组all_result，每20个为一组
        let per_form_row:u32 = 20;
        let part_results: Vec<Vec<&PropertyFeeDetailPo>> = all_result.chunks(per_form_row as usize).map(|e|e.iter().map(|a|a).collect_vec() ).collect();

        for ref part_result in part_results{
            sheet.build_a_form(part_result,&mut 0,per_form_row)?;
        }

        let  buffer = workbook.save_to_buffer()?;
        Ok(buffer)
    }

    lazy_static!(
        pub static ref BASIC_FORMATTER:Format = Format::new()
                    .set_border(FormatBorder::Thin)
                    .set_align(FormatAlign::Center)
                    .set_align(FormatAlign::VerticalCenter)
                    .set_font_name("宋体")
                    .set_font_size(10);
    );
    lazy_static!(
        pub static ref NUM_FORMATTER:Format = rust_xlsxwriter::Format::from(BASIC_FORMATTER.clone())
            .set_num_format("0.00").set_bold();
    );
    lazy_static!(
            pub static ref BOLD_FORMATTER:Format = Format::from(&BASIC_FORMATTER.clone()).set_bold();
    );
    pub trait BuildForm{
        fn build_a_form(&mut self, part_result: &Vec<&PropertyFeeDetailPo>, current_row: &mut u32, per_line: u32) ->AppResult<()>;
        fn build_header(&mut self, current_row: &mut u32)->AppResult<()>;

        fn build_sum_line(&mut self, current_row: &mut u32, per_form_row: u32) -> AppResult<()>;

        fn build_form_detail(&mut self, part_result: &Vec<&PropertyFeeDetailPo>, current_row: &mut u32, per_form_row: u32) -> AppResult<()>;
    }


    impl BuildForm for Worksheet{
        fn build_a_form(&mut self, part_result: &Vec<&PropertyFeeDetailPo>, current_row: &mut u32, per_form_row: u32) -> AppResult<()> {
            debug!("per_result_len:{} current_row:{},per_form_row:{}",part_result.len(),current_row,per_form_row);
            //设置行宽
            for index in 0..20{
                self.set_column_width(index, SETTINGS.excel_config.basic_width)?;
            }
            self.build_header(current_row)?;
            self.build_form_detail(part_result, current_row, per_form_row)?;
            self.build_sum_line(current_row, per_form_row)?;


            Ok(())
        }



        fn build_header(&mut self, current_row: &mut u32) ->AppResult<()>{
            debug!("row:{}",current_row);
            self.write_string_with_format(*current_row, COL_A, "房间号", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_B, "管理费", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_C, "停车费", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_D, "电梯费", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_E, "机房改造费", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_F, "电费", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_G, "电费分摊", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_H, "水费", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_I, "水费分摊", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_J, "违约金", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_K, "预存", &BOLD_FORMATTER)?;
            self.write_string_with_format(*current_row, COL_L, "总费用", &BOLD_FORMATTER)?;
            self.set_row_height(current_row.clone(),SETTINGS.excel_config.basic_height)?;
            //下标向下移动
            *current_row += 1;
            Ok(())
        }

        fn build_sum_line(&mut self, current_row: &mut u32, per_form_row: u32) -> AppResult<()> {

            debug!("current_row:{},per_form_row:{}",current_row,per_form_row);


            let end = current_row.clone();
            let star = end - per_form_row.clone();
            self.write_string_with_format(*current_row, COL_A, "合计：", &BOLD_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_B, format!("=SUM(B{}:B{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_C, format!("=SUM(C{}:C{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_D, format!("=SUM(D{}:D{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_E, format!("=SUM(E{}:E{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_F, format!("=SUM(F{}:F{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_G, format!("=SUM(G{}:G{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_H, format!("=SUM(H{}:H{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_I, format!("=SUM(I{}:I{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_J, format!("=SUM(J{}:J{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_K, format!("=SUM(K{}:K{})", star, end).as_str(), &NUM_FORMATTER)?;
            self.write_formula_with_format(*current_row, COL_L, format!("=SUM(B{}:I{})", end + 1, end + 1).as_str(), &NUM_FORMATTER)?;
            self.set_row_height(current_row.clone(), SETTINGS.excel_config.basic_height)?;

            *current_row += 1;
            Ok(())


        }


        fn build_form_detail(&mut self, part_result: &Vec<&PropertyFeeDetailPo>, current_row: &mut u32, per_form_row: u32) -> AppResult<()> {
            let form_star_row = current_row.clone();
            let form_star_col = 0;
            //设置表格的全局样式
            let form_end_row = form_star_row + per_form_row;
            let form_end_col = COL_L;

            self.set_range_format(form_star_row, form_star_col, form_end_row, form_end_col, &Format::new()
                .set_border(FormatBorder::Thin)
                .set_align(FormatAlign::Center)
                .set_align(FormatAlign::VerticalCenter)
                .set_font_name("宋体")
                .set_font_size(10)
            )?;
            for index in 0..per_form_row {
                if let Some(data) = part_result.get(index as usize) {
                    self.write_string_with_format(*current_row, 0, data.room_number.as_deref().unwrap_or(""), &BOLD_FORMATTER)?;
                    if let Some(fee) = data.management_fee.as_ref() {
                        self.write_number_with_format(*current_row, 1, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    if let Some(fee) = data.part_fee.as_ref() {
                        self.write_number_with_format(*current_row, 2, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    if let Some(fee) = data.machine_room_renovation_fee.as_ref() {
                        self.write_number_with_format(*current_row, 4, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    if let Some(fee) = data.electric_fee.as_ref() {
                        self.write_number_with_format(*current_row, 5, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    if let Some(fee) = data.electric_share_fee.as_ref() {
                        self.write_number_with_format(*current_row, 6, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    if let Some(fee) = data.water_fee.as_ref() {
                        self.write_number_with_format(*current_row, 7, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    if let Some(fee) = data.water_share_fee.as_ref() {
                        self.write_number_with_format(*current_row, 8, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    if let Some(fee) = data.liquidate_fee.as_ref() {
                        self.write_number_with_format(*current_row, 9, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    if let Some(fee) = data.pre_store_fee.as_ref() {
                        self.write_number_with_format(*current_row, 10, fee.to_f64().unwrap_or(0.0), &NUM_FORMATTER)?;
                    }
                    //=SUM(B2:I2)
                    self.write_formula_with_format(*current_row, 11, format!("=SUM(B{}:I{})", current_row.clone() + 1, current_row.clone() + 1).as_str(), &NUM_FORMATTER)?;
                } else {
                    //=SUM(B2:I2)
                    self.write_formula_with_format(*current_row, 11, format!("=SUM(B{}:I{})", current_row.clone() + 1, current_row.clone() + 1).as_str(), &NUM_FORMATTER)?;
                }

                self.set_row_height(current_row.clone(), SETTINGS.excel_config.basic_height)?;
                //下标向下移动
                *current_row += 1;
            }

            Ok(())
        }
    }

    const COL_A:u16=0;
    const COL_B:u16=1;
    const COL_C:u16=2;
    const COL_D:u16=3;
    const COL_E:u16=4;
    const COL_F:u16=5;
    const COL_G:u16=6;
    const COL_H:u16=7;
    const COL_I:u16=8;
    const COL_J:u16=9;
    const COL_K:u16=10;
    const COL_L:u16=11;



}

