use crate::helper::{BASE_RATE_12, BASE_RATE_6};
use crate::msg::{
    EarnedResponse, FundConfigResponse, GetClaimAbleKusdResponse, GetClaimAbleSeilorResponse,
    GetReservedSeilorForVestingResponse, UserLastWithdrawTimeResponse,
    UserRewardPerTokenPaidResponse, UserRewardsResponse, UserTime2fullRedemptionResponse,
    UserUnstakeRateResponse,
};
use crate::state::{
    is_ve_minter, read_fund_config, read_last_withdraw_time, read_rewards,
    read_time2full_redemption, read_unstake_rate, read_user_reward_per_token_paid,
};
use cosmwasm_std::{
    to_binary, Addr, Deps, Env, QueryRequest, StdError, StdResult, Uint128, Uint256, Uint64,
    WasmQuery,
};
use cw20::{BalanceResponse, MinterResponse, TokenInfoResponse};
use cw20_base::msg::QueryMsg::{Balance, TokenInfo};
use std::str::FromStr;

pub fn fund_config(deps: Deps) -> StdResult<FundConfigResponse> {
    let config = read_fund_config(deps.storage)?;
    Ok(FundConfigResponse {
        gov: config.gov,
        ve_seilor_addr: config.ve_seilor_addr,
        seilor_addr: config.seilor_addr,
        kusd_denom: config.kusd_denom.to_string(),
        kusd_reward_addr: config.kusd_reward_addr,
        kusd_reward_total_amount: config.kusd_reward_total_amount,
        kusd_reward_total_paid_amount: config.kusd_reward_total_paid_amount,
        reward_per_token_stored: config.reward_per_token_stored,
        exit_cycle: config.exit_cycle,
        claim_able_time: config.claim_able_time,
        new_gov: config.new_gov,
        token_cap: config.token_cap,
    })
}

// Total staked
pub fn total_staked(deps: Deps) -> StdResult<Uint128> {
    let ve_seilor_addr = read_fund_config(deps.storage)?.ve_seilor_addr;
    let res: TokenInfoResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: ve_seilor_addr.to_string(),
        msg: to_binary(&TokenInfo {})?,
    }))?;

    Ok(res.total_supply)
}

pub fn staked_of(deps: Deps, staker: Addr) -> StdResult<Uint128> {
    let ve_seilor_addr = read_fund_config(deps.storage)?.ve_seilor_addr;
    let res: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: ve_seilor_addr.to_string(),
        msg: to_binary(&Balance {
            address: staker.to_string(),
        })?,
    }))?;

    Ok(res.balance)
}

pub fn get_claim_able_seilor(
    deps: Deps,
    env: Env,
    user: Addr,
) -> StdResult<GetClaimAbleSeilorResponse> {
    let time2full_redemption_user = read_time2full_redemption(deps.storage, user.clone());
    let last_withdraw_time_user = read_last_withdraw_time(deps.storage, user.clone());
    let unstake_rate_user = read_unstake_rate(deps.storage, user.clone());
    let diff_time;
    let current_time = env.block.time.seconds();
    let mut amount = Uint256::zero();
    if time2full_redemption_user.gt(&last_withdraw_time_user) {
        if current_time.gt(&time2full_redemption_user.u64()) {
            diff_time =
                Uint256::from(time2full_redemption_user.checked_sub(last_withdraw_time_user)?);
        } else {
            diff_time = Uint256::from(
                env.block
                    .time
                    .seconds()
                    .checked_sub(last_withdraw_time_user.u64())
                    .unwrap(),
            );
        }
        amount = unstake_rate_user.multiply_ratio(diff_time, Uint256::from(BASE_RATE_12));
    }

    Ok(GetClaimAbleSeilorResponse {
        amount: Uint128::from_str(&amount.to_string())?,
    })
}

pub fn get_reserved_seilor_for_vesting(
    deps: Deps,
    env: Env,
    user: Addr,
) -> StdResult<GetReservedSeilorForVestingResponse> {
    let time2full_redemption_user = read_time2full_redemption(deps.storage, user.clone());
    let unstake_rate_user = read_unstake_rate(deps.storage, user.clone());
    let mut diff_time = Uint256::zero();
    let current_time = env.block.time.seconds();
    if current_time.lt(&time2full_redemption_user.u64()) {
        diff_time = Uint256::from(
            time2full_redemption_user
                .checked_sub(Uint64::from(current_time))
                .unwrap(),
        );
    }
    let amount = unstake_rate_user.multiply_ratio(diff_time, Uint256::from(BASE_RATE_12));
    Ok(GetReservedSeilorForVestingResponse {
        amount: Uint128::from_str(&amount.to_string()).unwrap(),
    })
}

pub fn earned(deps: Deps, account: Addr) -> StdResult<EarnedResponse> {
    let config = read_fund_config(deps.storage)?;
    let user_reward_per_token_paid = read_user_reward_per_token_paid(deps.storage, account.clone());
    let user_rewards = read_rewards(deps.storage, account.clone());
    let staked = staked_of(deps, account)?;
    let a = staked.checked_mul(
        config
            .reward_per_token_stored
            .checked_sub(user_reward_per_token_paid)
            .unwrap(),
    )?;
    let b = a.checked_div(Uint128::new(BASE_RATE_6))?;
    let amount = b.checked_add(user_rewards)?;
    Ok(EarnedResponse { amount })
}

// function getClaimAbleUSD(address user) external view returns (uint256 amount) {
// amount = lybra.getMintedEUSDByShares(earned(user));
// }
pub fn get_claim_able_kusd(deps: Deps, user: Addr) -> StdResult<GetClaimAbleKusdResponse> {
    let amount = earned(deps, user.clone()).unwrap().amount;
    Ok(GetClaimAbleKusdResponse { amount })
}

pub fn get_user_reward_per_token_paid(
    deps: Deps,
    account: Addr,
) -> StdResult<UserRewardPerTokenPaidResponse> {
    let user_reward_per_token_paid = read_user_reward_per_token_paid(deps.storage, account);
    Ok(UserRewardPerTokenPaidResponse {
        user_reward_per_token_paid,
    })
}

pub fn get_user_rewards(deps: Deps, account: Addr) -> StdResult<UserRewardsResponse> {
    let user_rewards = read_rewards(deps.storage, account);
    Ok(UserRewardsResponse { user_rewards })
}

pub fn get_user_time2full_redemption(
    deps: Deps,
    account: Addr,
) -> StdResult<UserTime2fullRedemptionResponse> {
    let user_time2full_redemption = read_time2full_redemption(deps.storage, account);
    Ok(UserTime2fullRedemptionResponse {
        user_time2full_redemption,
    })
}

pub fn get_user_unstake_rate(deps: Deps, account: Addr) -> StdResult<UserUnstakeRateResponse> {
    let user_unstake_rate = read_unstake_rate(deps.storage, account);
    Ok(UserUnstakeRateResponse { user_unstake_rate })
}

pub fn get_user_last_withdraw_time(
    deps: Deps,
    account: Addr,
) -> StdResult<UserLastWithdrawTimeResponse> {
    let user_last_withdraw_time = read_last_withdraw_time(deps.storage, account);
    Ok(UserLastWithdrawTimeResponse {
        user_last_withdraw_time,
    })
}

pub fn is_ve_fund_minter(deps: Deps, minter: Addr) -> StdResult<bool> {
    is_ve_minter(deps.storage, minter)
}

pub fn query_token_minter_cap(deps: Deps, token_addr: Addr) -> StdResult<Option<Uint128>> {
    let minter: MinterResponse = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: token_addr.to_string(),
            msg: to_binary(&cw20_base::msg::QueryMsg::Minter {})?,
        }))
        .unwrap_or_else(|_| MinterResponse {
            minter: "".to_string(),
            cap: None,
        });
    Ok(minter.cap)
}

pub fn query_token_total_supply(deps: Deps, token_addr: Addr) -> StdResult<Uint128> {
    let res: TokenInfoResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: token_addr.to_string(),
        msg: to_binary(&TokenInfo {})?,
    }))?;
    Ok(res.total_supply)
}

pub fn check_total_supply(
    deps: Deps,
    token_addr: Addr,
    ve_token_addr: Addr,
    token_cap: Option<Uint128>,
    add_amount: Uint128,
) -> StdResult<()> {
    if token_cap.is_some() {
        let token_total_supply = query_token_total_supply(deps, token_addr)?;
        let ve_token_total_supply = query_token_total_supply(deps, ve_token_addr)?;
        let total_supply = token_total_supply
            .checked_add(ve_token_total_supply)?
            .checked_add(add_amount)?;
        if total_supply > token_cap.unwrap() {
            return Err(StdError::generic_err(
                "total supply of token and ve token is greater than token cap",
            ));
        }
    }
    Ok(())
}
