use cosmwasm_std::{Addr, Deps, Env, QueryRequest, StdResult, to_binary, Uint128, Uint64, WasmQuery};
use cw20::{BalanceResponse, TokenInfoResponse};
use cw20_base::msg::QueryMsg::{Balance, TokenInfo};
use crate::msg::{EarnedResponse, GetClaimAbleKptResponse, GetClaimAbleKusdResponse, GetReservedKptForVestingResponse, KptFundConfigResponse};
use crate::state::{KptFundConfig, read_kpt_fund_config, read_last_withdraw_time, read_rewards, read_time2full_redemption, read_unstake_rate, read_user_reward_per_token_paid};


pub fn kpt_fund_config(deps: Deps) -> StdResult<KptFundConfigResponse> {
    let config = read_kpt_fund_config(deps.storage).unwrap();
    Ok(KptFundConfigResponse {
        gov: config.gov,
        ve_kpt_addr: config.ve_kpt_addr,
        kpt_addr: config.kpt_addr,
        kusd_denom: config.kusd_denom.to_string(),
        kusd_reward_addr: config.kusd_reward_addr,
        kusd_reward_total_amount: config.kusd_reward_total_amount,
        kusd_reward_total_paid_amount: config.kusd_reward_total_paid_amount,
        reward_per_token_stored: config.reward_per_token_stored,
        exit_cycle: config.exit_cycle,
        claim_able_time: config.claim_able_time,
    })
}

// Total staked
pub fn total_staked(deps: Deps) -> Uint128 {
    let ve_kpt_addr = read_kpt_fund_config(deps.storage).unwrap().ve_kpt_addr;
    let res: TokenInfoResponse = deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: ve_kpt_addr.to_string(),
            msg: to_binary(&TokenInfo {}).unwrap(),
        })).unwrap();

    res.total_supply
}


pub fn staked_of(deps: Deps, staker: Addr) -> Uint128 {
    let ve_kpt_addr = read_kpt_fund_config(deps.storage).unwrap().ve_kpt_addr;
    let res: BalanceResponse = deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: ve_kpt_addr.to_string(),
            msg: to_binary(&Balance { address: staker.to_string() }).unwrap(),
        })).unwrap();

    res.balance
}

pub fn get_claim_able_kpt(deps: Deps, env: Env, user: Addr) -> StdResult<GetClaimAbleKptResponse> {
    let time2full_redemption_user = read_time2full_redemption(deps.storage, user.clone());
    let last_withdraw_time_user = read_last_withdraw_time(deps.storage, user.clone());
    let unstake_rate_user = read_unstake_rate(deps.storage, user.clone());
    let diff_time;
    let current_time = env.block.time.seconds();
    if current_time.gt(&time2full_redemption_user.u64()) {
        diff_time = Uint128::from(time2full_redemption_user.checked_sub(last_withdraw_time_user).unwrap());
    } else {
        diff_time = Uint128::from(env.block.time.seconds().checked_sub(last_withdraw_time_user.u64()).unwrap());
    }
    let amount = unstake_rate_user.checked_mul(diff_time).unwrap();

    Ok(GetClaimAbleKptResponse { amount })
}

pub fn get_reserved_kpt_for_vesting(deps: Deps, env: Env, user: Addr) -> StdResult<GetReservedKptForVestingResponse> {
    let time2full_redemption_user = read_time2full_redemption(deps.storage, user.clone());
    let unstake_rate_user = read_unstake_rate(deps.storage, user.clone());
    let mut diff_time = Uint128::zero();
    let current_time = env.block.time.seconds();
    if current_time.gt(&time2full_redemption_user.u64()) {
        diff_time = Uint128::from(time2full_redemption_user.checked_sub(Uint64::from(env.block.time.seconds())).unwrap());
    }
    let amount = unstake_rate_user.checked_mul(diff_time).unwrap();
    Ok(GetReservedKptForVestingResponse { amount })
}

pub fn earned(
    deps: Deps,
    account: Addr,
) -> StdResult<EarnedResponse> {
    let config: KptFundConfig = read_kpt_fund_config(deps.storage).unwrap();
    let user_reward_per_token_paid = read_user_reward_per_token_paid(deps.storage, account.clone());
    let user_rewards = read_rewards(deps.storage, account.clone());
    let staked = staked_of(deps, account);
    let a = staked.checked_mul(config.reward_per_token_stored.checked_sub(user_reward_per_token_paid).unwrap()).unwrap();
    let b = a.checked_div(Uint128::new(1_000_000u128)).unwrap();
    let amount = b.checked_add(user_rewards).unwrap();
    Ok(EarnedResponse { amount })
}

pub fn get_claim_able_kusd(deps: Deps, user: Addr) -> StdResult<GetClaimAbleKusdResponse> {
    let amount = earned(deps, user.clone()).unwrap().amount;
    Ok(GetClaimAbleKusdResponse { amount })
}