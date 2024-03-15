use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use std::collections::HashMap;

#[cw_serde]
pub struct Config {
    pub token_address: Addr,
    pub token_distribute_address: Addr,
    pub total_distribute_amount: Uint128,
    pub user_register_amount: Uint128,
}
#[cw_serde]
pub struct PeriodConfig {
    pub period_id: u64,
    pub period_claimed_time: u64,
    pub period_claimed_amount: Uint128,
    pub period_total_amount: Uint128,
    pub claimed_from_distribute: bool,
}

#[cw_serde]
pub struct UserPeriodConfig {
    pub user_per_period_amount: Uint128,
    pub user_total_claimed_amount: Uint128,
    pub user_total_amount: Uint128,
    pub user_claimed_periods: HashMap<u64, UserPeriodClaimedDetails>,
}

#[cw_serde]
pub struct UserPeriodClaimedDetails {
    pub claimed_amount: Uint128,
    pub claimed_time: u64,
}

const CONFIG: Item<Config> = Item::new("config");
const PERIOD_CONFIGS: Map<&u64, PeriodConfig> = Map::new("period_configs");

const USER_PERIOD_CONFIGS: Map<&Addr, UserPeriodConfig> = Map::new("user_period_configs");

const USER_STATUS: Map<&Addr, bool> = Map::new("user_status");

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    CONFIG.save(storage, config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

pub fn store_period_config(
    storage: &mut dyn Storage,
    period_config: &PeriodConfig,
) -> StdResult<()> {
    PERIOD_CONFIGS.save(storage, &period_config.period_id, period_config)
}

pub fn read_period_config(storage: &dyn Storage, period_id: &u64) -> StdResult<PeriodConfig> {
    PERIOD_CONFIGS.load(storage, period_id)
}

pub fn has_period_config(storage: &dyn Storage, period_id: &u64) -> bool {
    PERIOD_CONFIGS.has(storage, period_id)
}

// Query all period configuration records, sorted in ascending order by period_id
pub fn read_all_period_config(storage: &dyn Storage) -> StdResult<Vec<PeriodConfig>> {
    let mut result: Vec<PeriodConfig> = Vec::new();
    for item in PERIOD_CONFIGS.range_raw(storage, None, None, cosmwasm_std::Order::Ascending) {
        let (_, value) = item?;
        result.push(value);
    }
    Ok(result)
}

pub fn store_user_period_config(
    storage: &mut dyn Storage,
    user_address: &Addr,
    user_period_config: &UserPeriodConfig,
) -> StdResult<()> {
    USER_PERIOD_CONFIGS.save(storage, user_address, user_period_config)
}

pub fn read_user_period_config(
    storage: &dyn Storage,
    user_address: &Addr,
) -> StdResult<UserPeriodConfig> {
    USER_PERIOD_CONFIGS.load(storage, user_address)
}

pub fn has_user_period_config(storage: &dyn Storage, user_address: &Addr) -> bool {
    USER_PERIOD_CONFIGS.has(storage, user_address)
}

pub fn store_user_status(
    storage: &mut dyn Storage,
    user_address: &Addr,
    status: bool,
) -> StdResult<()> {
    USER_STATUS.save(storage, user_address, &status)
}

pub fn read_user_status(storage: &dyn Storage, user_address: &Addr) -> StdResult<bool> {
    USER_STATUS
        .may_load(storage, user_address)
        .map(|v| v.unwrap_or(false))
}

pub fn has_user_status(storage: &dyn Storage, user_address: &Addr) -> bool {
    USER_STATUS.has(storage, user_address)
}
