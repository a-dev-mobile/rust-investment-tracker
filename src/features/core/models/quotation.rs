use serde::{Deserialize, Serialize};

use crate::gen::tinkoff_public_invest_api_contract_v1::Quotation;

/// Human-readable Quotation model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotationModel {
    pub units: i64,
    pub nano: i32,
    pub value: f64,
}

impl From<&Quotation> for QuotationModel {
    fn from(q: &Quotation) -> Self {
        let units = q.units;
        let nano = q.nano;
        let value = units as f64 + (nano as f64 / 1_000_000_000.0);

        Self { units, nano, value }
    }
}
