use std::collections::HashMap;
use cosmwasm_std::{Deps, Env, QueryRequest, StdError, StdResult, to_binary, WasmQuery};
use crate::helper::{calc_claim_amount, calc_claim_index, is_empty_str};
use crate::msg::{AllConfigAndStateResponse, BoxClaimableAmountInfoResponse, BoxOpenInfoResponse, QueryBoxClaimableInfoResponse};
use crate::random_role::random_num;
use crate::state::{get_box_open_info, get_box_reward_config, get_box_reward_config_state, get_reward_config};
use crate::third_msg::{DistributeQueryMsg, QueryClaimableInfoResponse};

pub fn query_all_config_and_state(deps: Deps) -> StdResult<AllConfigAndStateResponse> {
    let config = get_reward_config(deps.storage)?;
    let box_config = get_box_reward_config(deps.storage)?;
    let box_state = get_box_reward_config_state(deps.storage)?;
    Ok(AllConfigAndStateResponse {
        config,
        box_config,
        box_state,
    })
}

pub fn query_box_open_info(deps: Deps, token_ids: Vec<String>) -> StdResult<Vec<BoxOpenInfoResponse>> {
    let mut res = Vec::new();
    for token_id in token_ids {
        let box_open_info = get_box_open_info(deps.storage, token_id.clone())?;
        res.push(BoxOpenInfoResponse {
            token_id,
            open_user: box_open_info.open_user,
            open_reward_amount: box_open_info.open_reward_amount,
            open_box_time: box_open_info.open_box_time,
            is_random_box: box_open_info.is_random_box,
            is_reward_box: box_open_info.is_reward_box,
        });
    }
    Ok(res)
}

pub fn test_random(env: Env, token_ids: Vec<String>) -> StdResult<HashMap<String, u64>> {
    random_num(env, token_ids).map_err(|e| StdError::generic_err(e.to_string()))
}

pub fn query_box_claimable_infos(deps: Deps, token_ids: Vec<String>) -> StdResult<QueryBoxClaimableInfoResponse> {
    let config = get_box_reward_config(deps.storage)?;


    if !is_empty_str(config.box_reward_distribute_addr.as_str()) && !is_empty_str(config.box_reward_distribute_rule_type.as_str()) {
        let config_state = get_box_reward_config_state(deps.storage)?;
        // query can claim amount from distribute contract
        let claimable_info: QueryClaimableInfoResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: config.box_reward_distribute_addr.to_string(),
            msg: to_binary(&DistributeQueryMsg::QueryClaimableInfo {
                rule_type: config.box_reward_distribute_rule_type,
            })?,
        }))?;

        // let mut global_reward_claim_index = config_state.global_reward_claim_index;
        // if claimable_info.can_claim_amount > 0 {
        //     global_reward_claim_index += claimable_info.can_claim_amount * BASE_RATE_12 / config.global_reward_total_amount;
        // }

        let global_reward_claim_index = calc_claim_index(
            claimable_info.can_claim_amount, config.global_reward_total_amount, config_state.global_reward_claim_index)?;


        let mut box_claimable_infos = vec![];
        let mut total_claimable_amount = 0u128;
        for token_id in token_ids {
            let box_open_info = get_box_open_info(deps.storage, token_id.clone())?;
            // let diff_index = global_reward_claim_index - box_open_info.reward_claim_index;
            // let claimable_amount = box_open_info.open_reward_amount * diff_index / BASE_RATE_12;
            let claimable_amount = calc_claim_amount(global_reward_claim_index,
                                                     box_open_info.open_reward_amount, box_open_info.reward_claim_index)?;

            total_claimable_amount += claimable_amount;
            box_claimable_infos.push(BoxClaimableAmountInfoResponse {
                token_id,
                claimable_amount,
            });
        }
        return Ok(QueryBoxClaimableInfoResponse {
            next_reward_claim_index: global_reward_claim_index,
            total_claimable_distribute_amount: claimable_info.can_claim_amount,
            total_claimable_amount,
            box_claimable_infos,
        });
    }

    Ok(QueryBoxClaimableInfoResponse {
        next_reward_claim_index: 0,
        total_claimable_distribute_amount: 0,
        total_claimable_amount: 0,
        box_claimable_infos: vec![]
    })
}


