use cosmwasm_std::{Addr, Order, StdResult, Storage, Uint128, Uint64};
use cosmwasm_storage::{Bucket, ReadonlyBucket};
use cw_storage_plus::{Item, Map};
use cw_utils::calc_range_start_string;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureConfig {
    pub gov: Addr,
    pub lock_token: Addr,
    pub start_time: u64,
    pub end_time: u64,
    // Integral reward coefficient
    pub integral_reward_coefficient: Uint128,
    pub lock_duration: u64,
    // punish coefficient
    pub punish_coefficient: Uint128,
    // nft cost integral
    pub mint_nft_cost_integral: Uint128,
    pub winning_num: HashSet<u64>,
    pub mod_num: u64,
    // punish receiver
    pub punish_receiver: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureState {
    pub current_locked_amount: Uint128,
    pub current_integral_amount: Uint128,
    pub total_locked_amount: Uint128,
    pub total_withdraw_amount: Uint128,
    pub total_punish_amount: Uint128,
    pub total_win_nft_num: u64,
    pub total_lose_nft_num: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureUserState {
    pub current_locked_amount: Uint128,
    pub current_integral_amount: Uint128,

    pub win_nft_num: u64,
    pub lose_nft_num: u64,

    pub start_lock_time: u64,
    pub end_lock_time: u64,

    pub total_locked_amount: Uint128,
    pub total_withdraw_amount: Uint128,
    pub total_punish_amount: Uint128,
    pub total_cost_integral_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserLockRecord {
    pub record_id: u64,
    pub user_addr: Addr,
    pub lock_amount: Uint128,
    pub integral_reward_coefficient: Uint128,
    pub lock_duration: u64,
    pub start_lock_time: u64,
    pub end_lock_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserWithdrawRecord {
    pub record_id: u64,
    pub user_addr: Addr,
    pub withdraw_amount: Uint128,
    pub withdraw_time: u64,
    pub punish_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserMintNftRecord {
    pub record_id: u64,
    pub user_addr: Addr,
    pub mint_nft_num: u64,
    pub win_nft_num: u64,
    pub mint_nft_cost_integral_amount: Uint128,
    pub mint_time: u64,
}

const TREASURE_CONFIG: Item<TreasureConfig> = Item::new("treasure_config");

const TREASURE_STATE: Item<TreasureState> = Item::new("treasure_state");

const TREASURE_USER_STATE: Map<Addr, TreasureUserState> = Map::new("treasure_user_state");

const RECORD_INDEX: Item<Uint64> = Item::new("record_index");

pub fn store_treasure_config(storage: &mut dyn Storage, data: &TreasureConfig) -> StdResult<()> {
    TREASURE_CONFIG.save(storage, data)
}

pub fn read_treasure_config(storage: &dyn Storage) -> StdResult<TreasureConfig> {
    TREASURE_CONFIG.load(storage)
}

pub fn store_treasure_state(storage: &mut dyn Storage, data: &TreasureState) -> StdResult<()> {
    TREASURE_STATE.save(storage, data)
}

pub fn read_treasure_state(storage: &dyn Storage) -> StdResult<TreasureState> {
    TREASURE_STATE.load(storage)
}

pub fn store_treasure_user_state(
    storage: &mut dyn Storage,
    user_addr: &Addr,
    user_state: &TreasureUserState,
) -> StdResult<()> {
    TREASURE_USER_STATE.save(storage, user_addr.clone(), user_state)
}

pub fn read_treasure_user_state(
    storage: &dyn Storage,
    user_addr: &Addr,
) -> StdResult<TreasureUserState> {
    TREASURE_USER_STATE
        .load(storage, user_addr.clone())
        .map_or_else(
            |_| {
                Ok(TreasureUserState {
                    current_locked_amount: Uint128::zero(),
                    current_integral_amount: Uint128::zero(),
                    win_nft_num: 0,
                    lose_nft_num: 0,
                    start_lock_time: 0,
                    end_lock_time: 0,
                    total_locked_amount: Uint128::zero(),
                    total_withdraw_amount: Uint128::zero(),
                    total_punish_amount: Uint128::zero(),
                    total_cost_integral_amount: Uint128::zero(),
                })
            },
            |user_state| Ok(user_state),
        )
}

pub fn generate_next_record_id(storage: &mut dyn Storage) -> StdResult<u64> {
    let mut record_index = RECORD_INDEX.load(storage).unwrap_or(Uint64::zero());
    record_index += Uint64::one();
    RECORD_INDEX.save(storage, &record_index)?;
    Ok(record_index.u64())
}

pub fn store_user_lock_record(
    storage: &mut dyn Storage,
    user_lock_record: &UserLockRecord,
) -> StdResult<()> {
    let user_addr = user_lock_record.user_addr.clone();
    let binding = user_addr.clone().to_string();
    let namespace = &format!("{}_lock", binding);
    let key = user_lock_record.record_id.clone().to_be_bytes();
    Bucket::new(storage, namespace.as_bytes()).save(&key, user_lock_record)
}

pub fn read_user_lock_records(
    storage: &dyn Storage,
    user_addr: &Addr,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<UserLockRecord>> {
    let binding = user_addr.clone().to_string();
    let namespace = &format!("{}_lock", binding);
    let record_bucket: ReadonlyBucket<UserLockRecord> =
        ReadonlyBucket::new(storage, namespace.as_bytes());

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = calc_range_start_string(start_after);
    record_bucket
        .range(start.as_deref(), None, Order::Descending)
        .take(limit)
        .map(|elem| {
            let (_, v) = elem?;
            Ok(UserLockRecord {
                record_id: v.record_id,
                user_addr: v.user_addr,
                lock_amount: v.lock_amount,
                integral_reward_coefficient: v.integral_reward_coefficient,
                lock_duration: v.lock_duration,
                start_lock_time: v.start_lock_time,
                end_lock_time: v.end_lock_time,
            })
        })
        .collect()
}

pub fn store_user_withdraw_record(
    storage: &mut dyn Storage,
    user_withdraw_record: &UserWithdrawRecord,
) -> StdResult<()> {
    let user_addr = user_withdraw_record.user_addr.clone();
    let binding = user_addr.clone().to_string();
    let namespace = &format!("{}_withdraw", binding);
    let key = user_withdraw_record.record_id.clone().to_be_bytes();
    Bucket::new(storage, namespace.as_bytes()).save(&key, user_withdraw_record)
}

pub fn read_user_withdraw_records(
    storage: &dyn Storage,
    user_addr: &Addr,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<UserWithdrawRecord>> {
    let binding = user_addr.clone().to_string();
    let namespace = &format!("{}_withdraw", binding);
    let record_bucket: ReadonlyBucket<UserWithdrawRecord> =
        ReadonlyBucket::new(storage, namespace.as_bytes());

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = calc_range_start_string(start_after);
    record_bucket
        .range(start.as_deref(), None, Order::Descending)
        .take(limit)
        .map(|elem| {
            let (_, v) = elem?;
            Ok(UserWithdrawRecord {
                record_id: v.record_id,
                user_addr: v.user_addr,
                withdraw_amount: v.withdraw_amount,
                withdraw_time: v.withdraw_time,
                punish_amount: v.punish_amount,
            })
        })
        .collect()
}

pub fn store_user_mint_nft_record(
    storage: &mut dyn Storage,
    user_nft_record: &UserMintNftRecord,
) -> StdResult<()> {
    let user_addr = user_nft_record.user_addr.clone();
    let binding = user_addr.clone().to_string();
    let namespace = &format!("{}_mint_nft", binding);
    let key = user_nft_record.record_id.clone().to_be_bytes();
    Bucket::new(storage, namespace.as_bytes()).save(&key, user_nft_record)
}

pub fn read_user_mint_nft_records(
    storage: &dyn Storage,
    user_addr: &Addr,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<UserMintNftRecord>> {
    let binding = user_addr.clone().to_string();
    let namespace = &format!("{}_mint_nft", binding);
    let record_bucket: ReadonlyBucket<UserMintNftRecord> =
        ReadonlyBucket::new(storage, namespace.as_bytes());

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = calc_range_start_string(start_after);
    record_bucket
        .range(start.as_deref(), None, Order::Descending)
        .take(limit)
        .map(|elem| {
            let (_, v) = elem?;
            Ok(UserMintNftRecord {
                record_id: v.record_id,
                user_addr: v.user_addr,
                mint_nft_num: v.mint_nft_num,
                win_nft_num: v.win_nft_num,
                mint_time: v.mint_time,
                mint_nft_cost_integral_amount: v.mint_nft_cost_integral_amount,
            })
        })
        .collect()
}
