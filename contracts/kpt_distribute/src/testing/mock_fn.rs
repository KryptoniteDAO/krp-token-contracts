use crate::contract::instantiate;
use crate::msg::{InstantiateMsg, RuleConfigMsg};
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps, Response, StdResult};
use std::collections::HashMap;

pub const CREATOR: &str = "creator";
pub const LOOT_BOX_OWNER: &str = "loot_box_owner";
pub const TEAM_OWNER: &str = "team_owner";
pub const SHO_OWNER: &str = "sho_owner";
pub const DAO_OWNER: &str = "dao_owner";
pub const MINING_OWNER: &str = "mining_owner";
pub const MM_OWNER: &str = "mm_owner";
pub const RESERVE_OWNER: &str = "reserve_owner";
pub const AIRDROP_OWNER: &str = "airdrop_owner";

pub fn mock_instantiate_msg(distribute_token: Addr) -> InstantiateMsg {
    let mut rule_configs_map = HashMap::new();

    rule_configs_map.insert(
        "loot_box".to_string(),
        RuleConfigMsg {
            rule_name: "loot_box".to_string(),
            rule_owner: Addr::unchecked(LOOT_BOX_OWNER.clone().to_string()),
            rule_total_amount: 100000000000000u128,
            start_release_amount: 20000000000000u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1696315268u64,
            start_linear_release_time: 1696315269u64,
            unlock_linear_release_amount: 80000000000000u128,
            unlock_linear_release_time: 31622399u64,
        },
    );
    rule_configs_map.insert(
        "team".to_string(),
        RuleConfigMsg {
            rule_name: "team".to_string(),
            rule_owner: Addr::unchecked(TEAM_OWNER.clone().to_string()),
            rule_total_amount: 120000000000000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1704264068u64,
            start_linear_release_time: 1704264069u64,
            unlock_linear_release_amount: 120000000000000u128,
            unlock_linear_release_time: 157852799u64,
        },
    );
    rule_configs_map.insert(
        "sho".to_string(),
        RuleConfigMsg {
            rule_name: "sho".to_string(),
            rule_owner: Addr::unchecked(SHO_OWNER.clone().to_string()),
            rule_total_amount: 10000000000000u128,
            start_release_amount: 5000000000000u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1696315268u64,
            start_linear_release_time: 1696315269u64,
            unlock_linear_release_amount: 5000000000000u128,
            unlock_linear_release_time: 15811199u64,
        },
    );
    rule_configs_map.insert(
        "dao".to_string(),
        RuleConfigMsg {
            rule_name: "dao".to_string(),
            rule_owner: Addr::unchecked(DAO_OWNER.clone().to_string()),
            rule_total_amount: 100000000000000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1704264068u64,
            start_linear_release_time: 1704264069u64,
            unlock_linear_release_amount: 100000000000000u128,
            unlock_linear_release_time: 94694400u64,
        },
    );
    rule_configs_map.insert(
        "mining".to_string(),
        RuleConfigMsg {
            rule_name: "mining".to_string(),
            rule_owner: Addr::unchecked(MINING_OWNER.clone().to_string()),
            rule_total_amount: 500000000000000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1704264068u64,
            start_linear_release_time: 1704264069u64,
            unlock_linear_release_amount: 500000000000000u128,
            unlock_linear_release_time: 65836800u64,
        },
    );
    rule_configs_map.insert(
        "mm".to_string(),
        RuleConfigMsg {
            rule_name: "mm".to_string(),
            rule_owner: Addr::unchecked(MM_OWNER.clone().to_string()),
            rule_total_amount: 50000000000000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1688366468u64,
            start_linear_release_time: 1688366468u64,
            unlock_linear_release_amount: 50000000000000u128,
            unlock_linear_release_time: 55296000u64,
        },
    );
    rule_configs_map.insert(
        "reserve".to_string(),
        RuleConfigMsg {
            rule_name: "reserve".to_string(),
            rule_owner: Addr::unchecked(RESERVE_OWNER.clone().to_string()),
            rule_total_amount: 110000000000000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1704264068u64,
            start_linear_release_time: 1704264069u64,
            unlock_linear_release_amount: 110000000000000u128,
            unlock_linear_release_time: 94694400u64,
        },
    );
    rule_configs_map.insert(
        "airdrop".to_string(),
        RuleConfigMsg {
            rule_name: "airdrop".to_string(),
            rule_owner: Addr::unchecked(AIRDROP_OWNER.clone().to_string()),
            rule_total_amount: 10000000000000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            lock_end_time: 1688366468u64,
            start_linear_release_time: 1688366468u64,
            unlock_linear_release_amount: 10000000000000u128,
            unlock_linear_release_time: 31622400u64,
        },
    );
    InstantiateMsg {
        gov: None,
        total_amount: 1000000000000000,
        distribute_token,
        rule_configs_map,
    }
}

pub fn mock_instantiate(
    msg: InstantiateMsg,
) -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier>,
    Env,
    MessageInfo,
    StdResult<Response>,
) {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);
    (deps, env, info, res)
}
