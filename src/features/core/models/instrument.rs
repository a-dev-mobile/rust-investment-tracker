use serde::{Deserialize, Serialize};

use super::{bond::BondModel, etf::EtfModel, future::FutureModel, share::ShareModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InstrumentEnum {
    Share(ShareModel),
    Bond(BondModel),
    Etf(EtfModel),
    Future(FutureModel),
}
