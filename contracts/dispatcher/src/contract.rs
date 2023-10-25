use crate::error::ContractError;
use crate::handler::{add_users, update_config, user_claim};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_global_infos, query_user_info, query_user_infos};
use crate::state::{store_global_config, store_global_state, GlobalConfig, GlobalState};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint256,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:seilor-distribute";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let gov = msg.gov.unwrap_or_else(|| info.sender.clone());

    // verify that start_lock_period_time is greater than the current block time.
    if msg.start_lock_period_time < env.block.time.seconds() {
        return Err(StdError::generic_err(
            "start_lock_period_time must be greater than the current block time",
        ));
    }
    // verify that duration_per_period is greater than 0
    if msg.duration_per_period <= 0 {
        return Err(StdError::generic_err(
            "duration_per_period must be greater than 0",
        ));
    }
    let global_config = GlobalConfig {
        gov,
        claim_token: msg.claim_token,
        total_lock_amount: msg.total_lock_amount,
        start_lock_period_time: msg.start_lock_period_time,
        duration_per_period: msg.duration_per_period,
        periods: msg.periods,
    };

    let global_state = GlobalState {
        total_user_lock_amount: Uint256::zero(),
        total_user_claimed_lock_amount: Uint256::zero(),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    store_global_config(deps.storage, &global_config)?;
    store_global_state(deps.storage, &global_state)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig(msg) => update_config(deps, env, info, msg),
        ExecuteMsg::AddUser(msg) => add_users(deps, info, msg),
        ExecuteMsg::UserClaim {} => user_claim(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryGlobalConfig { .. } => to_binary(&query_global_infos(deps)?),
        QueryMsg::QueryUserInfo { user } => to_binary(&query_user_info(deps, env, user)?),
        QueryMsg::QueryUserInfos { start_after, limit } => {
            to_binary(&query_user_infos(deps, env, start_after, limit)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
