use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use crate::state::VeSeilorLockSetting;

#[cw_serde]
pub struct LockStatusResponse {
    pub unlock_time: Uint128,
    pub duration: Uint128,
    pub mining_boost: Uint128,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub ve_seilor_lock_settings: Vec<VeSeilorLockSetting>,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddLockSetting {
        duration: Uint128,
        mining_boost: Uint128,
    },
    ChangeGov {
        gov: Addr,
    },
    SetLockStatus {
        index: u32,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetUnlockTimeResponse)]
    GetUnlockTime {
        user: Addr,
    },
    #[returns(LockStatusResponse)]
    GetUserLockStatus {
        user: Addr,
    },
    #[returns(GetUserBoostResponse)]
    GetUserBoost {
        user: Addr,
        user_updated_at: Uint128,
        finish_at: Uint128,
    },
    #[returns(GetBoostConfigResponse)]
    GetBoostConfig {},
}

#[cw_serde]
pub struct GetBoostConfigResponse {
    pub gov: Addr,
    pub ve_seilor_lock_settings: Vec<VeSeilorLockSetting>,
}

#[cw_serde]
pub struct GetUnlockTimeResponse {
    pub unlock_time: Uint128,
}

#[cw_serde]
pub struct GetUserBoostResponse {
    pub user_boost: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}
