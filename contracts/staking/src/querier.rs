use crate::helper::{BASE_RATE_14, BASE_RATE_6};
use crate::msg::{
    BalanceOfResponse, EarnedResponse, GetBoostResponse, GetUserRewardPerTokenPaidResponse,
    GetUserUpdatedAtResponse, LastTimeRewardApplicableResponse, RewardPerTokenResponse,
    StakingConfigResponse, StakingStateResponse,
};
use crate::state::{
    read_balance_of, read_rewards, read_staking_config, read_staking_state,
    read_user_reward_per_token_paid, read_user_updated_at,
};
use cosmwasm_std::{
    to_binary, Addr, Deps, Env, QueryRequest, StdResult, Uint128, Uint256, WasmQuery,
};
use std::str::FromStr;

// Returns the last time the reward was applicable
pub fn last_time_reward_applicable(
    deps: Deps,
    env: Env,
) -> StdResult<LastTimeRewardApplicableResponse> {
    let block_time = Uint128::from(env.block.time.seconds());
    let finish_at = read_staking_state(deps.storage)?.finish_at;
    Ok(LastTimeRewardApplicableResponse {
        last_time_reward_applicable: _min(finish_at, block_time),
    })
}

// Calculates and returns the reward per token
pub fn reward_per_token(deps: Deps, env: Env) -> StdResult<RewardPerTokenResponse> {
    let staking_state = read_staking_state(deps.storage)?;

    if staking_state.total_supply.is_zero() {
        return Ok(RewardPerTokenResponse {
            reward_per_token: staking_state.reward_per_token_stored,
        });
    }

    let last_time_reward_applicable_response = last_time_reward_applicable(deps, env)?;
    let last_time_reward_applicable =
        last_time_reward_applicable_response.last_time_reward_applicable;
    let reward_rate = staking_state.reward_rate;
    let updated_at = staking_state.updated_at;
    let reward_per_token_stored = staking_state.reward_per_token_stored;

    let rewards_256 = reward_rate.multiply_ratio(
        Uint256::from(last_time_reward_applicable - updated_at),
        Uint256::from(BASE_RATE_6),
    );
    let rewards_128 = Uint128::from_str(&rewards_256.to_string())?;
    let reward_per_token = reward_per_token_stored + rewards_128 / staking_state.total_supply;
    Ok(RewardPerTokenResponse { reward_per_token })
}

pub fn get_boost(deps: Deps, account: Addr) -> StdResult<GetBoostResponse> {
    let staking_config = read_staking_config(deps.storage)?;
    let staking_state = read_staking_state(deps.storage)?;

    let user_updated_at = read_user_updated_at(deps.storage, account.clone());
    let finish_at = staking_state.finish_at;

    let msg = boost::msg::QueryMsg::GetUserBoost {
        user: account.clone(),
        user_updated_at,
        finish_at,
    };

    let get_user_boost_res: boost::msg::GetUserBoostResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: staking_config.boost.to_string(),
            msg: to_binary(&msg)?,
        }))?;

    let user_boost = get_user_boost_res.user_boost;
    let boost = Uint128::new(100u128) * Uint128::new(BASE_RATE_6) + user_boost;
    Ok(GetBoostResponse { boost })
}

pub fn earned(deps: Deps, env: Env, account: Addr) -> StdResult<EarnedResponse> {
    let rewards = read_rewards(deps.storage, account.clone());
    let balance_of = read_balance_of(deps.storage, account.clone());
    let reward_per_token_response: RewardPerTokenResponse = reward_per_token(deps, env)?;
    let reward_per_token = reward_per_token_response.reward_per_token;
    let user_reward_per_token_paid = read_user_reward_per_token_paid(deps.storage, account.clone());

    let boost = get_boost(deps, account.clone())?.boost;
    let earned = ((balance_of * boost * (reward_per_token - user_reward_per_token_paid))
        / Uint128::new(BASE_RATE_14))
        + rewards;
    Ok(EarnedResponse { earned })
}

pub fn is_empty_address(address: &str) -> bool {
    address.trim().is_empty()
}

fn _min(a: Uint128, b: Uint128) -> Uint128 {
    if a.lt(&b) {
        a
    } else {
        b
    }
}

pub fn query_staking_config(deps: Deps) -> StdResult<StakingConfigResponse> {
    let staking_config = read_staking_config(deps.storage)?;
    Ok(StakingConfigResponse {
        gov: staking_config.gov,
        staking_token: staking_config.staking_token,
        rewards_token: staking_config.rewards_token,
        boost: staking_config.boost,
        fund: staking_config.fund,
        reward_controller_addr: staking_config.reward_controller_addr,
        new_gov: staking_config.new_gov,
    })
}

pub fn query_staking_state(deps: Deps) -> StdResult<StakingStateResponse> {
    let staking_state = read_staking_state(deps.storage)?;
    Ok(StakingStateResponse {
        duration: staking_state.duration,
        total_supply: staking_state.total_supply,
        reward_rate: staking_state.reward_rate,
        reward_per_token_stored: staking_state.reward_per_token_stored,
        finish_at: staking_state.finish_at,
        updated_at: staking_state.updated_at,
    })
}

pub fn get_user_updated_at(deps: Deps, account: Addr) -> StdResult<GetUserUpdatedAtResponse> {
    let user_updated_at = read_user_updated_at(deps.storage, account.clone());
    Ok(GetUserUpdatedAtResponse {
        updated_at: user_updated_at,
    })
}

pub fn get_user_reward_per_token_paid(
    deps: Deps,
    account: Addr,
) -> StdResult<GetUserRewardPerTokenPaidResponse> {
    let user_reward_per_token_paid = read_user_reward_per_token_paid(deps.storage, account.clone());
    Ok(GetUserRewardPerTokenPaidResponse {
        reward_per_token_paid: user_reward_per_token_paid,
    })
}

pub fn balance_of(deps: Deps, account: Addr) -> StdResult<BalanceOfResponse> {
    let balance_of = read_balance_of(deps.storage, account.clone());
    Ok(BalanceOfResponse { balance_of })
}
