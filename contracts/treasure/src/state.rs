use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureConfig {
    pub gov: Addr,
    pub lock_token: Addr,
    pub start_lock_time: u64,
    pub end_lock_time: u64,
    //dust reward per second
    // pub dust_reward_per_second: Uint128,
    pub withdraw_delay_duration: u64,

    // no delay punish coefficient
    pub no_delay_punish_coefficient: Uint128,
    // punish receiver
    pub punish_receiver: Addr,

    // // nft start pre mint time
    // pub nft_start_pre_mint_time: u64,
    // // nft end pre mint time
    // pub nft_end_pre_mint_time: u64,
    // // nft cost dust
    // pub mint_nft_cost_dust: Uint128,
    // pub winning_num: HashSet<u64>,
    // pub mod_num: u64,
    pub new_gov: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureState {
    pub current_unlock_amount: Uint128,
    pub current_locked_amount: Uint128,

    pub total_locked_amount: Uint128,
    // pub total_unlock_amount: Uint128,
    pub total_withdraw_amount: Uint128,
    pub total_punish_amount: Uint128,
    // pub total_cost_dust_amount: Uint128,
    //
    // pub total_win_nft_num: u64,
    // pub total_lose_nft_num: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureUserState {
    pub current_locked_amount: Uint128,
    pub last_lock_time: u64,
    pub current_unlock_amount: Uint128,
    pub last_unlock_time: u64,
    // pub current_dust_amount: Uint128,
    //
    // pub win_nft_num: u64,
    // pub lose_nft_num: u64,
    pub total_locked_amount: Uint128,
    pub total_unlock_amount: Uint128,
    pub total_withdraw_amount: Uint128,
    pub total_punish_amount: Uint128,
    // pub total_cost_dust_amount: Uint128,
}

const TREASURE_CONFIG: Item<TreasureConfig> = Item::new("treasure_config");

const TREASURE_STATE: Item<TreasureState> = Item::new("treasure_state");

const TREASURE_USER_STATE: Map<Addr, TreasureUserState> = Map::new("treasure_user_state");

// const GLOBAL_INDEX: Item<Uint64> = Item::new("global_index");

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
                    last_lock_time: 0,
                    current_unlock_amount: Uint128::zero(),
                    last_unlock_time: 0,
                    // current_dust_amount: Uint128::zero(),
                    // win_nft_num: 0,
                    // lose_nft_num: 0,
                    total_locked_amount: Uint128::zero(),
                    total_unlock_amount: Uint128::zero(),
                    total_withdraw_amount: Uint128::zero(),
                    total_punish_amount: Uint128::zero(),
                    // total_cost_dust_amount: Uint128::zero(),
                })
            },
            |user_state| Ok(user_state),
        )
}

// pub fn generate_next_global_id(storage: &mut dyn Storage) -> StdResult<u64> {
//     let mut record_index = GLOBAL_INDEX.load(storage).unwrap_or(Uint64::zero());
//     record_index += Uint64::one();
//     GLOBAL_INDEX.save(storage, &record_index)?;
//     Ok(record_index.u64())
// }
