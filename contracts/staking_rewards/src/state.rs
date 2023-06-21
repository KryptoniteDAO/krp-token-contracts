use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingConfig{
    pub gov: Addr,
    // Immutable variables for staking and rewards tokens
    pub staking_token: Addr,
    pub rewards_token: Addr,
    pub ve_kpt_boost: Addr,
    pub kpt_fund: Addr,
    pub reward_controller_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingState{
    // Duration of rewards to be paid out (in seconds) 2_592_000 = 30 days
    pub duration: Uint128,
    // Timestamp of when the rewards finish
    pub finish_at: Uint128,
    // Minimum of last updated time and reward finish time
    pub updated_at: Uint128,
    // Reward to be paid out per second
    pub reward_rate: Uint128,
    // Sum of (reward rate * dt * 1e6 / total supply)
    pub reward_per_token_stored: Uint128,

    pub total_supply: Uint128,
}

const STAKING_CONFIG: Item<StakingConfig> = Item::new("staking_config");
const STAKING_STATE: Item<StakingState> = Item::new("staking_state");

const USER_REWARD_PER_TOKEN_PAID: Map<Addr, Uint128> = Map::new("user_reward_per_token_paid");
const REWARDS: Map<Addr, Uint128> = Map::new("rewards");
const USER_UPDATED_AT: Map<Addr, Uint128> = Map::new("user_updated_at");
// User address => staked amount
const STAKING_USER_BALANCE_OF: Map<&Addr, Uint128> = Map::new("staking_user_balance_of");

pub fn store_user_reward_per_token_paid(storage: &mut dyn Storage, user: Addr, reward_per_token_paid: &Uint128) ->StdResult<()> {
    USER_REWARD_PER_TOKEN_PAID.save(storage, user, reward_per_token_paid)
}

pub fn read_user_reward_per_token_paid(storage: &dyn Storage, user: Addr) -> Uint128 {
    USER_REWARD_PER_TOKEN_PAID.load(storage, user).unwrap_or(Uint128::zero())
}

pub fn store_rewards(storage: &mut dyn Storage, user: Addr, rewards: &Uint128) -> StdResult<()> {
    REWARDS.save(storage, user, rewards)
}

pub fn read_rewards(storage: &dyn Storage, user: Addr) -> Uint128 {
    REWARDS.load(storage, user).unwrap_or(Uint128::zero())
}

pub fn store_user_updated_at(storage: &mut dyn Storage, user: Addr, updated_at: &Uint128) -> StdResult<()> {
    USER_UPDATED_AT.save(storage, user, updated_at)
}

pub fn read_user_updated_at(storage: &dyn Storage, user: Addr) -> Uint128 {
    USER_UPDATED_AT.load(storage, user).unwrap_or(Uint128::zero())
}

pub fn store_balance_of(storage: &mut dyn Storage, user: Addr, balance: &Uint128) -> StdResult<()> {
    STAKING_USER_BALANCE_OF.save(storage, &user, balance)
}

pub fn read_balance_of(storage: &dyn Storage, user: Addr) -> Uint128 {
    STAKING_USER_BALANCE_OF.load(storage, &user).unwrap_or(Uint128::zero())
}

pub fn store_staking_config(storage: &mut dyn Storage, config: &StakingConfig) -> StdResult<()> {
    STAKING_CONFIG.save(storage, config)?;
    Ok(())
}

pub fn read_staking_config(storage: &dyn Storage) -> StdResult<StakingConfig> {
    STAKING_CONFIG.load(storage)
}

pub fn store_staking_state(storage: &mut dyn Storage, state: &StakingState) -> StdResult<()> {
    STAKING_STATE.save(storage, state)
}

pub fn read_staking_state(storage: &dyn Storage) -> StdResult<StakingState> {
    STAKING_STATE.load(storage)
}
