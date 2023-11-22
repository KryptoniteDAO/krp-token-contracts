use crate::helper::BASE_RATE_12;
use crate::msg::{QueryClaimableInfoResponse, QueryConfigResponse, QueryRuleInfoResponse};
use crate::state::{read_distribute_config, read_rule_config, read_rule_config_state};
use cosmwasm_std::{
    to_binary, Addr, Deps, Env, QueryRequest, StdError, StdResult, Uint128, WasmQuery,
};
use cw20::Cw20QueryMsg::TokenInfo;
use cw20::{MinterResponse, TokenInfoResponse};

pub fn query_claimable_info(
    deps: Deps,
    env: Env,
    rule_type: String,
) -> StdResult<QueryClaimableInfoResponse> {
    let block_time = env.block.time.seconds();

    let rule_config = read_rule_config(deps.storage, &rule_type)?;
    let rule_config_state = read_rule_config_state(deps.storage, &rule_type)?;

    let total_can_claimed_amount = rule_config.rule_total_amount - rule_config_state.claimed_amount;
    // check if can claim
    if total_can_claimed_amount == 0u128 {
        return Ok(QueryClaimableInfoResponse {
            can_claim_amount: 0,
            release_amount: 0,
            linear_release_amount: 0,
        });
    }

    if rule_config.lock_start_time != 0 && rule_config.lock_start_time > block_time {
        return Ok(QueryClaimableInfoResponse {
            can_claim_amount: 0,
            release_amount: 0,
            linear_release_amount: 0,
        });
    }

    let mut release_amount = 0u128;
    let mut linear_release_amount = 0u128;

    //Calculate the start release amount
    if rule_config.start_release_amount != 0 {
        release_amount = rule_config.start_release_amount;
        //update the start release state
    }

    //Calculate the linear release amount
    if block_time > rule_config.start_linear_release_time {
        let start_calc_time = if rule_config_state.last_claim_linear_release_time
            > rule_config.start_linear_release_time
        {
            rule_config_state.last_claim_linear_release_time
        } else {
            rule_config.start_linear_release_time
        };

        if block_time > rule_config.end_linear_release_time {
            if rule_config_state.is_start_release {
                linear_release_amount = rule_config.unlock_linear_release_amount
                    + rule_config.start_release_amount
                    - rule_config_state.released_amount;
            } else {
                linear_release_amount =
                    rule_config.unlock_linear_release_amount - rule_config_state.released_amount;
            }
        } else {
            let diff_time = block_time - start_calc_time;
            linear_release_amount =
                u128::from(diff_time) * rule_config.linear_release_per_second / BASE_RATE_12;
        }

        // let start_time = if block_time > rule_config.end_linear_release_time {
        //     rule_config.end_linear_release_time
        // } else {
        //     block_time
        // };
        //
        // let diff_time = start_time - start_calc_time;
        //
        // linear_release_amount =
        //     u128::from(diff_time) * rule_config.linear_release_per_second / BASE_RATE_12;
    }
    let can_claim_amount =
        release_amount + linear_release_amount - rule_config_state.claimed_amount;

    Ok(QueryClaimableInfoResponse {
        can_claim_amount,
        release_amount,
        linear_release_amount,
    })
}

pub fn query_rule_info(deps: Deps, rule_type: String) -> StdResult<QueryRuleInfoResponse> {
    let rule_config = read_rule_config(deps.storage, &rule_type)?;
    let rule_config_state = read_rule_config_state(deps.storage, &rule_type)?;
    Ok(QueryRuleInfoResponse {
        rule_config,
        rule_config_state,
    })
}

pub fn query_config(deps: Deps) -> StdResult<crate::msg::QueryConfigResponse> {
    let config = read_distribute_config(deps.storage)?;
    Ok(QueryConfigResponse {
        gov: config.gov,
        total_amount: config.total_amount,
        distribute_token: config.distribute_token,
        distribute_ve_token: config.distribute_ve_token,
        rules_total_amount: config.rules_total_amount,
        token_cap: config.token_cap,
    })
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
