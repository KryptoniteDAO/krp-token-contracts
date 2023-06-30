use cosmwasm_std::{Addr, Order, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlindBoxRewardConfig {
    pub gov: Addr,
    pub nft_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardLevelConfig {
    pub reward_level_index: u8,
    pub reward_amount: u128,
    pub is_random_box: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardLevelConfigState {
    pub reward_level_index: u8,
    pub level_total_claimed_amount: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardTokenConfig {
    pub total_reward_amount: u128,
    pub claimable_time: u64,
    pub reward_levels: Vec<RewardLevelConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardTokenConfigState {
    pub total_claimed_amount: u128,
    pub total_claimed_count: u128,
    pub reward_levels: Vec<RewardLevelConfigState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RandomBoxRewardLevelConfig {
    pub random_level_index: u8,
    pub random_from_mod: u128,
    pub random_to_mod: u128,
    pub random_total_count: u128,
    pub random_amount: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RandomBoxRewardLevelState {
    pub random_level_index: u8,
    pub random_opened_count: u128,
    pub random_total_claimed_amount: u128,
}

const BLIND_BOX_REWARD_CONFIG: Item<BlindBoxRewardConfig> = Item::new("blind_box_reward_config");

// key reward token => value RewardTokenConfig
const BLIND_BOX_REWARD_TOKEN: Map<String, RewardTokenConfig> = Map::new("blind_box_reward_token_config");

// key reward token => value RewardTokenConfigState
const BLIND_BOX_REWARD_TOKEN_STATE: Map<String, RewardTokenConfigState> = Map::new("blind_box_reward_token_state");

// key reward token => value Vec<RandomBoxRewardLevelConfig>
const RANDOM_BOX_REWARD_LEVEL_CONFIG: Map<String, Vec<RandomBoxRewardLevelConfig>> = Map::new("random_box_reward_level_config");

// key reward token => value Vec<RandomBoxRewardLevelState>
const RANDOM_BOX_REWARD_LEVEL_STATE: Map<String, Vec<RandomBoxRewardLevelState>> = Map::new("random_box_reward_level_state");


pub fn store_blind_box_reward_config(storage: &mut dyn Storage, blind_box_reward_config: &BlindBoxRewardConfig) -> StdResult<()> {
    BLIND_BOX_REWARD_CONFIG.save(storage, blind_box_reward_config)
}

pub fn read_blind_box_reward_config(storage: &dyn Storage) -> StdResult<BlindBoxRewardConfig> {
    BLIND_BOX_REWARD_CONFIG.load(storage)
}

pub fn store_blind_box_reward_token_config(storage: &mut dyn Storage, reward_token: String, reward_token_config: &RewardTokenConfig) -> StdResult<()> {
    BLIND_BOX_REWARD_TOKEN.save(storage, reward_token, reward_token_config)
}

pub fn read_blind_box_reward_token_config(storage: &dyn Storage, reward_token: String) -> StdResult<RewardTokenConfig> {
    BLIND_BOX_REWARD_TOKEN.load(storage, reward_token)
}

pub fn get_blind_box_reward_token_config_keys<'a>(storage: &'a dyn Storage) -> Box<dyn Iterator<Item=StdResult<String>> + 'a> {
    BLIND_BOX_REWARD_TOKEN.keys(storage, None, None, Order::Ascending)
}

pub fn store_blind_box_reward_token_state(storage: &mut dyn Storage, reward_token: String, reward_token_config_state: &RewardTokenConfigState) -> StdResult<()> {
    BLIND_BOX_REWARD_TOKEN_STATE.save(storage, reward_token, reward_token_config_state)
}

pub fn read_blind_box_reward_token_state(storage: &dyn Storage, reward_token: String) -> StdResult<RewardTokenConfigState> {
    let res = BLIND_BOX_REWARD_TOKEN_STATE.load(storage, reward_token).unwrap_or(RewardTokenConfigState {
        total_claimed_amount: 0,
        total_claimed_count: 0,
        reward_levels: vec![]
    })?;
    Ok(res)
}

pub fn get_blind_box_reward_token_state_keys<'a>(storage: &'a dyn Storage) -> Box<dyn Iterator<Item=StdResult<String>> + 'a> {
    BLIND_BOX_REWARD_TOKEN_STATE.keys(storage, None, None, Order::Ascending)
}

pub fn store_random_box_reward_level_config(storage: &mut dyn Storage, reward_token: String, random_box_reward_level_config: &Vec<RandomBoxRewardLevelConfig>) -> StdResult<()> {
    RANDOM_BOX_REWARD_LEVEL_CONFIG.save(storage, reward_token, random_box_reward_level_config)
}

pub fn read_random_box_reward_level_config(storage: &dyn Storage, reward_token: String) -> StdResult<Vec<RandomBoxRewardLevelConfig>> {
    RANDOM_BOX_REWARD_LEVEL_CONFIG.load(storage, reward_token)
}

pub fn get_random_box_reward_level_config_keys<'a>(storage: &'a dyn Storage) -> Box<dyn Iterator<Item=StdResult<String>> + 'a> {
    RANDOM_BOX_REWARD_LEVEL_CONFIG.keys(storage, None, None, Order::Ascending)
}

pub fn store_random_box_reward_level_state(storage: &mut dyn Storage, reward_token: String, random_box_reward_level_state: &Vec<RandomBoxRewardLevelState>) -> StdResult<()> {
    RANDOM_BOX_REWARD_LEVEL_STATE.save(storage, reward_token, random_box_reward_level_state)
}

pub fn read_random_box_reward_level_state(storage: &dyn Storage, reward_token: String) -> StdResult<Vec<RandomBoxRewardLevelState>> {
    RANDOM_BOX_REWARD_LEVEL_STATE.load(storage, reward_token)
}

pub fn get_random_box_reward_level_state_keys<'a>(storage: &'a dyn Storage) -> Box<dyn Iterator<Item=StdResult<String>> + 'a> {
    RANDOM_BOX_REWARD_LEVEL_STATE.keys(storage, None, None, Order::Ascending)
}

