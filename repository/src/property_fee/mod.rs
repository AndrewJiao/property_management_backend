use crate::common_type;
use crate::component::operation_trait::FeeCalculator;
use crate::schema::basic::t_property_fee_detail;
use crate::schema::basic::t_property_fee_detail::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::data_result::AppResult;
use common::db_config::{db_get_connection, Conn};
use diesel::pg::Pg;
use diesel::{AsChangeset, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

common_type!();

#[derive(Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = t_property_fee_detail)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct PropertyFeeDetailPo {
    pub id: i64,
    pub room_number: Option<String>,
    pub room_owner_name: Option<String>,
    pub management_fee: Option<BigDecimal>,
    pub part_fee: Option<BigDecimal>,
    pub machine_room_renovation_fee: Option<BigDecimal>,
    pub electric_fee: Option<BigDecimal>,
    pub electric_share_fee: Option<BigDecimal>,
    pub water_fee: Option<BigDecimal>,
    pub water_share_fee: Option<BigDecimal>,
    pub liquidate_fee: Option<BigDecimal>,
    pub pre_store_fee: Option<BigDecimal>,
    pub record_version: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub is_delete: bool,
    pub delete_at: Option<NaiveDateTime>,
    pub comment: Option<String>,
    pub total_fee: Option<BigDecimal>,
    pub lift_fee: Option<BigDecimal>,
    pub total_charge: BigDecimal,
}


type BoxedQuery<'a> = t_property_fee_detail::BoxedQuery<'a,Pg,crate::SqlType<PropertyFeeDetailPo>>;
impl PropertyFeeDetailPo{
    fn all<'a>() -> BoxedQuery<'a>{
        table.select(PropertyFeeDetailPo::as_select()).into_boxed()
    }
    pub fn by_id<'a>(p_id:i64) -> BoxedQuery<'a> {
        Self::all().filter(with_id_filter(p_id)).filter(with_data_enable())
    }

    ///
    /// 根据roomNumber和version获取可用的物业费明细
    ///
    pub fn by_room_number_and_version (p_room_number: &str, p_version: &str,conn:&mut Conn) -> AppResult<PropertyFeeDetailPo>{
        let result = Self::all()
            .filter(room_number.eq(p_room_number))
            .filter(record_version.eq(p_version))
            .filter(is_delete.eq(false))
            .first(conn)?;
        Ok(result)
    }
    ///
    /// 增量查询
    ///
    pub fn by_room_number_flow(p_room_number: Option<Vec<&str>>, offset: i64, limit: i64) -> AppResult<Vec<PropertyFeeDetailPo>> {
        if let Some(p_room_number) = p_room_number {
            let result = Self::all()
                .filter(room_number.eq_any(p_room_number))
                .filter(is_delete.eq(false))
                .offset(offset)
                .limit(limit)
                .order_by(create_time.desc())
                .get_results(&mut db_get_connection())?;
            Ok(result)
        } else {
            Ok(vec![])
        }
    }

    pub fn by_version (p_version:&str,conn:&mut Conn) -> AppResult<Vec<PropertyFeeDetailPo>>{
        let result = Self::all()
            .filter(record_version.eq(p_version))
            .filter(is_delete.eq(false))
            .get_results(conn)?;
        Ok(result)
    }

    ///
    /// tips：这里的计算不会算上预存
    /// (废弃)
    ///
    // #[deprecated(note = "这里的计算不会算上预存")]
    pub fn fee_calculate(&self) -> BigDecimal {
        let mut v_total_fee = BigDecimal::from(0);
        if let Some(ref v_management_fee) = self.management_fee {
            v_total_fee += v_management_fee;
        }
        if let Some(ref v_part_fee) = self.part_fee {
            v_total_fee += v_part_fee;
        }
        if let Some(ref v_lift_fee) = self.lift_fee {
            v_total_fee += v_lift_fee;
        }
        if let Some(ref v_machine_room_renovation_fee) = self.machine_room_renovation_fee {
            v_total_fee += v_machine_room_renovation_fee;
        }
        if let Some(ref v_electric_fee) = self.electric_fee {
            v_total_fee += v_electric_fee;
        }
        if let Some(ref v_electric_share_fee) = self.electric_share_fee {
            v_total_fee += v_electric_share_fee;
        }
        if let Some(ref v_water_fee) = self.water_fee {
            v_total_fee += v_water_fee;
        }
        if let Some(ref v_water_share_fee) = self.water_share_fee {
            v_total_fee += v_water_share_fee;
        }
        if let Some(ref v_liquidate_fee) = self.liquidate_fee {
            v_total_fee += v_liquidate_fee;
        }
        //如果小于0说明预存是足够的，就把账单变为0也就是total
        v_total_fee
    }

}

#[derive(Identifiable, AsChangeset, AutoOperation)]
#[diesel(table_name = t_property_fee_detail)]
pub struct PropertyFeeDetailUpdatePo<'a> {
    pub id: i64,
    pub management_fee: Option<&'a BigDecimal>,
    pub part_fee: Option<&'a BigDecimal>,
    pub machine_room_renovation_fee: Option<&'a BigDecimal>,
    pub lift_fee: Option<&'a BigDecimal>,
    pub electric_fee: Option<&'a BigDecimal>,
    pub electric_share_fee: Option<&'a BigDecimal>,
    pub water_fee: Option<&'a BigDecimal>,
    pub water_share_fee: Option<&'a BigDecimal>,
    pub liquidate_fee: Option<&'a BigDecimal>,
    pub pre_store_fee: Option<&'a BigDecimal>,
    pub update_by: Option<&'a str>,
    pub update_time: Option<NaiveDateTime>,
    pub is_delete: Option<bool>,
    pub delete_at: Option<NaiveDateTime>,
    pub comment: Option<&'a str>,
    pub total_fee: Option<BigDecimal>,
    pub total_charge: Option<BigDecimal>,
}
impl <'c> PropertyFeeDetailUpdatePo<'c>{
    pub fn update<'a, 'b>(&'a mut self, update_po: PropertyFeeDetailUpdatePo<'b>)
    where
        'b: 'a,
        'b: 'c,
    {
        if update_po.management_fee.is_some() {
            self.management_fee = update_po.management_fee;
        }
        if update_po.part_fee.is_some() {
            self.part_fee = update_po.part_fee;
        }
        if update_po.machine_room_renovation_fee.is_some() {
            self.machine_room_renovation_fee = update_po.machine_room_renovation_fee;
        }
        if update_po.lift_fee.is_some() {
            self.lift_fee = update_po.lift_fee;
        }
        if update_po.electric_fee.is_some() {
            self.electric_fee = update_po.electric_fee;
        }
        if update_po.electric_share_fee.is_some() {
            self.electric_share_fee = update_po.electric_share_fee;
        }
        if update_po.water_fee.is_some() {
            self.water_fee = update_po.water_fee;
        }
        if update_po.water_share_fee.is_some() {
            self.water_share_fee = update_po.water_share_fee;
        }
        if update_po.liquidate_fee.is_some() {
            self.liquidate_fee = update_po.liquidate_fee;
        }
        if update_po.pre_store_fee.is_some() {
            self.pre_store_fee = update_po.pre_store_fee;
        }
        if update_po.comment.is_some() {
            self.comment = update_po.comment;
        }
    }
}
impl<'a> From<&'a PropertyFeeDetailPo> for PropertyFeeDetailUpdatePo<'a> {
    fn from(po: &'a PropertyFeeDetailPo) -> Self {
        Self {
            id: po.id,
            management_fee: po.management_fee.as_ref(),
            part_fee: po.part_fee.as_ref(),
            machine_room_renovation_fee: po.machine_room_renovation_fee.as_ref(),
            lift_fee: po.lift_fee.as_ref(),
            electric_fee: po.electric_fee.as_ref(),
            electric_share_fee: po.electric_share_fee.as_ref(),
            water_fee: po.water_fee.as_ref(),
            water_share_fee: po.water_share_fee.as_ref(),
            liquidate_fee: po.liquidate_fee.as_ref(),
            pre_store_fee: po.pre_store_fee.as_ref(),
            update_by: Some(&po.update_by),
            update_time: Some(po.update_time),
            is_delete: Some(po.is_delete),
            delete_at: po.delete_at,
            comment: po.comment.as_deref(),
            total_fee: po.total_fee.clone(),
            total_charge: Some(po.total_charge.clone())
        }
    }
}

impl FeeCalculator for PropertyFeeDetailUpdatePo<'_> {
    fn fee_calculate(&mut self) {
        let mut v_total_fee = BigDecimal::from(0);
        if let Some(v_management_fee) = self.management_fee {
            v_total_fee += v_management_fee;
        }
        if let Some(v_part_fee) = self.part_fee {
            v_total_fee += v_part_fee;
        }
        if let Some(v_lift_fee) = self.lift_fee {
            v_total_fee += v_lift_fee;
        }
        if let Some(v_machine_room_renovation_fee) = self.machine_room_renovation_fee {
            v_total_fee += v_machine_room_renovation_fee;
        }
        if let Some(v_electric_fee) = self.electric_fee {
            v_total_fee += v_electric_fee;
        }
        if let Some(v_electric_share_fee) = self.electric_share_fee {
            v_total_fee += v_electric_share_fee;
        }
        if let Some(v_water_fee) = self.water_fee {
            v_total_fee += v_water_fee;
        }
        if let Some(v_water_share_fee) = self.water_share_fee {
            v_total_fee += v_water_share_fee;
        }
        if let Some(v_liquidate_fee) = self.liquidate_fee {
            v_total_fee += v_liquidate_fee;
        }
        self.total_fee = Some(v_total_fee.clone());

        if let Some(v_pre_store_fee) = self.pre_store_fee {
            v_total_fee += v_pre_store_fee;
        }
        self.total_charge = Some(v_total_fee);
    }
}

#[derive(Insertable, AutoOperation)]
#[diesel(table_name = t_property_fee_detail)]
pub struct PropertyFeeDetailInsertPo<'a> {
    pub room_number: &'a str,
    pub room_owner_name: Option<&'a str>,
    pub management_fee: Option<BigDecimal>,
    pub part_fee: Option<BigDecimal>,
    pub lift_fee: Option<BigDecimal>,
    pub machine_room_renovation_fee: Option<BigDecimal>,
    pub electric_fee: Option<BigDecimal>,
    pub electric_share_fee: Option<BigDecimal>,
    pub water_fee: Option<BigDecimal>,
    pub water_share_fee: Option<BigDecimal>,
    pub liquidate_fee: Option<BigDecimal>,
    pub pre_store_fee: Option<BigDecimal>,
    pub record_version: &'a str,
    pub create_by: &'a str,
    pub update_by: &'a str,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_delete: bool,
    pub delete_at: Option<NaiveDateTime>,
    pub comment: Option<&'a str>,
    pub total_fee: Option<BigDecimal>,
    pub total_charge: Option<BigDecimal>,
}

impl PropertyFeeDetailInsertPo<'_>{
    pub fn fee_calculate(&mut self) {
        let mut v_total_fee = BigDecimal::from(0);
        if let Some(ref v_management_fee) = self.management_fee {
            v_total_fee += v_management_fee;
        }
        if let Some(ref v_part_fee) = self.part_fee {
            v_total_fee += v_part_fee;
        }
        if let Some(ref v_lift_fee) = self.lift_fee {
            v_total_fee += v_lift_fee;
        }
        if let Some(ref v_machine_room_renovation_fee) = self.machine_room_renovation_fee {
            v_total_fee += v_machine_room_renovation_fee;
        }
        if let Some(ref v_electric_fee) = self.electric_fee {
            v_total_fee += v_electric_fee;
        }
        if let Some(ref v_electric_share_fee) = self.electric_share_fee {
            v_total_fee += v_electric_share_fee;
        }
        if let Some(ref v_water_fee) = self.water_fee {
            v_total_fee += v_water_fee;
        }
        if let Some(ref v_water_share_fee) = self.water_share_fee {
            v_total_fee += v_water_share_fee;
        }
        if let Some(ref v_liquidate_fee) = self.liquidate_fee {
            v_total_fee += v_liquidate_fee;
        }
        self.total_fee = Some(v_total_fee.clone());

        if let Some(ref v_pre_store_fee) = self.pre_store_fee {
            v_total_fee += v_pre_store_fee;
        }
        self.total_charge = Some(v_total_fee);
    }
}
