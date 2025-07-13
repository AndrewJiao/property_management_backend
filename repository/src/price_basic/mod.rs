use crate::schema::public::sql_types::CalculateOperation;
use crate::schema::public::t_price_basic;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::prelude::*;
use management_macro::AutoOperation;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use common::data_result::AppResult;
use common::db_config::db_get_connection;

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
use crate::schema::public::t_price_basic::*;
type BoxedQuery<'a> = t_price_basic::BoxedQuery<'a, Pg, crate::SqlType<PriceBasicPo>>;
impl PriceBasicPo {
    pub fn all() -> BoxedQuery<'static> {
        table.select(PriceBasicPo::as_select()).into_boxed()
    }

    pub fn with_price_type(price_type: BasicPriceType) -> AppResult<PriceBasicPo> {
        let result = table.filter(basic_code.eq(price_type.to_string())).select(PriceBasicPo::as_select())
            .first::<PriceBasicPo>(&mut db_get_connection())?;
        Ok(result)
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
    LiftFeeBasic,
    LiftFeePlus,
    BusinessWaterFee,
    BusinessElectricFee,
    BusinessWaterShareFee,
    BusinessElectricShareFee,
    ElectronCarPartFee,
    BusinessManageFee
}
impl Display for BasicPriceType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BasicPriceType::ManagementFee => { "ManagementFee".to_string() }
            BasicPriceType::CarPartFee => { "CarPartFee".to_string() }
            BasicPriceType::MotorCyclePartFee => { "MotorCyclePartFee".to_string() }
            BasicPriceType::MachineRoomRenovationFee => { "MachineRoomRenovationFee".to_string() }
            BasicPriceType::ElectricFee => { "ElectricFee".to_string() }
            BasicPriceType::ElectricShareFee => { "ElectricShareFee".to_string() }
            BasicPriceType::WaterFee => { "WaterFee".to_string() }
            BasicPriceType::WaterShareFee => { "WaterShareFee".to_string() }
            BasicPriceType::LiquidateFee => { "LiquidateFee".to_string() }
            BasicPriceType::PreStoreFee => { "PreStoreFee".to_string() }
            BasicPriceType::LiftFeeBasic => { "LiftFeeBasic".to_string() }
            BasicPriceType::LiftFeePlus => { "LiftFeePlus".to_string() }
            BasicPriceType::BusinessWaterFee => { "BusinessWaterFee".to_string() }
            BasicPriceType::BusinessElectricFee => { "BusinessElectricFee".to_string() }
            BasicPriceType::BusinessWaterShareFee => { "BusinessWaterShareFee".to_string() }
            BasicPriceType::BusinessElectricShareFee => { "BusinessElectricShareFee".to_string() }
            BasicPriceType::ElectronCarPartFee => { "ElectronCarPartFee".to_string() }
            BasicPriceType::BusinessManageFee => { "BusinessManageFee".to_string() }
        };
        write!(f, "{}", str)
    }
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
            "LiftFeeBasic" => { Ok(BasicPriceType::LiftFeeBasic) }
            "LiftFeePlus" => { Ok(BasicPriceType::LiftFeePlus) }
            "BusinessWaterFee" => { Ok(BasicPriceType::BusinessWaterFee) }
            "BusinessElectricFee" => { Ok(BasicPriceType::BusinessElectricFee) }
            "BusinessWaterShareFee" => { Ok(BasicPriceType::BusinessWaterShareFee) }
            "BusinessElectricShareFee" => { Ok(BasicPriceType::BusinessElectricShareFee) }
            "ElectronCarPartFee" => { Ok(BasicPriceType::ElectronCarPartFee) }
            "BusinessManageFee" => { Ok(BasicPriceType::BusinessManageFee) }
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




