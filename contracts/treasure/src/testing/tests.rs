use crate::handler::update_config;
use crate::msg::TreasureConfigMsg;
use crate::testing::mock_fn::{mock_instantiate, mock_instantiate_msg, LOCK_TOKEN};
use cosmwasm_std::{Addr, Uint128};
use std::collections::HashSet;

#[test]
fn test_instantiate() {
    let msg = mock_instantiate_msg(Addr::unchecked(LOCK_TOKEN.clone()));
    let (_, _, _, res) = mock_instantiate(msg.clone());
    assert_eq!(0, res.messages.len());
    assert_eq!(res.attributes.len(), 1);
}

#[test]
fn test_update_config() {
    let msg = mock_instantiate_msg(Addr::unchecked(LOCK_TOKEN.clone()));
    let (mut deps, _env, info, _) = mock_instantiate(msg.clone());
    let new_winning_num: HashSet<u64> = (75..100).collect();
    let res = update_config(
        deps.as_mut(),
        info.clone(),
        TreasureConfigMsg {
            gov: Option::from(Addr::unchecked("new_gov".to_string())),
            lock_token: Option::from(Addr::unchecked("new_lock_token".to_string())),
            start_time: Option::from(11111),
            end_time: Option::from(11112),
            integral_reward_coefficient: Option::from(Uint128::from(11113u128)),
            lock_duration: Option::from(11114u64),
            punish_coefficient: Option::from(Uint128::from(11115u128)),
            mint_nft_cost_integral: Option::from(Uint128::from(11116u128)),
            winning_num: Option::from(new_winning_num.clone()),
            mod_num: Option::from(11117u64),
            punish_receiver: Option::from(Addr::unchecked("new_punish_receiver".to_string())),
        },
    )
    .unwrap();
    assert!(res.attributes.len() > 0);

    let new_config = crate::querier::query_config_infos(deps.as_ref()).unwrap();
    assert_eq!(
        new_config.config.gov,
        Addr::unchecked("new_gov".to_string())
    );
    assert_eq!(
        new_config.config.lock_token,
        Addr::unchecked("new_lock_token".to_string())
    );
    assert_eq!(new_config.config.start_time, 11111);
    assert_eq!(new_config.config.end_time, 11112);
    assert_eq!(
        new_config.config.integral_reward_coefficient,
        Uint128::from(11113u128)
    );
    assert_eq!(new_config.config.lock_duration, 11114u64);
    assert_eq!(
        new_config.config.punish_coefficient,
        Uint128::from(11115u128)
    );
    assert_eq!(
        new_config.config.mint_nft_cost_integral,
        Uint128::from(11116u128)
    );
    assert_eq!(new_config.config.winning_num, new_winning_num);
    assert_eq!(new_config.config.mod_num, 11117u64);
    assert_eq!(
        new_config.config.punish_receiver,
        Addr::unchecked("new_punish_receiver".to_string())
    );
}
