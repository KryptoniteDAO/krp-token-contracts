use crate::handler::{add_period_configs, add_user_period_configs, user_claim_periods};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{
    query_all_period_configs, query_config, query_period_config, query_user_period_config,
};
use crate::state::{store_config, Config};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "kryptonite.finance:distribute-co";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg.gov.unwrap_or_else(|| info.sender.clone());
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(owner.as_str()))?;

    let config = Config {
        token_address: msg.token_address,
        token_distribute_address: msg.token_distribute_address,
    };
    store_config(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddPeriodConfigs { period_configs } => {
            add_period_configs(deps, env, info, period_configs)
        }
        ExecuteMsg::AddUserPeriodConfigs {
            user_period_configs,
        } => add_user_period_configs(deps, env, info, user_period_configs),
        ExecuteMsg::UserClaimPeriods { period_ids } => {
            user_claim_periods(deps, env, info, period_ids)
        }
        ExecuteMsg::UpdateOwnership(action) => {
            let res = cw_ownable::update_ownership(deps, &env.block, &info.sender, action);
            match res {
                Ok(_) => Ok(Response::default()),
                Err(err) => Err(StdError::generic_err(err.to_string())),
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::QueryPeriodConfig { period_id } => {
            to_binary(&query_period_config(deps, period_id)?)
        }
        QueryMsg::QueryUserPeriodConfig { user_address } => {
            to_binary(&query_user_period_config(deps, user_address)?)
        }
        QueryMsg::GetOwnership { .. } => to_binary(&cw_ownable::get_ownership(deps.storage)?),
        QueryMsg::QueryAllPeriodConfigs { .. } => to_binary(&query_all_period_configs(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
