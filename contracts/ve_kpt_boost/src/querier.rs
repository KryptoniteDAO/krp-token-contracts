use cosmwasm_std::{Addr, Deps, Env, StdResult, Uint128};
use crate::msg::{GetBoostConfigResponse, GetUnlockTimeResponse, GetUserBoostResponse, LockStatusResponse};
use crate::state::{LockStatus, read_user_lock_status};


// Function to get the user's unlock time
pub fn get_unlock_time(deps: Deps, user: Addr) -> StdResult<GetUnlockTimeResponse> {
    let user_lock_status: LockStatus = read_user_lock_status(deps.storage, user)?;
    Ok(GetUnlockTimeResponse {
        unlock_time: user_lock_status.unlock_time,
    })
}

pub fn get_user_lock_status(deps: Deps, user: Addr) -> StdResult<LockStatusResponse> {
    let user_lock_status: LockStatus = read_user_lock_status(deps.storage, user)?;
    Ok(LockStatusResponse {
        unlock_time: user_lock_status.unlock_time,
        duration: user_lock_status.duration,
        mining_boost: user_lock_status.mining_boost,
    })
}


/**
 * @notice calculate the user's mining boost based on their lock status
 * @dev Based on the user's userUpdatedAt time, finishAt time, and the current time,
 * there are several scenarios that could occur, including no acceleration, full acceleration, and partial acceleration.
 */
pub fn get_user_boost(deps: Deps, env: Env, user: Addr, user_updated_at: Uint128, finish_at: Uint128) -> StdResult<GetUserBoostResponse> {
    let user_lock_status = read_user_lock_status(deps.storage, user)?;
    let current_time = Uint128::from(env.block.time.seconds());
    let boost_end_time = user_lock_status.unlock_time;
    let max_boost = user_lock_status.mining_boost;

    let user_boost;
    if user_updated_at.ge(&boost_end_time) || user_updated_at.ge(&finish_at) {
        user_boost = Uint128::zero();
    } else {
        if finish_at.le(&boost_end_time) || current_time.le(&boost_end_time) {
            user_boost = max_boost;
        } else {
            let time = if current_time.gt(&finish_at) { finish_at } else { current_time };
            user_boost =
                (boost_end_time.checked_sub(user_updated_at)).unwrap()
                    .checked_mul(max_boost).unwrap()
                    .checked_div(time.checked_sub(user_updated_at).unwrap()).unwrap();
        }
    }
    return Ok(GetUserBoostResponse {
        user_boost,
    });
}

pub fn get_boost_config(deps: Deps) -> StdResult<GetBoostConfigResponse> {
    let boost_config = crate::state::read_boost_config(deps.storage)?;
    Ok(GetBoostConfigResponse {
        gov: boost_config.gov,
        ve_kpt_lock_settings: boost_config.ve_kpt_lock_settings,
    })
}