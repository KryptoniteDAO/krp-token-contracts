use crate::handler::{update_staking_config, update_staking_duration};
use crate::msg::UpdateStakingConfigStruct;
use crate::querier::query_staking_state;
use crate::state::read_staking_config;
use crate::testing::mock_fn::{
    mock_instantiate, mock_instantiate_msg, REWARD_TOKEN_ADDR, STAKING_TOKEN_ADDR,
};
use cosmwasm_std::{Addr, Uint128};

#[test]
fn test_instantiate() {
    let staking_token = Addr::unchecked(STAKING_TOKEN_ADDR.clone().to_string());
    let rewards_token = Addr::unchecked(REWARD_TOKEN_ADDR.clone().to_string());
    let msg = mock_instantiate_msg(staking_token, rewards_token);
    let (_, _, _, res) = mock_instantiate(msg.clone());
    let res = res.unwrap();
    assert_eq!(0, res.messages.len());
    assert_eq!(res.attributes.len(), 2);
}

#[test]
fn test_update_staking_config() {
    let staking_token = Addr::unchecked(STAKING_TOKEN_ADDR.clone().to_string());
    let rewards_token = Addr::unchecked(REWARD_TOKEN_ADDR.clone().to_string());
    let msg = mock_instantiate_msg(staking_token, rewards_token);
    let (mut deps, _, info, _) = mock_instantiate(msg.clone());
    let update_config_msg = UpdateStakingConfigStruct {
        reward_controller_addr: Some(Addr::unchecked("new_reward_controller_addr".to_string())),
    };
    let res = update_staking_config(deps.as_mut(), info.clone(), update_config_msg);
    assert!(res.is_ok());
    let staking_config = read_staking_config(&deps.storage).unwrap();
    assert_eq!(staking_config.gov, Addr::unchecked("creator".to_string()));
}

#[test]
fn test_update_staking_duration() {
    let staking_token = Addr::unchecked(STAKING_TOKEN_ADDR.clone().to_string());
    let rewards_token = Addr::unchecked(REWARD_TOKEN_ADDR.clone().to_string());
    let msg = mock_instantiate_msg(staking_token, rewards_token);
    let (mut deps, env, info, _) = mock_instantiate(msg.clone());

    let res = update_staking_duration(deps.as_mut(), env, info.clone(), Uint128::from(1000u128));
    assert!(res.is_ok());

    let staking_state = query_staking_state(deps.as_ref()).unwrap();
    assert_eq!(staking_state.duration, Uint128::from(1000u128));
}
