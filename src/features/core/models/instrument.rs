use serde::{Deserialize, Serialize};

use super::{
    bond::TinkoffBondModel, etf::TinkoffEtfModel, future::TinkoffFutureModel,
    share::TinkoffShareModel,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TinkoffInstrumentEnum {
    Share(TinkoffShareModel),
    Bond(TinkoffBondModel),
    Etf(TinkoffEtfModel),
    Future(TinkoffFutureModel),
}
