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
    pub reward_amount: u128,
    pub level_total_claimed_amount: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardTokenConfig {
    pub total_reward_amount: u128,
    pub total_claimed_amount: u128,
    pub total_claimed_count: u128,
    pub claimable_time: u64,
    pub reward_levels: Vec<RewardLevelConfig>,
}

const BLIND_BOX_REWARD_CONFIG: Item<BlindBoxRewardConfig> = Item::new("blind_box_reward_config");

const BLIND_BOX_REWARD_TOKEN: Map<String, RewardTokenConfig> = Map::new("blind_box_reward_token_config");


pub fn store_blind_box_reward_config(storage: &mut dyn Storage, blind_box_reward_config: &BlindBoxRewardConfig) -> StdResult<()> {
    BLIND_BOX_REWARD_CONFIG.save(storage, blind_box_reward_config)?;
    Ok(())
}

pub fn read_blind_box_reward_config(storage: &dyn Storage) -> StdResult<BlindBoxRewardConfig> {
    BLIND_BOX_REWARD_CONFIG.load(storage)
}

pub fn store_blind_box_reward_token_config(storage: &mut dyn Storage, reward_token: String, reward_token_config: &RewardTokenConfig) -> StdResult<()> {
    BLIND_BOX_REWARD_TOKEN.save(storage, reward_token, reward_token_config)?;
    Ok(())
}

pub fn read_blind_box_reward_token_config(storage: &dyn Storage, reward_token: String) -> RewardTokenConfig {
    BLIND_BOX_REWARD_TOKEN.load(storage, reward_token).unwrap_or(RewardTokenConfig {
        total_reward_amount: 0,
        total_claimed_amount: 0,
        total_claimed_count: 0,
        claimable_time: 0,
        reward_levels: vec![],
    })
}

pub fn get_blind_box_reward_token_config_keys<'a>(storage: &'a dyn Storage) -> Box<dyn Iterator<Item=StdResult<String>> + 'a> {
    BLIND_BOX_REWARD_TOKEN.keys(storage,None,None,Order::Ascending)
}