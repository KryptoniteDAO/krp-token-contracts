use crate::msg::{GlobalInfosResponse, RegretInfoResponse, UserInfoResponse};
use crate::state::{
    read_global_config, read_global_state, read_regret_info, read_user_by_page, read_user_state,
};
use cosmwasm_std::{Addr, Deps, Env, StdResult, Uint256};

pub fn query_global_infos(deps: Deps) -> StdResult<GlobalInfosResponse> {
    let config = read_global_config(deps.storage)?;
    let state = read_global_state(deps.storage)?;
    Ok(GlobalInfosResponse { config, state })
}

pub fn query_user_info(deps: Deps, env: Env, user: Addr) -> StdResult<UserInfoResponse> {
    let state = read_user_state(deps.storage, &user)?;
    let config = read_global_config(deps.storage)?;
    let current_time = env.block.time.seconds();
    let mut current_period = 0u64;
    let mut claimable_lock_amount = Uint256::zero();
    let mut claimable_unlock_amount = Uint256::zero();
    if current_time > config.start_lock_period_time {
        if state.last_claimed_period < config.periods {
            current_period =
                (current_time - config.start_lock_period_time) / config.duration_per_period;
            if current_period > config.periods {
                current_period = config.periods;
            }
        } else {
            current_period = config.periods;
        }
    };
    if !state.is_regret {
        claimable_lock_amount =
            state.user_per_lock_amount * Uint256::from(current_period - state.last_claimed_period);
        if current_time > config.start_time {
            claimable_unlock_amount = state.total_user_unlock_amount - state.claimed_unlock_amount;
        }
    }

    Ok(UserInfoResponse {
        state,
        current_period,
        claimable_lock_amount,
        claimable_unlock_amount,
    })
}

pub fn query_regret_info(deps: Deps) -> StdResult<RegretInfoResponse> {
    let info = read_regret_info(deps.storage)?;
    Ok(RegretInfoResponse { info })
}

pub fn query_user_infos(
    deps: Deps,
    env: Env,
    start_after: Option<Addr>,
    limit: Option<u32>,
) -> StdResult<Vec<UserInfoResponse>> {
    let mut res = vec![];
    let addresses = read_user_by_page(deps.storage, start_after, limit)?;
    for addr in addresses {
        res.push(query_user_info(deps, env.clone(), addr)?);
    }
    Ok(res)
}
