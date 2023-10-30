use crate::contract::instantiate;
use crate::msg::{InstantiateMsg, RuleConfigMsg};
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps, Response, StdResult};
use std::collections::HashMap;

pub const CREATOR: &str = "creator";
pub const COMMUNITY_OFFERING_OWNER: &str = "community_offering_owner";
pub const TEAM_OWNER: &str = "team_owner";
// pub const SHO_OWNER: &str = "sho_owner";
pub const DAO_OWNER: &str = "dao_owner";
pub const MINING_OWNER: &str = "mining_owner";
//pub const MM_OWNER: &str = "mm_owner";
pub const RESERVE_OWNER: &str = "reserve_owner";
//pub const AIRDROP_OWNER: &str = "airdrop_owner";

pub fn mock_instantiate_msg(distribute_token: Addr) -> InstantiateMsg {
    let mut rule_configs_map = HashMap::new();

    rule_configs_map.insert(
        "community_offering".to_string(),
        RuleConfigMsg {
            rule_name: "community_offering".to_string(),
            rule_owner: Addr::unchecked(COMMUNITY_OFFERING_OWNER.clone().to_string()),
            rule_total_amount: 175_000_000_000_000u128,
            start_release_amount: 35_000_000_000_000u128,
            lock_start_time: 1688366468u64,
            // lock_end_time: 1696315268u64,
            start_linear_release_time: 1696315269u64,
            unlock_linear_release_amount: 140_000_000_000_000u128,
            unlock_linear_release_time: 31622399u64,
        },
    );
    rule_configs_map.insert(
        "team".to_string(),
        RuleConfigMsg {
            rule_name: "team".to_string(),
            rule_owner: Addr::unchecked(TEAM_OWNER.clone().to_string()),
            rule_total_amount: 200_000_000_000_000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            // lock_end_time: 1704264068u64,
            start_linear_release_time: 1704264069u64,
            unlock_linear_release_amount: 200_000_000_000_000u128,
            unlock_linear_release_time: 157852799u64,
        },
    );

    rule_configs_map.insert(
        "dao".to_string(),
        RuleConfigMsg {
            rule_name: "dao".to_string(),
            rule_owner: Addr::unchecked(DAO_OWNER.clone().to_string()),
            rule_total_amount: 230_000_000_000_000u128,
            start_release_amount: 69_000_000_000_000u128,
            lock_start_time: 1688366468u64,
            // lock_end_time: 1704264068u64,
            start_linear_release_time: 1704264069u64,
            unlock_linear_release_amount: 161_000_000_000_000u128,
            unlock_linear_release_time: 94694400u64,
        },
    );
    rule_configs_map.insert(
        "mining".to_string(),
        RuleConfigMsg {
            rule_name: "mining".to_string(),
            rule_owner: Addr::unchecked(MINING_OWNER.clone().to_string()),
            rule_total_amount: 350_000_000_000_000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            // lock_end_time: 1704264068u64,
            start_linear_release_time: 1704264069u64,
            unlock_linear_release_amount: 350_000_000_000_000u128,
            unlock_linear_release_time: 65836800u64,
        },
    );

    rule_configs_map.insert(
        "reserve".to_string(),
        RuleConfigMsg {
            rule_name: "reserve".to_string(),
            rule_owner: Addr::unchecked(RESERVE_OWNER.clone().to_string()),
            rule_total_amount: 45_000_000_000_000u128,
            start_release_amount: 0u128,
            lock_start_time: 1688366468u64,
            // lock_end_time: 1704264068u64,
            start_linear_release_time: 1704264069u64,
            unlock_linear_release_amount: 45_000_000_000_000u128,
            unlock_linear_release_time: 94694400u64,
        },
    );

    InstantiateMsg {
        gov: None,
        total_amount: 1_000_000_000_000_000,
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
