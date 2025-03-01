use serde::{Deserialize, Serialize};

use super::{bond::HumanBond, etf::HumanEtf, future::HumanFuture, share::HumanShare};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HumanInstrument {
    Share(HumanShare),
    Bond(HumanBond),
    Etf(HumanEtf),
    Future(HumanFuture),
}
