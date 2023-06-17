use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, StdResult, Storage, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct KptFundConfig {
    pub gov: Addr,
    pub ve_kpt_addr: Addr,
    pub kpt_addr: Addr,
    pub kusd_denom: String,
    pub kusd_reward_addr: Addr,
    pub kusd_reward_total_amount: Uint128,
    pub kusd_reward_total_paid_amount: Uint128,
    // Sum of (reward rate * dt * 1e18 / total supply)
    pub reward_per_token_stored: Uint128,
    // uint256 immutable exitCycle = 30 days;
    pub exit_cycle: Uint64,
    // uint256 public claimAbleTime;
    pub claim_able_time: Uint64,
}

const KPT_FUND_CONFIG: Item<KptFundConfig> = Item::new("kpt_fund_config");
// User address => rewardPerTokenStored
// mapping(address => uint) public userRewardPerTokenPaid;
const USER_REWARD_PER_TOKEN_PAID: Map<Addr, Uint128> = Map::new("user_reward_per_token_paid");
// User address => rewards to be claimed
// mapping(address => uint) public rewards;
const REWARDS: Map<Addr, Uint128> = Map::new("rewards");

// mapping(address => uint) public time2fullRedemption;
const TIME2FULL_REDEMPTION: Map<Addr, Uint64> = Map::new("time2full_redemption");
// mapping(address => uint) public unstakeRate;
const UNSTAKE_RATE: Map<Addr, Uint128> = Map::new("unstake_rate");
// mapping(address => uint) public lastWithdrawTime;
const LAST_WITHDRAW_TIME: Map<Addr, Uint64> = Map::new("last_withdraw_time");

pub fn store_kpt_fund_config(storage: &mut dyn Storage, kpt_fund_config: &KptFundConfig) -> StdResult<()> {
    KPT_FUND_CONFIG.save(storage, kpt_fund_config)?;
    Ok(())
}

pub fn read_kpt_fund_config(storage: &dyn Storage) -> StdResult<KptFundConfig> {
    KPT_FUND_CONFIG.load(storage)
}

pub fn store_user_reward_per_token_paid(storage: &mut dyn Storage, user: Addr, reward_per_token_paid: &Uint128) -> StdResult<()> {
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

pub fn store_time2full_redemption(storage: &mut dyn Storage, user: Addr, time2full_redemption: &Uint64) -> StdResult<()> {
    TIME2FULL_REDEMPTION.save(storage, user, time2full_redemption)?;
    Ok(())
}

pub fn read_time2full_redemption(storage: &dyn Storage, user: Addr) -> Uint64 {
    TIME2FULL_REDEMPTION.load(storage, user).unwrap_or(Uint64::zero())
}

pub fn store_unstake_rate(storage: &mut dyn Storage, user: Addr, unstake_rate: &Uint128) -> StdResult<()> {
    UNSTAKE_RATE.save(storage, user, unstake_rate)?;
    Ok(())
}

pub fn read_unstake_rate(storage: &dyn Storage, user: Addr) -> Uint128 {
    UNSTAKE_RATE.load(storage, user).unwrap_or(Uint128::zero())
}

pub fn store_last_withdraw_time(storage: &mut dyn Storage, user: Addr, last_withdraw_time: &Uint64) -> StdResult<()> {
    LAST_WITHDRAW_TIME.save(storage, user, last_withdraw_time)?;
    Ok(())
}

pub fn read_last_withdraw_time(storage: &dyn Storage, user: Addr) -> Uint64 {
    LAST_WITHDRAW_TIME.load(storage, user).unwrap_or(Uint64::zero())
}