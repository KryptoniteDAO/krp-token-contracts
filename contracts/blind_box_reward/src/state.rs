use std::collections::HashMap;
use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardConfig {
    pub gov: Addr,
    pub nft_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BoxRewardConfig {
    pub box_reward_token: Addr,
    pub box_open_time: u64,

    pub ordinary_box_reward_level_config: HashMap<u8, OrdinaryBoxRewardLevelConfig>,

    pub box_reward_distribute_addr: Addr,
    pub box_reward_distribute_rule_type: String,
    pub global_reward_total_amount: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BoxRewardConfigState {
    pub ordinary_total_reward_amount: u128,
    pub ordinary_total_open_box_count: u64,
    pub ordinary_box_reward_level_config_state: HashMap<u8, OrdinaryBoxRewardLevelConfigState>,

    pub global_reward_claim_index: u128,
    pub global_reward_claim_total_amount: u128,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrdinaryBoxRewardLevelConfig {
    // nft box level index
    // pub in_box_level_index: u8,
    pub reward_amount: u128,
    pub max_reward_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrdinaryBoxRewardLevelConfigState {
    pub total_reward_amount: u128,
    pub total_open_box_count: u64,
}

// user reward info

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BoxOpenInfo {
    pub open_user: Addr,
    pub open_reward_amount: u128,
    pub open_box_time: u64,
    pub is_reward_box: bool,
    pub reward_claim_index: u128,
    pub reward_claimed_amount: u128,
    pub box_level_index: u8,
}

//RewardConfig
const REWARD_CONFIG: Item<RewardConfig> = Item::new("reward_config");

pub fn get_reward_config(storage: &dyn Storage) -> StdResult<RewardConfig> {
    REWARD_CONFIG.load(storage)
}

pub fn set_reward_config(storage: &mut dyn Storage, config: &RewardConfig) -> StdResult<()> {
    REWARD_CONFIG.save(storage, config)
}

//BoxRewardConfig
const BOX_REWARD_CONFIG: Item<BoxRewardConfig> = Item::new("box_reward_config");

pub fn get_box_reward_config(storage: &dyn Storage) -> StdResult<BoxRewardConfig> {
    BOX_REWARD_CONFIG.load(storage)
}

pub fn set_box_reward_config(storage: &mut dyn Storage, config: &BoxRewardConfig) -> StdResult<()> {
    BOX_REWARD_CONFIG.save(storage, config)
}

//BoxRewardConfigState
const BOX_REWARD_CONFIG_STATE: Item<BoxRewardConfigState> = Item::new("box_reward_config_state");

pub fn get_box_reward_config_state(storage: &dyn Storage) -> StdResult<BoxRewardConfigState> {
    BOX_REWARD_CONFIG_STATE.load(storage)
}

pub fn set_box_reward_config_state(storage: &mut dyn Storage, config: &BoxRewardConfigState) -> StdResult<()> {
    BOX_REWARD_CONFIG_STATE.save(storage, config)
}

//BoxOpenInfo
const BOX_OPEN_INFO: Map<String, BoxOpenInfo> = Map::new("box_open_info");

pub fn is_box_open(storage: &dyn Storage, key: String) -> StdResult<bool> {
    BOX_OPEN_INFO.may_load(storage, key).map(|v| v.is_some())
}

pub fn get_box_open_info(storage: &dyn Storage, key: String) -> StdResult<BoxOpenInfo> {
    let res = BOX_OPEN_INFO.load(storage, key).unwrap_or(BoxOpenInfo {
        open_user: Addr::unchecked(""),
        open_reward_amount: 0,
        open_box_time: 0,
        is_reward_box: false,
        reward_claim_index: 0,
        reward_claimed_amount: 0,
        box_level_index: 0
    });
    Ok(res)
}

pub fn set_box_open_info(storage: &mut dyn Storage, key: String, info: &BoxOpenInfo) -> StdResult<()> {
    BOX_OPEN_INFO.save(storage, key, info)
}