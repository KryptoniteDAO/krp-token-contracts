use crate::contract::instantiate;
use crate::msg::InstantiateMsg;
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps, Response, Uint64};

pub const CREATOR: &str = "creator";
pub const KUSD_DENOM: &str = "factory/token";
pub const KUSD_REWARD_ADDR: &str = "kusd_reward_addr";

pub fn mock_instantiate_msg(seilor_addr: Addr, ve_seilor_addr: Addr) -> InstantiateMsg {
    let msg = InstantiateMsg {
        gov: None,
        ve_seilor_addr,
        seilor_addr,
        kusd_denom: KUSD_DENOM.to_string(),
        kusd_reward_addr: Addr::unchecked(KUSD_REWARD_ADDR),
        exit_cycle: Uint64::from(2592000u64),
        claim_able_time: Uint64::from(1689190400u64),
    };
    msg
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
