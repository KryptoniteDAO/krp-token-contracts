use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct TotalSupplyResponse {
    pub total_supply: Uint128,
}


#[cw_serde]
pub struct GetUserBoostResponse {
    pub user_boost: Uint128,
}

#[cw_serde]
pub struct GetUnlockTimeResponse {
    pub unlock_time: Uint128,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum KusdRewardQueryMsg {
    #[returns(TotalSupplyResponse)]
    TotalSupplyQuery {},
}
#[cw_serde]
#[derive(QueryResponses)]
pub enum VeKptBoostQueryMsg {
    #[returns(GetUserBoostResponse)]
    GetUserBoost {
        user: Addr,
        user_updated_at: Uint128,
        finish_at: Uint128,
    },
    #[returns(GetUnlockTimeResponse)]
    GetUnlockTime {
        user: Addr,
    }
}

#[cw_serde]
pub enum KptFundExecuteMsg {
    RefreshReward {
        account: Addr,
    },
}

#[cw_serde]
pub enum VeKptExecuteMsg{
    Mint { recipient: String, amount: Uint128 },
}