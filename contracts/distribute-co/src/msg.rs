use crate::state::{PeriodConfig, UserPeriodClaimedDetails};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use std::collections::HashMap;

#[cw_serde]
pub struct UserPeriodConfigMsg {
    pub user_address: Addr,
    pub user_per_period_amount: Uint128,
    pub user_total_amount: Uint128,
    pub user_claimed_periods: HashMap<u64, UserPeriodClaimedDetails>,
}
#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub token_address: Addr,
    pub token_distribute_address: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddPeriodConfigs {
        period_configs: Vec<PeriodConfig>,
    },
    AddUserPeriodConfigs {
        user_period_configs: Vec<UserPeriodConfigMsg>,
    },
    UserClaimPeriods {
        period_ids: Vec<u64>,
    },
    UpdateOwnership(cw_ownable::Action),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Config)]
    QueryConfig {},
    #[returns(crate::state::PeriodConfig)]
    QueryPeriodConfig { period_id: u64 },
    #[returns(crate::state::UserPeriodConfig)]
    QueryUserPeriodConfig { user_address: Addr },
    #[returns(::cw_ownable::Ownership::<::cosmwasm_std::Addr >)]
    GetOwnership {},
    #[returns(Vec<crate::state::PeriodConfig>)]
    QueryAllPeriodConfigs {},
}

#[cw_serde]
pub struct MigrateMsg {}
