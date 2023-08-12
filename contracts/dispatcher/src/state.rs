use cosmwasm_std::{Addr, Order, StdResult, Storage, Uint256};
use cosmwasm_storage::{Bucket, ReadonlyBucket};
use cw_storage_plus::{Item, Map};
use cw_utils::calc_range_start;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalConfig {
    pub gov: Addr,
    pub claim_token: Addr,
    pub total_lock_amount: Uint256,
    pub start_lock_period_time: u64,
    pub duration_per_period: u64,
    pub periods: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalState {
    pub total_user_lock_amount: Uint256,
    pub total_user_claimed_lock_amount: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserState {
    pub user: Addr,
    pub total_user_lock_amount: Uint256,
    pub claimed_lock_amount: Uint256,

    pub last_claimed_period: u64,
    pub user_per_lock_amount: Uint256,
}

const GLOBAL_CONFIG: Item<GlobalConfig> = Item::new("global_config");

const GLOBAL_STATE: Item<GlobalState> = Item::new("global_state");

const USER_STATE_MAP: Map<Addr, UserState> = Map::new("user_state");

// const REGRET_INFO: Item<RegretInfo> = Item::new("regret_info");

const USER_PAGE_NAMESPACE: &[u8] = b"user_page_namespace";

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub fn store_global_config(storage: &mut dyn Storage, config: &GlobalConfig) -> StdResult<()> {
    GLOBAL_CONFIG.save(storage, config)
}

pub fn read_global_config(storage: &dyn Storage) -> StdResult<GlobalConfig> {
    GLOBAL_CONFIG.load(storage)
}

pub fn store_global_state(storage: &mut dyn Storage, state: &GlobalState) -> StdResult<()> {
    GLOBAL_STATE.save(storage, state)
}

pub fn read_global_state(storage: &dyn Storage) -> StdResult<GlobalState> {
    GLOBAL_STATE.load(storage)
}

pub fn store_user_state(
    storage: &mut dyn Storage,
    user: &Addr,
    state: &UserState,
) -> StdResult<()> {
    USER_STATE_MAP.save(storage, user.clone(), state)
}

pub fn read_user_state(storage: &dyn Storage, user: &Addr) -> StdResult<UserState> {
    USER_STATE_MAP.load(storage, user.clone()).map_or_else(
        |_| {
            Ok(UserState {
                user: Addr::unchecked(""),
                total_user_lock_amount: Uint256::zero(),
                claimed_lock_amount: Uint256::zero(),
                last_claimed_period: 0,
                user_per_lock_amount: Uint256::zero(),
            })
        },
        |state| Ok(state),
    )
}

pub fn store_user_by_page(storage: &mut dyn Storage, user: &Addr) -> StdResult<()> {
    Bucket::new(storage, USER_PAGE_NAMESPACE).save(user.clone().as_bytes(), user)
}

pub fn read_user_by_page(
    storage: &dyn Storage,
    start_after: Option<Addr>,
    limit: Option<u32>,
) -> StdResult<Vec<Addr>> {
    let start = calc_range_start(start_after);
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let readonly_bucket: ReadonlyBucket<Addr> = ReadonlyBucket::new(storage, USER_PAGE_NAMESPACE);
    readonly_bucket
        .range(start.as_deref(), None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, v) = item?;
            Ok(v)
        })
        .collect()
}
