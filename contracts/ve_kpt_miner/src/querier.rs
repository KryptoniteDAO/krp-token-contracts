use cosmwasm_std::{Addr, BalanceResponse, BankQuery, Deps, Env, QueryRequest, StdResult, to_binary, Uint128, WasmQuery};
use crate::msg::{EarnedResponse, GetBoostResponse, GetMinerConfigResponse, GetMinerStateResponse, LastTimeRewardApplicableResponse, RewardPerTokenResponse};
use crate::state::{read_is_redemption_provider, read_miner_config, read_miner_state, read_rewards, read_user_reward_per_token_paid, read_user_updated_at};
use crate::third_msg::{GetUserBoostResponse, TotalSupplyResponse, VeKptBoostQueryMsg};
use crate::third_msg::KusdRewardQueryMsg::TotalSupplyQuery;

pub fn total_staked(deps: Deps) -> Uint128 {
    let miner_config = read_miner_config(deps.storage).unwrap();
    let total_supply_response: TotalSupplyResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: miner_config.kusd_controller_addr.to_string(),
        msg: to_binary(&TotalSupplyQuery {}).unwrap(),
    })).unwrap();
    total_supply_response.total_supply
}

pub fn staked_of(deps: Deps, user: Addr) -> StdResult<BalanceResponse> {
    let miner_config = read_miner_config(deps.storage).unwrap();

    let balance_query: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: user.to_string(),
        denom: miner_config.kusd_denom.to_string(),
    }).into())?;

    Ok(balance_query)
}


pub fn reward_per_token(deps: Deps, env: Env) -> StdResult<RewardPerTokenResponse> {
    let miner_state = read_miner_state(deps.storage).unwrap();
    let total_staked = total_staked(deps);
    if total_staked.is_zero() {
        return Ok(RewardPerTokenResponse {
            reward_per_token: miner_state.reward_per_token_stored
        });
    }
    let miner_state = read_miner_state(deps.storage).unwrap();

    let last_time_reward_applicable_response: LastTimeRewardApplicableResponse = last_time_reward_applicable(deps, env).unwrap();
    let last_time_reward_applicable = last_time_reward_applicable_response.last_time_reward_applicable;
    let reward_rate = miner_state.reward_rate;
    let updated_at = miner_state.updated_at;
    let reward_per_token_stored = miner_state.reward_per_token_stored;

    let reward_per_token = reward_per_token_stored + (reward_rate * (last_time_reward_applicable - updated_at) * Uint128::new(1000000)) / total_staked;
    Ok(RewardPerTokenResponse {
        reward_per_token,
    })
}


pub fn is_empty_address(address: &str) -> bool {
    address.trim().is_empty()
}

pub fn last_time_reward_applicable(deps: Deps, env: Env) -> StdResult<LastTimeRewardApplicableResponse> {
    let block_time = Uint128::from(env.block.time.seconds());
    let finish_at = read_miner_state(deps.storage).unwrap().finish_at;
    Ok(LastTimeRewardApplicableResponse {
        last_time_reward_applicable: _min(finish_at, block_time),
    })
}


pub fn get_boost(deps: Deps, account: Addr) -> StdResult<GetBoostResponse> {
    let miner_config = read_miner_config(deps.storage).unwrap();
    let miner_state = read_miner_state(deps.storage).unwrap();
    let is_redemption_provider = read_is_redemption_provider(deps.storage, account.clone());
    let redemption_boost = if is_redemption_provider {
        miner_state.extra_rate
    } else {
        Uint128::zero()
    };
    let user_updated_at = read_user_updated_at(deps.storage, account.clone());
    let finish_at = miner_state.finish_at;

    let msg = VeKptBoostQueryMsg::GetUserBoost {
        user: account.clone(),
        user_updated_at,
        finish_at,
    };

    let get_user_boost_res: GetUserBoostResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: miner_config.ve_kpt_boost_addr.to_string(),
        msg: to_binary(&msg)?,
    })).unwrap();

    let user_boost = get_user_boost_res.user_boost;
    let boost = Uint128::new(100) * Uint128::new(1000000) + redemption_boost + user_boost;
    Ok(GetBoostResponse {
        boost,
    })
}

pub fn earned(deps: Deps, env: Env, account: Addr) -> StdResult<EarnedResponse> {
    let rewards = read_rewards(deps.storage, account.clone());
    let staked_of_response: BalanceResponse = staked_of(deps, account.clone()).unwrap();
    let staked_of = staked_of_response.amount.amount;
    let reward_per_token_response: RewardPerTokenResponse = reward_per_token(deps, env).unwrap();
    let reward_per_token = reward_per_token_response.reward_per_token;
    let user_reward_per_token_paid = read_user_reward_per_token_paid(deps.storage, account.clone());

    let boost = get_boost(deps, account.clone()).unwrap().boost;
    let earned = ((staked_of * boost * (reward_per_token - user_reward_per_token_paid))
        / Uint128::new(1000000000000)) + rewards;
    Ok(EarnedResponse {
        earned,
    })
}

fn _min(a: Uint128, b: Uint128) -> Uint128 {
    if a.lt(&b) {
        a
    } else {
        b
    }
}

pub fn get_miner_config(deps: Deps) -> StdResult<GetMinerConfigResponse> {
    let miner_config = read_miner_config(deps.storage).unwrap();
    Ok(GetMinerConfigResponse {
        gov: miner_config.gov,
        kusd_controller_addr: miner_config.kusd_controller_addr,
        kusd_denom: miner_config.kusd_denom,
        ve_kpt_boost_addr: miner_config.ve_kpt_boost_addr,
        kpt_fund_addr: miner_config.kpt_fund_addr,
        ve_kpt_addr: miner_config.ve_kpt_addr,
        reward_controller_addr: miner_config.reward_controller_addr,
    })
}

pub fn get_miner_state(deps: Deps) -> StdResult<GetMinerStateResponse> {
    let miner_state = read_miner_state(deps.storage).unwrap();
    Ok(GetMinerStateResponse {
        duration: miner_state.duration,
        reward_rate: miner_state.reward_rate,
        reward_per_token_stored: miner_state.reward_per_token_stored,
        finish_at: miner_state.finish_at,
        updated_at: miner_state.updated_at,
        extra_rate: miner_state.extra_rate,
        lockdown_period: miner_state.lockdown_period,
    })
}