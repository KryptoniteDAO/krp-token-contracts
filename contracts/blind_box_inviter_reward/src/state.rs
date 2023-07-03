use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InviterRewardConfig {
    pub gov: Addr,
    pub nft_contract: Addr,
    pub reward_native_token: String,
    pub start_mint_box_time: u64,
    pub end_mint_box_time: u64,
    pub start_claim_token_time: u64,
    pub end_claim_token_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InviterRewardConfigState {
    pub total_mint_box_count: u32,
    pub total_claim_token_quantity: u128,
    pub mint_box_level_detail: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InviterOptDetail {
    pub mint_box_count: u32,
    pub claim_token_quantity: u128,
    pub mint_box_level_detail: HashMap<u8, u32>,
}

const INVITER_REWARD_CONFIG: Item<InviterRewardConfig> = Item::new("inviter_reward_config");

const INVITER_REWARD_CONFIG_STATE: Item<InviterRewardConfigState> = Item::new("inviter_reward_config_state");

const INVITER_OPT_DETAIL: Map<&Addr, InviterOptDetail> = Map::new("inviter_opt_detail");

pub fn store_inviter_reward_config(storage: &mut dyn Storage, data: &InviterRewardConfig) -> StdResult<()> {
    INVITER_REWARD_CONFIG.save(storage, data)
}

pub fn read_inviter_reward_config(storage: &dyn Storage) -> StdResult<InviterRewardConfig> {
    INVITER_REWARD_CONFIG.load(storage)
}

pub fn store_inviter_reward_config_state(storage: &mut dyn Storage, data: &InviterRewardConfigState) -> StdResult<()> {
    INVITER_REWARD_CONFIG_STATE.save(storage, data)
}

pub fn read_inviter_reward_config_state(storage: &dyn Storage) -> StdResult<InviterRewardConfigState> {
    INVITER_REWARD_CONFIG_STATE.load(storage)
}

pub fn store_inviter_opt_detail(storage: &mut dyn Storage, key: &Addr, data: &InviterOptDetail) -> StdResult<()> {
    INVITER_OPT_DETAIL.save(storage, key, data)
}

pub fn read_inviter_opt_detail(storage: &dyn Storage, key: &Addr) -> StdResult<InviterOptDetail> {
    let res = INVITER_OPT_DETAIL.load(storage, key).unwrap_or(InviterOptDetail {
        mint_box_count: 0,
        claim_token_quantity: 0,
        mint_box_level_detail: HashMap::new(),
    });
    Ok(res)
}

