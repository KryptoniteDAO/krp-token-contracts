use crate::contract::instantiate;
use crate::msg::InstantiateMsg;
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps, Response, Uint128};
use std::collections::HashSet;

pub const CREATOR: &str = "creator";
pub const LOCK_TOKEN: &str = "lock_token";
pub const PUNISH_RECEIVER: &str = "punish_receiver";

pub fn mock_instantiate_msg(lock_token: Addr) -> InstantiateMsg {
    let winning_num: HashSet<u64> = (0..25).collect();

    InstantiateMsg {
        gov: None,
        lock_token,
        start_time: 1688128677,
        end_time: 1690720710,
        integral_reward_coefficient: Uint128::from(10u128),
        lock_duration: 86400 * 30,
        punish_coefficient: Uint128::from(300000u128),
        mint_nft_cost_integral: Uint128::from(1_000_000u128 * 10_000u128),
        winning_num,
        mod_num: 100,
        punish_receiver: Addr::unchecked(PUNISH_RECEIVER.to_string()),
    }
}

pub fn mock_instantiate(
    msg: InstantiateMsg,
) -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier>,
    Env,
    MessageInfo,
    Response,
) {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    (deps, env, info, res)
}
