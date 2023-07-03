use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Deps, to_binary, Binary};
use cw2::set_contract_version;
use cw_utils::nonpayable;
use crate::error::ContractError;
use crate::handler::{add_rule_config, claim, update_config, update_rule_config};
use crate::helper::BASE_RATE_12;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_claimable_info, query_config, query_rule_info};
use crate::state::{ DistributeConfig, RuleConfig, RuleConfigState, store_distribute_config, store_rule_config, store_rule_config_state};


// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:kpt-distribute";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let r = nonpayable(&info.clone());
    if r.is_err() {
        return Err(StdError::generic_err(r.err().unwrap().to_string()));
    }

    let gov = if let Some(gov_addr) = msg.gov {
        gov_addr
    } else {
        info.clone().sender
    };


    // init rule config && state
    let mut rule_total_amount = 0u128;
    for (rule_type, rule_msg) in msg.rule_configs_map {

        rule_total_amount += rule_msg.rule_total_amount.clone();
        let end_linear_release_time = rule_msg.start_linear_release_time + rule_msg.unlock_linear_release_time;
        let linear_release_per_second = rule_msg.unlock_linear_release_amount * BASE_RATE_12 / u128::from(rule_msg.unlock_linear_release_time);
        let rule_config = RuleConfig {
            rule_name: rule_msg.rule_name,
            rule_owner: rule_msg.rule_owner,
            rule_total_amount: rule_msg.rule_total_amount,
            start_release_amount: rule_msg.start_release_amount,
            lock_start_time: rule_msg.lock_start_time,
            lock_end_time: rule_msg.lock_end_time,
            start_linear_release_time: rule_msg.start_linear_release_time,
            end_linear_release_time,
            unlock_linear_release_amount: rule_msg.unlock_linear_release_amount,
            unlock_linear_release_time: rule_msg.unlock_linear_release_time,
            linear_release_per_second,
        };
        store_rule_config(deps.storage, &rule_type, &rule_config)?;

        let rule_config_state = RuleConfigState {
            is_start_release: false,
            claimed_amount: 0u128,
            released_amount: 0u128,
            last_claim_linear_release_time: 0,
        };
        store_rule_config_state(deps.storage, &rule_type, &rule_config_state)?;
    }
    // init distribute config
    let distribute_config = DistributeConfig {
        gov: gov.clone(),
        total_amount: msg.total_amount,
        distribute_token: msg.distribute_token,
        rules_total_amount: rule_total_amount,
    };

    if distribute_config.total_amount < rule_total_amount {
        return Err(StdError::generic_err("total_amount must be greater than rule_total_amount"));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    store_distribute_config(deps.storage, &distribute_config)?;

    Ok(Response::new())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Claim { rule_type } => {
            claim(deps, env, info, rule_type)
        }
        ExecuteMsg::UpdateConfig { gov, distribute_token } => {
            update_config(deps, info, gov, distribute_token)
        }
        ExecuteMsg::UpdateRuleConfig { update_rule_msg } => {
            update_rule_config(deps, info, update_rule_msg)
        }
        ExecuteMsg::AddRuleConfig { rule_type, rule_msg } => {
            add_rule_config(deps, info, rule_type, rule_msg)
        }
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryClaimableInfo { rule_type } => {
            to_binary(&query_claimable_info(deps, env, rule_type)?)
        }
        QueryMsg::QueryRuleInfo { rule_type } => {
            to_binary(&query_rule_info(deps, rule_type)?)
        }
        QueryMsg::QueryConfig {} => {
            to_binary(&query_config(deps)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr};
    use crate::msg::RuleConfigMsg;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        // Positive test case
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let mut rule_configs_map = HashMap::new();
        rule_configs_map.insert("rule_type".to_string(),
                                RuleConfigMsg {
                                    rule_name: "rule_name".to_string(),
                                    rule_owner: Addr::unchecked("rule_owner"),
                                    rule_total_amount: 100,
                                    start_release_amount: 0,
                                    lock_start_time: 0,
                                    lock_end_time: 0,
                                    start_linear_release_time: 0,
                                    unlock_linear_release_amount: 0,
                                    unlock_linear_release_time: 1,
                                },
        );

        let msg = InstantiateMsg {
            gov: None,
            rule_configs_map,
            total_amount: 50000,
            distribute_token: Addr::unchecked("distribute_token"),
        };
        let res = instantiate(deps.as_mut(), env, info, msg);
        println!("{:?}", res);
        assert!(res.is_ok());

        let res = add_rule_config(deps.as_mut(), mock_info("creator", &[]), "rule_type".to_string(), RuleConfigMsg {
            rule_name: "rule_name".to_string(),
            rule_owner: Addr::unchecked("rule_owner"),
            rule_total_amount: 100,
            start_release_amount: 0,
            lock_start_time: 0,
            lock_end_time: 0,
            start_linear_release_time: 0,
            unlock_linear_release_amount: 0,
            unlock_linear_release_time: 1,
        });

        println!("{:?}", res);
        assert!(res.is_err());

    }
}