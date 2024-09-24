use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BatchTransferConfig {
    pub gov: Addr,
    pub token: Addr,
    pub burn_fee_percent: Uint128,
    pub burn_total_burn: Uint128,
}

const BATCH_TRANSFER_CONFIG: Item<BatchTransferConfig> = Item::new("batch_transfer_config");

pub fn store_batch_transfer_config(storage: &mut dyn Storage, config: &BatchTransferConfig) -> StdResult<()> {
    BATCH_TRANSFER_CONFIG.save(storage, config)
}

pub fn read_batch_transfer_config(storage: &dyn Storage) -> StdResult<BatchTransferConfig> {
    BATCH_TRANSFER_CONFIG.load(storage)
}