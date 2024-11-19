use crate::const_value::SETTINGS;
use bigdecimal::BigDecimal;
use validator::ValidationError;

pub fn validate_big_decimal(value: &BigDecimal) -> Result<(), ValidationError> {
    if value > &BigDecimal::from(SETTINGS.app_config.number_max) {
        let error = ValidationError::new("value is more than max");
        return Err(error);
    }
    if value < &BigDecimal::from(SETTINGS.app_config.number_min) {
        let error = ValidationError::new("value is less than min");
        return Err(error);
    }
    Ok(())
}