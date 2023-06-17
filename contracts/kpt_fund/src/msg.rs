
use cosmwasm_schema::{cw_serde,QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint64};

#[cw_serde]
pub struct UpdateConfigMsg {
    pub gov: Option<Addr>,
    pub ve_kpt_addr: Option<Addr>,
    pub kpt_addr: Option<Addr>,
    pub kusd_denom: Option<String>,
    pub kusd_reward_addr: Option<Addr>,
    pub exit_cycle: Option<Uint64>,
    pub claim_able_time: Option<Uint64>,
}


#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub ve_kpt_addr: Addr,
    pub kpt_addr: Addr,
    pub kusd_denom: String,
    pub kusd_reward_addr: Addr,
    pub exit_cycle: Uint64,
    pub claim_able_time: Uint64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateKptFundConfig {
        gov: Option<Addr>,
        ve_kpt_addr: Option<Addr>,
        kpt_addr: Option<Addr>,
        kusd_denom: Option<String>,
        kusd_reward_addr: Option<Addr>,
        exit_cycle: Option<Uint64>,
        claim_able_time: Option<Uint64>,
    },
    RefreshReward {
        account: Addr,
    },
    Stake {
        amount: Uint128,
    },
    Unstake {
        amount: Uint128,
    },
    Withdraw {
        user: Addr,
    },
    ReStake {},
    GetReward {},
    NotifyRewardAmount {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(KptFundConfigResponse)]
    KptFundConfig {},

    #[returns(GetClaimAbleKptResponse)]
    GetClaimAbleKpt {
        user: Addr,
    },
    #[returns(GetReservedKptForVestingResponse)]
    GetReservedKptForVesting {
        user: Addr,
    },
    #[returns(EarnedResponse)]
    Earned {
        account: Addr,
    },
    #[returns(GetClaimAbleKusdResponse)]
    GetClaimAbleKusd {
        account: Addr,
    }
}

#[cw_serde]
pub struct GetClaimAbleKusdResponse {
    pub amount: Uint128,
}
#[cw_serde]
pub struct EarnedResponse {
    pub amount: Uint128,
}

#[cw_serde]
pub struct GetReservedKptForVestingResponse {
    pub amount: Uint128,
}

#[cw_serde]
pub struct GetClaimAbleKptResponse {
    pub amount: Uint128,
}

#[cw_serde]
pub struct KptFundConfigResponse {
    pub gov: Addr,
    pub ve_kpt_addr: Addr,
    pub kpt_addr: Addr,
    pub kusd_denom: String,
    pub kusd_reward_addr: Addr,
    pub kusd_reward_total_amount: Uint128,
    pub kusd_reward_total_paid_amount: Uint128,
    // Sum of (reward rate * dt * 1e18 / total supply)
    pub reward_per_token_stored: Uint128,
    // uint256 immutable exitCycle = 30 days;
    pub exit_cycle: Uint64,
    // uint256 public claimAbleTime;
    pub claim_able_time: Uint64,
}


#[cw_serde]
pub struct MigrateMsg {}
