use crate::handler::update_config;
use crate::msg::TreasureConfigMsg;
use crate::testing::mock_fn::{mock_instantiate, mock_instantiate_msg, LOCK_TOKEN};
use cosmwasm_std::{Addr, Uint128};

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
    let (mut deps, env, info, _) = mock_instantiate(msg.clone());
    // let new_winning_num: HashSet<u64> = (75..100).collect();
    let res = update_config(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        TreasureConfigMsg {
            lock_token: Option::from(Addr::unchecked("new_lock_token".to_string())),
            start_lock_time: Option::from(1572797419),
            end_lock_time: Option::from(1581797419),
            // dust_reward_per_second: Option::from(Uint128::from(10_000_000u128)),
            withdraw_delay_duration: Option::from(11114u64),
            // winning_num: Option::from(new_winning_num.clone()),
            // mod_num: Option::from(11117u64),
            punish_receiver: Option::from(Addr::unchecked("new_punish_receiver".to_string())),
            // nft_start_pre_mint_time: Option::from(11117),
            // nft_end_pre_mint_time: Option::from(11118),
            no_delay_punish_coefficient: Option::from(Uint128::from(11115u128)),
            // mint_nft_cost_dust: Option::from(Uint128::from(11116u128)),
        },
    )
    .unwrap();
    assert!(res.attributes.len() > 0);

    let new_config = crate::querier::query_config_infos(deps.as_ref()).unwrap();
    assert_eq!(
        new_config.config.gov,
        Addr::unchecked("creator".to_string())
    );
    assert_eq!(
        new_config.config.lock_token,
        Addr::unchecked("new_lock_token".to_string())
    );
    assert_eq!(new_config.config.start_lock_time, 1572797419);
    assert_eq!(new_config.config.end_lock_time, 1581797419);
    // assert_eq!(
    //     new_config.config.dust_reward_per_second,
    //     Uint128::from(10_000_000u128)
    // );
    assert_eq!(new_config.config.withdraw_delay_duration, 11114u64);

    // assert_eq!(new_config.config.winning_num, new_winning_num);
    // assert_eq!(new_config.config.mod_num, 11117u64);
    assert_eq!(
        new_config.config.punish_receiver,
        Addr::unchecked("new_punish_receiver".to_string())
    );
    // assert_eq!(new_config.config.nft_start_pre_mint_time, 11117);
    // assert_eq!(new_config.config.nft_end_pre_mint_time, 11118);
    assert_eq!(
        new_config.config.no_delay_punish_coefficient,
        Uint128::from(11115u128)
    );
    // assert_eq!(
    //     new_config.config.mint_nft_cost_dust,
    //     Uint128::from(11116u128)
    // );
}
