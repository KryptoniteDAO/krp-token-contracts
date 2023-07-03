use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DistributeConfig {
    pub gov: Addr,
    pub total_amount: u128,
    pub distribute_token: Addr,
    pub rules_total_amount: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RuleConfig {
    pub rule_name: String,
    pub rule_owner: Addr,
    pub rule_total_amount: u128,
    pub start_release_amount: u128,
    pub lock_start_time: u64,
    pub lock_end_time: u64,
    pub start_linear_release_time: u64,
    pub end_linear_release_time: u64,
    pub unlock_linear_release_amount: u128,
    pub unlock_linear_release_time: u64,
    pub linear_release_per_second: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RuleConfigState {
    pub is_start_release: bool,
    pub released_amount: u128,
    pub claimed_amount: u128,
    pub last_claim_linear_release_time: u64,
}

const DISTRIBUTE_CONFIG: Item<DistributeConfig> = Item::new("distribute_config");


const RULE_CONFIG: Map<&str, RuleConfig> = Map::new("rule_config");

const RULE_CONFIG_STATE: Map<&str, RuleConfigState> = Map::new("rule_config_state");


pub fn store_distribute_config(storage: &mut dyn Storage, config: &DistributeConfig) -> StdResult<()> {
    DISTRIBUTE_CONFIG.save(storage, config)
}

pub fn read_distribute_config(storage: &dyn Storage) -> StdResult<DistributeConfig> {
    DISTRIBUTE_CONFIG.load(storage)
}

pub fn store_rule_config(storage: &mut dyn Storage, key: &str, data: &RuleConfig) -> StdResult<()> {
    RULE_CONFIG.save(storage, key, data)
}

pub fn check_rule_config_exist(storage: &dyn Storage, key: &str) -> StdResult<bool> {
    Ok(RULE_CONFIG.may_load(storage, key).unwrap().is_some())
}

pub fn read_rule_config(storage: &dyn Storage, key: &str) -> StdResult<RuleConfig> {
    RULE_CONFIG.load(storage, key)
}

pub fn store_rule_config_state(storage: &mut dyn Storage, key: &str, data: &RuleConfigState) -> StdResult<()> {
    RULE_CONFIG_STATE.save(storage, key, data)
}

pub fn read_rule_config_state(storage: &dyn Storage, key: &str) -> StdResult<RuleConfigState> {
    RULE_CONFIG_STATE.load(storage, key)
}

