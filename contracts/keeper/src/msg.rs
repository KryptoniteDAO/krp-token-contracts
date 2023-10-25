use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub threshold: Uint128,
    /// contract address of seilor fund
    pub rewards_contract: String,
    pub rewards_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        threshold: Option<Uint128>,
        rewards_contract: Option<String>,
        rewards_denom: Option<String>,
    },
    Distribute {},
    SetOwner {
        owner: Addr,
    },
    AcceptOwnership {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(StateResponse)]
    State {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
    pub threshold: Uint128,
    pub rewards_contract: String,
    pub rewards_denom: String,
    pub new_owner: Option<String>,
}

#[cw_serde]
pub struct StateResponse {
    // Duration of rewards to be paid out (in seconds) 2_592_000 = 30 days
    pub distributed_amount: Uint128,
    // Timestamp of when the keeper distribute reward to users
    pub update_time: Uint128,
    // total rewards from staking reward on chain
    pub distributed_total: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}
