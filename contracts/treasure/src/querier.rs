use crate::msg::{ConfigInfosResponse, UserInfosResponse};
use crate::state::read_treasure_config;
use cosmwasm_std::{Addr, Deps, Env, StdResult};

pub fn query_config_infos(deps: Deps) -> StdResult<ConfigInfosResponse> {
    let config = read_treasure_config(deps.storage)?;
    let state = crate::state::read_treasure_state(deps.storage)?;
    Ok(ConfigInfosResponse { config, state })
}

pub fn query_user_infos(deps: Deps, _env: Env, user: Addr) -> StdResult<UserInfosResponse> {
    let user_state = crate::state::read_treasure_user_state(deps.storage, &user)?;
    // let config = read_treasure_config(deps.storage)?;
    // let current_dust_amount = compute_user_dust(
    //     env.block.time.seconds(),
    //     user_state.last_lock_time,
    //     config.end_lock_time,
    //     user_state.current_locked_amount,
    //     config.dust_reward_per_second,
    // )?;
    // user_state.current_dust_amount += current_dust_amount;
    Ok(UserInfosResponse { user_state })
}

// 计算用户积分奖励
// pub fn compute_user_dust(
//     block_time: u64,
//     user_last_lock_time: u64,
//     global_end_time: u64,
//     user_current_locked_amount: Uint128,
//     dust_reward_per_second: Uint128,
// ) -> StdResult<Uint128> {
//     let mut dust_amount = Uint128::zero();
//
//     if user_last_lock_time < block_time
//         && user_last_lock_time < global_end_time
//         && user_current_locked_amount > Uint128::zero()
//     {
//         let diff_time;
//         if block_time > global_end_time {
//             diff_time = global_end_time - user_last_lock_time;
//         } else {
//             diff_time = block_time - user_last_lock_time;
//         }
//         dust_amount =
//             user_current_locked_amount * Uint128::from(diff_time) * dust_reward_per_second
//                 / Uint128::from(BASE_RATE_12);
//     }
//     Ok(dust_amount)
// }
