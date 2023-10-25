use crate::handler::{accept_gov, add_lock_setting, set_gov, set_lock_status};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{get_boost_config, get_unlock_time, get_user_boost, get_user_lock_status};
use crate::state::{store_boost_config, BoostConfig};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-ve-seilor-boost";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let gov = msg.gov.unwrap_or_else(|| info.sender.clone());

    let config = BoostConfig {
        gov: gov,
        ve_seilor_lock_settings: msg.ve_seilor_lock_settings,
        new_gov: None,
    };

    store_boost_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "instantiate"),
        ("owner", info.sender.as_str()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddLockSetting {
            duration,
            mining_boost,
        } => add_lock_setting(deps, info, duration, mining_boost),
        ExecuteMsg::SetGov { gov } => set_gov(deps, info, gov),
        ExecuteMsg::AcceptGov {} => accept_gov(deps, info),
        ExecuteMsg::SetLockStatus { index } => {
            let _index = index as usize;
            set_lock_status(deps, env, info, _index)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUnlockTime { user } => to_binary(&get_unlock_time(deps, user)?),
        QueryMsg::GetUserLockStatus { user } => to_binary(&get_user_lock_status(deps, user)?),
        QueryMsg::GetUserBoost {
            user,
            user_updated_at,
            finish_at,
        } => to_binary(&get_user_boost(
            deps,
            env,
            user,
            user_updated_at,
            finish_at,
        )?),
        QueryMsg::GetBoostConfig {} => to_binary(&get_boost_config(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            gov: None,
            ve_seilor_lock_settings: vec![],
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.attributes[0], ("action", "instantiate"));
        assert_eq!(res.attributes[1], ("owner", "creator"));
    }
}
