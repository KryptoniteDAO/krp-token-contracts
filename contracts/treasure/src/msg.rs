use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;
use std::collections::HashSet;

#[cw_serde]
pub struct QueryUserInfosMsg {
    pub user_addr: Addr,
    pub query_user_state: bool,
    pub query_lock_records: bool,
    pub query_withdraw_records: bool,
    pub query_mint_nft_records: bool,
    pub start_after: Option<String>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct TreasureConfigMsg {
    pub gov: Option<Addr>,
    pub lock_token: Option<Addr>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub integral_reward_coefficient: Option<Uint128>,
    pub lock_duration: Option<u64>,
    pub punish_coefficient: Option<Uint128>,
    pub mint_nft_cost_integral: Option<Uint128>,
    pub winning_num: Option<HashSet<u64>>,
    pub mod_num: Option<u64>,
    pub punish_receiver: Option<Addr>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub lock_token: Addr,
    pub start_time: u64,
    pub end_time: u64,
    pub integral_reward_coefficient: Uint128,
    pub lock_duration: u64,
    pub punish_coefficient: Uint128,
    pub mint_nft_cost_integral: Uint128,
    pub winning_num: HashSet<u64>,
    pub mod_num: u64,
    pub punish_receiver: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    UpdateConfig(TreasureConfigMsg),
    UserWithdraw { amount: Uint128 },
    PreMintNft { mint_num: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigInfosResponse)]
    QueryConfigInfos {},
    #[returns(UserInfosResponse)]
    QueryUserInfos { msg: QueryUserInfosMsg },
}

#[cw_serde]
pub struct ConfigInfosResponse {
    pub config: crate::state::TreasureConfig,
    pub state: crate::state::TreasureState,
}

#[cw_serde]
pub struct UserInfosResponse {
    pub user_state: Option<crate::state::TreasureUserState>,
    pub lock_records: Option<Vec<crate::state::UserLockRecord>>,
    pub withdraw_records: Option<Vec<crate::state::UserWithdrawRecord>>,
    pub mint_nft_records: Option<Vec<crate::state::UserMintNftRecord>>,
}

#[cw_serde]
pub struct MigrateMsg {}

/// This structure describes a CW20 hook message.
#[cw_serde]
pub enum Cw20HookMsg {
    UserLockHook {},
}
