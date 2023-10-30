use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct TreasureConfigMsg {
    pub lock_token: Option<Addr>,
    pub start_lock_time: Option<u64>,
    pub end_lock_time: Option<u64>,
    // pub dust_reward_per_second: Option<Uint128>,
    pub withdraw_delay_duration: Option<u64>,
    pub no_delay_punish_coefficient: Option<Uint128>,
    pub punish_receiver: Option<Addr>,
    // pub nft_start_pre_mint_time: Option<u64>,
    // pub nft_end_pre_mint_time: Option<u64>,
    // pub mint_nft_cost_dust: Option<Uint128>,
    // pub winning_num: Option<HashSet<u64>>,
    // pub mod_num: Option<u64>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub lock_token: Addr,
    pub start_lock_time: u64,
    pub end_lock_time: u64,
    // pub dust_reward_per_second: Uint128,
    pub withdraw_delay_duration: u64,
    // no delay punish coefficient
    pub no_delay_punish_coefficient: Uint128,
    // punish receiver
    pub punish_receiver: Addr,
    // nft start pre mint time
    // pub nft_start_pre_mint_time: u64,
    // nft end pre mint time
    // pub nft_end_pre_mint_time: u64,
    // nft cost dust
    // pub mint_nft_cost_dust: Uint128,
    // pub winning_num: HashSet<u64>,
    // pub mod_num: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    UpdateConfig(TreasureConfigMsg),
    UserWithdraw { amount: Uint128 },
    UserUnlock { amount: Uint128 },
    // PreMintNft { mint_num: u64 },
    SetGov { gov: Addr },
    AcceptGov {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigInfosResponse)]
    QueryConfigInfos {},
    #[returns(UserInfosResponse)]
    QueryUserInfos { user: Addr },
}

#[cw_serde]
pub struct ConfigInfosResponse {
    pub config: crate::state::TreasureConfig,
    pub state: crate::state::TreasureState,
}

#[cw_serde]
pub struct UserInfosResponse {
    pub user_state: crate::state::TreasureUserState,
}

#[cw_serde]
pub struct MigrateMsg {}

/// This structure describes a CW20 hook message.
#[cw_serde]
pub enum Cw20HookMsg {
    UserLockHook {},
}
