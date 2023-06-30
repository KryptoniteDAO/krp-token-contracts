use std::collections::HashMap;
use cosmwasm_std::{Deps, Env, StdError, StdResult};
use crate::msg::{AllConfigAndStateResponse, BoxOpenInfoResponse};
use crate::random_role::random_num;
use crate::state::{get_box_open_info, get_box_reward_config, get_box_reward_config_state, get_reward_config};

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
        });
    }
    Ok(res)
}

pub fn test_random(env: Env, token_ids: Vec<String>) -> StdResult<HashMap<String, u64>> {
    random_num(env, token_ids).map_err(|e| StdError::generic_err(e.to_string()))
}