use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, StdError, StdResult, Storage};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct KptConfig {
    pub max_supply: u128,
    pub kpt_fund: Addr,
    pub gov: Addr,
    pub kpt_distribute:Addr,
}

const KPT_CONFIG: Item<KptConfig> = Item::new("kpt_config");

pub fn store_kpt_config(storage:&mut dyn Storage, kpt_config: &KptConfig) -> StdResult<()> {
    KPT_CONFIG.save(storage, kpt_config)
}

pub fn read_kpt_config(storage: &dyn Storage) -> StdResult<KptConfig> {
    KPT_CONFIG.load(storage).map_err(|_| StdError::generic_err("KptConfig not found"))
}