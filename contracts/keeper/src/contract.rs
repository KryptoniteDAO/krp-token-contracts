use crate::error::ContractError;
use crate::handler::{
    accept_ownership, distribute_rewards, optional_addr_validate, set_owner, update_config,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::querier::{query_config, query_state};
use crate::state::{store_config, store_state, Config, State};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};

use cw2::set_contract_version;
use cw_utils::nonpayable;

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:keeper";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let r = nonpayable(&info);
    if r.is_err() {
        return Err(ContractError::Std(StdError::generic_err("nonpayable")));
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let api = deps.api;
    let config = Config {
        owner: api.addr_canonicalize(&&msg.owner.as_str())?,
        threshold: msg.threshold,
        rewards_contract: api.addr_canonicalize(&msg.rewards_contract)?,
        rewards_denom: msg.rewards_denom,
        new_owner: None,
    };
    store_config(deps.storage, &config)?;

    let state = State {
        distributed_amount: Uint128::zero(),
        update_time: Uint128::from(env.block.time.seconds()),
        distributed_total: Uint128::zero(),
    };
    store_state(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "instantiate"),
        ("owner", info.sender.as_str()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            threshold,
            rewards_contract,
            rewards_denom,
        } => {
            let api = deps.api;
            update_config(
                deps,
                info,
                threshold,
                optional_addr_validate(api, rewards_contract)?,
                rewards_denom,
            )
        }
        ExecuteMsg::Distribute {} => distribute_rewards(deps, env),
        ExecuteMsg::SetOwner { owner } => set_owner(deps, info, owner),
        ExecuteMsg::AcceptOwnership {} => accept_ownership(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
