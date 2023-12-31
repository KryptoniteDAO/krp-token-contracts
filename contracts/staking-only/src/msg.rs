use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint256};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct UpdateStakingConfigStruct {
    pub reward_controller_addr: Option<Addr>,
}

#[cw_serde]
pub struct LastTimeRewardApplicableResponse {
    pub last_time_reward_applicable: Uint128,
}

#[cw_serde]
pub struct RewardPerTokenResponse {
    pub reward_per_token: Uint128,
}

#[cw_serde]
pub struct EarnedResponse {
    pub earned: Uint128,
}

#[cw_serde]
pub struct GetUserUpdatedAtResponse {
    pub updated_at: Uint128,
}

#[cw_serde]
pub struct GetUserRewardPerTokenPaidResponse {
    pub reward_per_token_paid: Uint128,
}

#[cw_serde]
pub struct BalanceOfResponse {
    pub balance_of: Uint128,
}

/// This structure describes a CW20 hook message.
#[cw_serde]
pub enum Cw20HookMsg {
    Stake {},
    NotifyRewardAmount {},
}

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub staking_token: Addr,
    pub rewards_token: Addr,
    pub reward_controller_addr: Addr,

    pub duration: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Receives a message of type [`Cw20ReceiveMsg`]
    Receive(Cw20ReceiveMsg),
    UpdateStakingConfig {
        config_msg: UpdateStakingConfigStruct,
    },
    UpdateStakingState {
        duration: Uint128,
    },
    GetReward {},
    Withdraw {
        amount: Uint128,
    },
    SetGov {
        gov: Addr,
    },
    AcceptGov {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(RewardPerTokenResponse)]
    RewardPerToken {},
    #[returns(LastTimeRewardApplicableResponse)]
    LastTimeRewardApplicable {},
    #[returns(GetBoostResponse)]
    GetBoost { account: Addr },
    #[returns(EarnedResponse)]
    Earned { account: Addr },
    #[returns(StakingConfigResponse)]
    QueryStakingConfig {},
    #[returns(StakingStateResponse)]
    QueryStakingState {},
    #[returns(GetUserUpdatedAtResponse)]
    GetUserUpdatedAt { account: Addr },
    #[returns(GetUserRewardPerTokenPaidResponse)]
    GetUserRewardPerTokenPaid { account: Addr },
    #[returns(BalanceOfResponse)]
    BalanceOf { account: Addr },
}

#[cw_serde]
pub struct StakingConfigResponse {
    pub gov: Addr,
    pub staking_token: Addr,
    pub rewards_token: Addr,
    pub reward_controller_addr: Addr,
    pub new_gov: Option<Addr>,
}

#[cw_serde]
pub struct GetBoostResponse {
    pub boost: Uint128,
}

#[cw_serde]
pub struct StakingStateResponse {
    pub duration: Uint128,
    pub finish_at: Uint128,
    pub updated_at: Uint128,
    pub reward_rate: Uint256,
    pub reward_per_token_stored: Uint128,
    pub total_supply: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}
