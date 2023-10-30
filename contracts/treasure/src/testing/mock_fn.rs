use crate::contract::instantiate;
use crate::msg::InstantiateMsg;
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps, Response, Uint128};

pub const CREATOR: &str = "creator";
pub const LOCK_TOKEN: &str = "lock_token";
pub const PUNISH_RECEIVER: &str = "punish_receiver";

pub fn mock_instantiate_msg(lock_token: Addr) -> InstantiateMsg {
    // let winning_num: HashSet<u64> = (0..25).collect();

    InstantiateMsg {
        gov: None,
        lock_token,
        start_lock_time: 1688128677,
        end_lock_time: 1689720710,
        // dust_reward_per_second: Uint128::from(16534391u128), // 7days reward 10 dust
        withdraw_delay_duration: 86400 * 14,
        // winning_num,
        // mod_num: 100,
        punish_receiver: Addr::unchecked(PUNISH_RECEIVER.to_string()),
        // nft_start_pre_mint_time: 1690520710,
        // nft_end_pre_mint_time: 1699620710,
        no_delay_punish_coefficient: Uint128::from(300000u128),
        // mint_nft_cost_dust: Uint128::from(1_000_000u128 * 10_000u128),
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
