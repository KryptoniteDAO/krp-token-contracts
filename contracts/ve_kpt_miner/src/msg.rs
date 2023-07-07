use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint256};


#[cw_serde]
pub struct UpdateMinerConfigStruct {
    pub gov: Option<Addr>,
    pub kusd_denom: Option<String>,
    pub kusd_controller_addr: Option<Addr>,
    pub ve_kpt_boost_addr: Option<Addr>,
    pub kpt_fund_addr: Option<Addr>,
    pub ve_kpt_addr: Option<Addr>,
    pub reward_controller_addr: Option<Addr>,
}

#[cw_serde]
pub struct UpdateMinerStateStruct {
    pub duration: Option<Uint128>,
    pub extra_rate: Option<Uint128>,
    pub lockdown_period: Option<Uint128>,
}


#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub kusd_denom: String,
    pub kusd_controller_addr: Addr,
    pub ve_kpt_boost_addr: Addr,
    pub kpt_fund_addr: Addr,
    pub ve_kpt_addr: Addr,
    pub reward_controller_addr: Addr,

    pub duration: Uint128,
    pub lockdown_period: Uint128,
    pub extra_rate: Option<Uint128>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateMinerConfig {
        gov: Option<Addr>,
        kusd_denom: Option<String>,
        kusd_controller_addr: Option<Addr>,
        ve_kpt_boost_addr: Option<Addr>,
        kpt_fund_addr: Option<Addr>,
        ve_kpt_addr: Option<Addr>,
        reward_controller_addr: Option<Addr>,
    },
    SetIsRedemptionProvider {
        user: Addr,
        is_redemption_provider: bool,
    },
    RefreshReward {
        account: Addr,
    },
    GetReward {},
    NotifyRewardAmount {
        amount: Uint128,
    },
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(cosmwasm_std::BalanceResponse)]
    StakedOf { user: Addr },
    #[returns(RewardPerTokenResponse)]
    RewardPerToken {},
    #[returns(LastTimeRewardApplicableResponse)]
    LastTimeRewardApplicable {},
    #[returns(GetBoostResponse)]
    GetBoost { account: Addr },
    #[returns(EarnedResponse)]
    Earned { account: Addr },
    #[returns(GetMinerConfigResponse)]
    GetMinerConfig {},
    #[returns(GetMinerStateResponse)]
    GetMinerState {},

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
pub struct GetBoostResponse {
    pub boost: Uint128,
}

#[cw_serde]
pub struct GetMinerConfigResponse {
    pub gov: Addr,
    pub kusd_denom: String,
    pub kusd_controller_addr: Addr,
    pub ve_kpt_boost_addr: Addr,
    pub kpt_fund_addr: Addr,
    pub ve_kpt_addr: Addr,
    pub reward_controller_addr: Addr,
}

#[cw_serde]
pub struct GetMinerStateResponse {
    pub duration: Uint128,
    pub finish_at: Uint128,
    pub updated_at: Uint128,
    pub reward_rate: Uint256,
    pub reward_per_token_stored: Uint128,
    pub extra_rate: Uint128,
    pub lockdown_period: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}