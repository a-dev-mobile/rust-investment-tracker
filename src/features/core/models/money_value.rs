use serde::{Deserialize, Serialize};

use crate::gen::tinkoff_public_invest_api_contract_v1::MoneyValue;

/// Human-readable MoneyValue model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanMoneyValue {
    pub currency: String,
    pub units: i64,
    pub nano: i32,
    pub value: f64,
    pub formatted: String,
}

impl From<&MoneyValue> for HumanMoneyValue {
    fn from(m: &MoneyValue) -> Self {
        let units = m.units;
        let nano = m.nano;
        let value = units as f64 + (nano as f64 / 1_000_000_000.0);
        
        Self {
            currency: m.currency.clone(),
            units,
            nano,
            value,
            formatted: format!("{:.9} {}", value, m.currency),
        }
    }
}
