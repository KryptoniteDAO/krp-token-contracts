use crate::error::ContractError;
use crate::handler::{add_rule_config, update_config, update_rule_config};
use crate::msg::{RuleConfigMsg, UpdateRuleConfigMsg};
use crate::querier::{query_config, query_rule_info};
use crate::testing::mock_fn::{mock_instantiate, mock_instantiate_msg};
use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{Addr, StdError};

const SEILOR_TOKEN: &str = "seilor_token";

#[test]
fn test_instantiate() {
    let mut msg = mock_instantiate_msg(Addr::unchecked(SEILOR_TOKEN));
    let (_, _, _, res) = mock_instantiate(msg.clone());
    assert!(res.is_ok());

    msg.total_amount = 10000000u128;
    let (_, _, _, res) = mock_instantiate(msg.clone());
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StdError::generic_err("total_amount must be greater than rule_total_amount")
    );
}

#[test]
fn test_update_config() {
    let msg = mock_instantiate_msg(Addr::unchecked(SEILOR_TOKEN));
    let (mut deps, _, info, res) = mock_instantiate(msg.clone());
    assert!(res.is_ok());

    let new_gov = Some(Addr::unchecked("new_gov"));
    let new_distribute_token = Some(Addr::unchecked("new_distribute_token"));
    let res = update_config(
        deps.as_mut(),
        info,
        new_gov.clone(),
        new_distribute_token.clone(),
    );
    assert!(res.is_ok());
    let config = query_config(deps.as_ref()).unwrap();
    assert_eq!(config.gov, Addr::unchecked("new_gov"));
    assert_eq!(
        config.distribute_token,
        Addr::unchecked("new_distribute_token")
    );

    let other_info = mock_info("other", &[]);
    let res = update_config(deps.as_mut(), other_info, new_gov, new_distribute_token);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), ContractError::Unauthorized {});
}

#[test]
fn test_update_rule_config() {
    let msg = mock_instantiate_msg(Addr::unchecked(SEILOR_TOKEN));
    let (mut deps, _, info, res) = mock_instantiate(msg.clone());
    assert!(res.is_ok());

    let update_rule_msg = UpdateRuleConfigMsg {
        rule_type: "team".to_string(),
        rule_name: Some("new_team_name".to_string()),
        rule_owner: Some(Addr::unchecked("new_rule_owner")),
    };
    let res = update_rule_config(deps.as_mut(), info, update_rule_msg.clone());
    assert!(res.is_ok());
    let query_data = query_rule_info(deps.as_ref(), "team".to_string()).unwrap();
    assert_eq!(query_data.rule_config.rule_name, "new_team_name");
    assert_eq!(
        query_data.rule_config.rule_owner,
        Addr::unchecked("new_rule_owner")
    );

    let other_info = mock_info("other", &[]);
    let res = update_rule_config(deps.as_mut(), other_info, update_rule_msg.clone());
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), ContractError::Unauthorized {});
}

#[test]
fn test_add_rule_config() {
    let mut msg = mock_instantiate_msg(Addr::unchecked(SEILOR_TOKEN));

    msg.rule_configs_map.insert(
        "team".to_string(),
        RuleConfigMsg {
            rule_name: "team".to_string(),
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

    let (mut deps, _, info, res) = mock_instantiate(msg.clone());
    assert!(res.is_ok());

    let query_data = query_rule_info(deps.as_ref(), "team".to_string()).unwrap();
    assert_eq!(query_data.rule_config.rule_name, "team".to_string());
    assert_eq!(
        query_data.rule_config.rule_owner,
        Addr::unchecked("rule_owner")
    );
    assert_eq!(query_data.rule_config.rule_total_amount, 100);

    let config = query_config(deps.as_ref()).unwrap();
    assert_eq!(config.rules_total_amount, 800000000000100u128);

    let res = add_rule_config(
        deps.as_mut(),
        info.clone(),
        "aaa1".to_string(),
        RuleConfigMsg {
            rule_name: "aaa1".to_string(),
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
    assert!(res.is_ok());

    let query_data = query_rule_info(deps.as_ref(), "aaa1".to_string()).unwrap();
    assert_eq!(query_data.rule_config.rule_name, "aaa1".to_string());
    assert_eq!(
        query_data.rule_config.rule_owner,
        Addr::unchecked("rule_owner")
    );
    assert_eq!(query_data.rule_config.rule_total_amount, 100);

    let config = query_config(deps.as_ref()).unwrap();
    assert_eq!(config.rules_total_amount, 800000000000200u128);

    let msg = mock_instantiate_msg(Addr::unchecked(SEILOR_TOKEN));
    let (mut deps, _, info, res) = mock_instantiate(msg.clone());
    assert!(res.is_ok());

    // rule exists
    let res = add_rule_config(
        deps.as_mut(),
        info.clone(),
        "team".to_string(),
        RuleConfigMsg {
            rule_name: "team".to_string(),
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
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        ContractError::Std(StdError::generic_err("rule config already exist",))
    );
    // add zero
    let res = add_rule_config(
        deps.as_mut(),
        info.clone(),
        "aaa1".to_string(),
        RuleConfigMsg {
            rule_name: "aaa1".to_string(),
            rule_owner: Addr::unchecked("rule_owner"),
            rule_total_amount: 0,
            start_release_amount: 0,
            lock_start_time: 0,
            lock_end_time: 0,
            start_linear_release_time: 0,
            unlock_linear_release_amount: 0,
            unlock_linear_release_time: 1,
        },
    );
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        ContractError::Std(StdError::generic_err(
            "rule total amount must be greater than zero",
        ))
    );

    // over amount
    let res = add_rule_config(
        deps.as_mut(),
        info.clone(),
        "test".to_string(),
        RuleConfigMsg {
            rule_name: "test".to_string(),
            rule_owner: Addr::unchecked("test_owner".clone().to_string()),
            rule_total_amount: 10000000000000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1688366468u64,
            start_linear_release_time: 1688366468u64,
            unlock_linear_release_amount: 10000000000000u128,
            unlock_linear_release_time: 1719988868,
        },
    );
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        ContractError::Std(StdError::generic_err(
            "rule total amount over distribute token amount",
        ))
    );
}
