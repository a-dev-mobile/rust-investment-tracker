
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyQuotation {
    pub units: i64,
    pub nano: i32,
}
