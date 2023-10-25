use crate::state::{GlobalConfig, GlobalState, UserState};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint256};

#[cw_serde]
pub struct UpdateGlobalConfigMsg {
    pub claim_token: Option<Addr>,
    pub start_lock_period_time: Option<u64>,
    pub total_lock_amount: Option<Uint256>,
}

#[cw_serde]
pub struct AddUserMsg {
    pub user: Addr,
    pub lock_amount: Uint256,
    pub replace: bool,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub claim_token: Addr,
    pub total_lock_amount: Uint256,
    pub start_lock_period_time: u64,
    pub duration_per_period: u64,
    pub periods: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateGlobalConfigMsg),
    AddUser(Vec<AddUserMsg>),
    UserClaim {},
    SetGov { gov: Addr },
    AcceptGov {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GlobalInfosResponse)]
    QueryGlobalConfig {},
    #[returns(UserInfoResponse)]
    QueryUserInfo { user: Addr },
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
}

#[cw_serde]
pub struct MigrateMsg {}
