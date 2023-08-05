use crate::constract::instantiate;
use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps, Response, Uint128};

pub const CREATOR: &str = "creator";
pub const REWARD_CONTROLLER_ADDR: &str = "reward_controller";
pub const STAKING_TOKEN_ADDR: &str = "staking_token";
pub const REWARD_TOKEN_ADDR: &str = "rewards_token";
pub const BOOST_ADDR: &str = "boost";
pub const FUND_ADDR: &str = "fund";
pub const KUSD_DENOM: &str = "factory/token";
pub const KUSD_REWARD_ADDR: &str = "kusd_reward_addr";

pub fn mock_instantiate_msg(
    staking_token: Addr,
    rewards_token: Addr,
    boost: Addr,
    fund: Addr,
) -> InstantiateMsg {
    InstantiateMsg {
        gov: None,
        staking_token,
        rewards_token,
        boost,
        fund,
        reward_controller_addr: Addr::unchecked(REWARD_CONTROLLER_ADDR.clone().to_string()),
        duration: Uint128::from(2592000u128),
    }
}

pub fn mock_instantiate(
    msg: InstantiateMsg,
) -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier>,
    Env,
    MessageInfo,
    Result<Response, ContractError>,
) {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);
    (deps, env, info, res)
}
