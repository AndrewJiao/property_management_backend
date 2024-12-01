use crate::schema::basic::sql_types::CalculateOperation;
use crate::schema::basic::t_price_basic;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::prelude::*;
use management_macro::AutoOperation;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Selectable, Queryable, Serialize)]
#[diesel(table_name = t_price_basic)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct PriceBasicPo {
    id: i64,
    name: Option<String>,
    pub basic_number: Option<BigDecimal>,
    create_by: Option<String>,
    update_by: Option<String>,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
    is_delete: Option<bool>,
    operation_type: Option<CalculateOperationType>,
    comment: Option<String>,
    pub basic_code: Option<BasicPriceType>,
}

pub trait PriceBasicConfigGet {
    fn to_price_type_map(self) -> HashMap<BasicPriceType, PriceBasicPo>;
}
impl PriceBasicConfigGet for Vec<PriceBasicPo> {
    fn to_price_type_map(self) -> HashMap<BasicPriceType, PriceBasicPo> {
        self.into_iter().flat_map(|po| {
            match po.basic_code {
                None => { None }
                Some(ref base_type) => {
                    Some((base_type.clone(), po))
                }
            }
        }).collect::<HashMap<BasicPriceType, PriceBasicPo>>()
    }
}
use crate::schema::basic::t_price_basic::*;
type BoxedQuery<'a> = t_price_basic::BoxedQuery<'a, Pg, crate::SqlType<PriceBasicPo>>;
impl PriceBasicPo {
    pub fn all() -> BoxedQuery<'static> {
        table.select(PriceBasicPo::as_select()).into_boxed()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Clone)]
pub enum BasicPriceType {
    ManagementFee,
    CarPartFee,
    MotorCyclePartFee,
    MachineRoomRenovationFee,
    ElectricFee,
    ElectricShareFee,
    WaterFee,
    WaterShareFee,
    LiquidateFee,
    PreStoreFee,
}

impl FromSql<diesel::sql_types::Text, Pg> for BasicPriceType {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let str = std::str::from_utf8(bytes.as_bytes())?;
        match str {
            "ManagementFee" => { Ok(BasicPriceType::ManagementFee) }
            "CarPartFee" => { Ok(BasicPriceType::CarPartFee) }
            "MotorCyclePartFee" => { Ok(BasicPriceType::MotorCyclePartFee) }
            "MachineRoomRenovationFee" => { Ok(BasicPriceType::MachineRoomRenovationFee) }
            "ElectricFee" => { Ok(BasicPriceType::ElectricFee) }
            "ElectricShareFee" => { Ok(BasicPriceType::ElectricShareFee) }
            "WaterFee" => { Ok(BasicPriceType::WaterFee) }
            "WaterShareFee" => { Ok(BasicPriceType::WaterShareFee) }
            "LiquidateFee" => { Ok(BasicPriceType::LiquidateFee) }
            "PreStoreFee" => { Ok(BasicPriceType::PreStoreFee) }
            _ => { Err(Box::from(format!("Invalid BasicPriceType for type {}", str))) }
        }
    }
}

#[derive(Identifiable, AsChangeset, AutoOperation)]
#[diesel(table_name = t_price_basic)]
pub struct UpdatePriceBasicPo<'a> {
    pub id: i64,
    pub name: Option<&'a str>,
    pub basic_number: Option<&'a BigDecimal>,
    pub update_time: Option<NaiveDateTime>,
    pub comment: Option<&'a str>,
}

#[derive(Serialize)]
pub enum CalculateOperationType {
    Add,
    Sub,
    Mul,
    Div,
}


impl FromSql<CalculateOperation, Pg> for CalculateOperationType {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let str = std::str::from_utf8(bytes.as_bytes())?;
        match str {
            "add" => { Ok(CalculateOperationType::Add) }
            "subtract" => { Ok(CalculateOperationType::Sub) }
            "multiply" => { Ok(CalculateOperationType::Mul) }
            "divide" => { Ok(CalculateOperationType::Div) }
            _ => { Err(Box::from("Invalid CalculateOperationType")) }
        }
    }
}




