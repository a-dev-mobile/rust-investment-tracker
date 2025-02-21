use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyMoneyValue {
    pub currency: String,
    pub units: i64,
    pub nano: i32,
}
