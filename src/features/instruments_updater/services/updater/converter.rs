use crate::features::core::models::{bond::HumanBond, share::HumanShare};
use mongodb::bson::{doc, Document};
use tracing::error;

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    pub(super) fn convert_share_to_document(&self, share: &crate::gen::tinkoff_public_invest_api_contract_v1::Share) -> Document {
        // Convert to the human-readable model first
        let human_share = HumanShare::from(share);

        // Convert to BSON Document using serde
        match mongodb::bson::to_document(&human_share) {
            Ok(doc) => doc,
            Err(e) => {
                error!(
                    "Failed to convert share {} to document: {}",
                    share.ticker, e
                );
                // Return empty document when conversion fails
                // This will be skipped during insertion
                doc! {}
            }
        }
    }

    pub(super) fn convert_bond_to_document(&self, bond: &crate::gen::tinkoff_public_invest_api_contract_v1::Bond) -> Document {
        // Convert to the human-readable model first
        let human_bond = HumanBond::from(bond);

        // Convert to BSON Document using serde
        match mongodb::bson::to_document(&human_bond) {
            Ok(doc) => doc,
            Err(e) => {
                error!(
                    "Failed to convert bond {} to document: {}",
                    bond.ticker, e
                );
                // Return empty document when conversion fails
                // This will be skipped during insertion
                doc! {}
            }
        }
    }
}