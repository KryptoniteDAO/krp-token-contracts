use crate::state::{GlobalConfig, GlobalState, RegretInfo, UserState};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint256};

#[cw_serde]
pub struct UpdateGlobalConfigMsg {
    pub gov: Option<Addr>,
    pub claim_token: Option<Addr>,
    pub start_time: Option<u64>,
    pub end_regret_time: Option<u64>,
    pub regret_token_receiver: Option<Addr>,

    pub total_lock_amount: Option<Uint256>,
    pub total_unlock_amount: Option<Uint256>,
}

#[cw_serde]
pub struct AddUserMsg {
    pub user: Addr,
    pub unlock_amount: Uint256,
    pub lock_amount: Uint256,
    pub replace: bool,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub claim_token: Addr,
    pub start_time: u64,

    pub end_regret_time: u64,
    pub regret_token_receiver: Addr,

    pub total_lock_amount: Uint256,
    pub total_unlock_amount: Uint256,

    pub start_lock_period_time: u64,
    pub duration_per_period: u64,
    pub periods: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateGlobalConfigMsg),
    AddUser(Vec<AddUserMsg>),
    UserRegret {},
    UserClaim {},
    RegretClaim {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GlobalInfosResponse)]
    QueryGlobalConfig {},
    #[returns(UserInfoResponse)]
    QueryUserInfo { user: Addr },
    #[returns(RegretInfoResponse)]
    QueryRegretInfo {},
    #[returns(Vec<UserInfoResponse>)]
    QueryUserInfos {
        start_after: Option<Addr>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct GlobalInfosResponse {
    pub config: GlobalConfig,
    pub state: GlobalState,
}

#[cw_serde]
pub struct UserInfoResponse {
    pub state: UserState,
    pub current_period: u64,
    pub claimable_lock_amount: Uint256,
    pub claimable_unlock_amount: Uint256,
}

#[cw_serde]
pub struct RegretInfoResponse {
    pub info: RegretInfo,
}

#[cw_serde]
pub struct MigrateMsg {}
