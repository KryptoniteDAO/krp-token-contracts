use cosmwasm_std::{Addr, StdResult, Storage, Uint128, Uint256};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MinerConfig{
    pub gov: Addr,
    pub kusd_denom: String,
    pub kusd_controller_addr: Addr,
    pub ve_kpt_boost_addr: Addr,
    pub kpt_fund_addr: Addr,
    pub ve_kpt_addr: Addr,
    pub reward_controller_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MinerState{
    // Duration of rewards to be paid out (in seconds) 2_592_000 = 30 days
    pub duration: Uint128,
    // Timestamp of when the rewards finish
    pub finish_at: Uint128,
    // Minimum of last updated time and reward finish time
    pub updated_at: Uint128,
    // Reward to be paid out per second
    pub reward_rate: Uint256,
    // Sum of (reward rate * dt * 1e6 / total supply)
    pub reward_per_token_stored: Uint128,
    // 50 * 1e6
    pub extra_rate: Uint128,
    // Currently, the official rebase time for Lido is between 12PM to 13PM UTC. 12 hours
    pub lockdown_period: Uint128,
}

const MINER_CONFIG: Item<MinerConfig> = Item::new("miner_config");
const MINER_STATE: Item<MinerState> = Item::new("miner_state");
// mapping(address => uint) public userRewardPerTokenPaid;
const USER_REWARD_PER_TOKEN_PAID: Map<Addr, Uint128> = Map::new("user_reward_per_token_paid");
const REWARDS: Map<Addr, Uint128> = Map::new("rewards");
const USER_UPDATED_AT: Map<Addr, Uint128> = Map::new("user_updated_at");
const IS_REDEMPTION_PROVIDER: Map<Addr, bool> = Map::new("is_redemption_provider");

pub fn store_user_reward_per_token_paid(storage: &mut dyn Storage, user: Addr, reward_per_token_paid: &Uint128) ->StdResult<()> {
    USER_REWARD_PER_TOKEN_PAID.save(storage, user, reward_per_token_paid)?;
    Ok(())
}

pub fn read_user_reward_per_token_paid(storage: &dyn Storage, user: Addr) -> Uint128 {
    USER_REWARD_PER_TOKEN_PAID.load(storage, user).unwrap_or(Uint128::zero())
}

pub fn store_rewards(storage: &mut dyn Storage, user: Addr, rewards: &Uint128) -> StdResult<()> {
    REWARDS.save(storage, user, rewards)?;
    Ok(())
}

pub fn read_rewards(storage: &dyn Storage, user: Addr) -> Uint128 {
    REWARDS.load(storage, user).unwrap_or(Uint128::zero())
}

pub fn store_user_updated_at(storage: &mut dyn Storage, user: Addr, updated_at: &Uint128) -> StdResult<()> {
    USER_UPDATED_AT.save(storage, user, updated_at)?;
    Ok(())
}

pub fn read_user_updated_at(storage: &dyn Storage, user: Addr) -> Uint128 {
    USER_UPDATED_AT.load(storage, user).unwrap_or(Uint128::zero())
}

pub fn store_miner_config(storage: &mut dyn Storage, config: &MinerConfig) -> StdResult<()> {
    MINER_CONFIG.save(storage, config)?;
    Ok(())
}

pub fn read_miner_config(storage: &dyn Storage) -> StdResult<MinerConfig> {
    MINER_CONFIG.load(storage)
}

pub fn store_miner_state(storage: &mut dyn Storage, state: &MinerState) -> StdResult<()> {
    MINER_STATE.save(storage, state)?;
    Ok(())
}

pub fn read_miner_state(storage: &dyn Storage) -> StdResult<MinerState> {
    MINER_STATE.load(storage)
}

#[allow(dead_code)]
pub fn store_is_redemption_provider(storage: &mut dyn Storage, user: Addr, is_redemption_provider: &bool) -> StdResult<()> {
    IS_REDEMPTION_PROVIDER.save(storage, user, is_redemption_provider)?;
    Ok(())
}

pub fn read_is_redemption_provider(storage: &dyn Storage, user: Addr) -> bool {
    IS_REDEMPTION_PROVIDER.load(storage, user).unwrap_or(false)
}


