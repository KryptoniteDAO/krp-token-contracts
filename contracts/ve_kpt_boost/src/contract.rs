use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Deps, to_binary, Binary, Addr};
use cw2::set_contract_version;
use cw_utils::nonpayable;
use crate::handler::{add_lock_setting, change_gov, modify_lock_setting, set_lock_status};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{get_boost_config, get_unlock_time, get_user_boost, get_user_lock_status};
use crate::state::{BoostConfig, store_boost_config};


// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-ve-kpt-boost";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let r = nonpayable(&info);
    if r.is_err() {
        return Err(StdError::generic_err("NonPayable"));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let gov = if let Some(gov_addr) = msg.gov {
        Addr::unchecked(gov_addr)
    } else {
        info.sender.clone()
    };

    let config = BoostConfig {
        gov: gov,
        ve_kpt_lock_settings: msg.ve_kpt_lock_settings,
    };

    store_boost_config(deps.storage, &config)?;
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
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddLockSetting { duration, mining_boost } => {
            add_lock_setting(deps, info, duration, mining_boost)
        }
        ExecuteMsg::ModifyLockSetting { index, duration, mining_boost } => {
            let _index = index as usize;
            modify_lock_setting(deps, info, _index, duration, mining_boost)
        }
        ExecuteMsg::ChangeGov { gov } => {
            change_gov(deps, info, gov)
        }
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
        QueryMsg::GetUserLockStatus { user } => to_binary(&get_user_lock_status(deps,  user)?),
        QueryMsg::GetUserBoost { user, user_updated_at, finish_at } => to_binary(&get_user_boost(deps, env, user, user_updated_at, finish_at)?),
        QueryMsg::GetBoostConfig {} => to_binary(&get_boost_config(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            gov: None,
            ve_kpt_lock_settings: vec![],
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.attributes[0], ("action", "instantiate"));
        assert_eq!(res.attributes[1], ("owner", "creator"));
        // Test with non-payable message
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[Coin::new(1000, "KPT")]);
        let msg = InstantiateMsg {
            gov: None,
            ve_kpt_lock_settings: vec![],
        };
        let res = instantiate(deps.as_mut(), env, info.clone(), msg);
        assert!(res.is_err());
    }
}