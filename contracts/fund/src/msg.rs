use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint256, Uint64};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct UpdateConfigMsg {
    pub ve_seilor_addr: Option<Addr>,
    pub seilor_addr: Option<Addr>,
    pub kusd_denom: Option<String>,
    pub kusd_reward_addr: Option<Addr>,
    pub claim_able_time: Option<Uint64>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub ve_seilor_addr: Addr,
    pub seilor_addr: Addr,
    pub kusd_denom: String,
    pub kusd_reward_addr: Addr,
    pub exit_cycle: Uint64,
    pub claim_able_time: Uint64,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Receives a message of type [`Cw20ReceiveMsg`]
    Receive(Cw20ReceiveMsg),
    UpdateFundConfig {
        update_config_msg: UpdateConfigMsg,
    },
    RefreshReward {
        account: Addr,
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
    SetGov {
        gov: Addr,
    },
    AcceptGov {},
    SetVeFundMinter {
        minter: Addr,
        is_ve_minter: bool,
    },
    VeFundMint {
        user: Addr,
        amount: Uint128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(FundConfigResponse)]
    FundConfig {},

    #[returns(GetClaimAbleSeilorResponse)]
    GetClaimAbleSeilor { user: Addr },
    #[returns(GetReservedSeilorForVestingResponse)]
    GetReservedSeilorForVesting { user: Addr },
    #[returns(EarnedResponse)]
    Earned { account: Addr },
    #[returns(GetClaimAbleKusdResponse)]
    GetClaimAbleKusd { account: Addr },
    #[returns(UserRewardPerTokenPaidResponse)]
    GetUserRewardPerTokenPaid { account: Addr },
    #[returns(UserRewardsResponse)]
    GetUserRewards { account: Addr },
    #[returns(UserTime2fullRedemptionResponse)]
    GetUserTime2fullRedemption { account: Addr },
    #[returns(UserUnstakeRateResponse)]
    GetUserUnstakeRate { account: Addr },
    #[returns(UserLastWithdrawTimeResponse)]
    GetUserLastWithdrawTime { account: Addr },
    #[returns(bool)]
    IsVeFundMinter { minter: Addr },
}

#[cw_serde]
pub struct UserRewardPerTokenPaidResponse {
    pub user_reward_per_token_paid: Uint128,
}

#[cw_serde]
pub struct UserRewardsResponse {
    pub user_rewards: Uint128,
}

#[cw_serde]
pub struct UserTime2fullRedemptionResponse {
    pub user_time2full_redemption: Uint64,
}

#[cw_serde]
pub struct UserUnstakeRateResponse {
    pub user_unstake_rate: Uint256,
}

#[cw_serde]
pub struct UserLastWithdrawTimeResponse {
    pub user_last_withdraw_time: Uint64,
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
pub struct GetReservedSeilorForVestingResponse {
    pub amount: Uint128,
}

#[cw_serde]
pub struct GetClaimAbleSeilorResponse {
    pub amount: Uint128,
}

#[cw_serde]
pub struct FundConfigResponse {
    pub gov: Addr,
    pub ve_seilor_addr: Addr,
    pub seilor_addr: Addr,
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
    pub new_gov: Option<Addr>,
}

#[cw_serde]
pub struct MigrateMsg {}

/// This structure describes a CW20 hook message.
#[cw_serde]
pub enum Cw20HookMsg {
    Stake {},
}
