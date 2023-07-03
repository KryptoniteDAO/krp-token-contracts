use cosmwasm_std::{Deps, Env, StdResult};
use crate::helper::BASE_RATE_12;
use crate::msg::{QueryClaimableInfoResponse, QueryConfigResponse, QueryRuleInfoResponse};
use crate::state::{read_distribute_config, read_rule_config, read_rule_config_state};

pub fn query_claimable_info(deps: Deps, env: Env, rule_type: String) -> StdResult<QueryClaimableInfoResponse> {
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
    if rule_config.start_release_amount != 0 && !rule_config_state.is_start_release {
        release_amount = rule_config.start_release_amount;
        //update the start release state
    }

    //Calculate the linear release amount
    if rule_config.start_linear_release_time > block_time {
        let start_calc_time = if rule_config_state.last_claim_linear_release_time > rule_config.start_linear_release_time {
            rule_config_state.last_claim_linear_release_time
        } else {
            rule_config.start_linear_release_time
        };
        let diff_time = block_time - start_calc_time;
        linear_release_amount = u128::from(diff_time) * rule_config.linear_release_per_second / BASE_RATE_12;
    }
    let can_claim_amount = release_amount + linear_release_amount;
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
        rules_total_amount: config.rules_total_amount,
    })
}
