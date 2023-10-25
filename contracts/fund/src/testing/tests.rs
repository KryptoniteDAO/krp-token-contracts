use crate::handler::update_fund_config;
use crate::msg::{FundConfigResponse, UpdateConfigMsg};
use crate::querier::fund_config;
use crate::state::read_fund_config;
use crate::testing::mock_fn::{
    mock_instantiate, mock_instantiate_msg, CREATOR, KUSD_DENOM, KUSD_REWARD_ADDR,
};
use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{Addr, Uint128, Uint64};

#[test]
fn test_instantiate() {
    let seilor_addr = Addr::unchecked("seilor".to_string());
    let ve_seilor_addr = Addr::unchecked("ve_seilor".to_string());
    let msg = mock_instantiate_msg(seilor_addr.clone(), ve_seilor_addr.clone());
    let (deps, _env, _info, res) = mock_instantiate(msg);
    assert_eq!(
        res.attributes,
        vec![("action", "instantiate"), ("owner", "creator"),]
    );

    let config = read_fund_config(deps.as_ref().storage).unwrap();
    assert_eq!(
        config.ve_seilor_addr.to_string(),
        ve_seilor_addr.to_string()
    );
    assert_eq!(config.seilor_addr.to_string(), seilor_addr.to_string());
    assert_eq!(config.kusd_denom, KUSD_DENOM.to_string());
    assert_eq!(
        config.kusd_reward_addr.to_string(),
        KUSD_REWARD_ADDR.to_string()
    );
    assert_eq!(config.kusd_reward_total_amount, Uint128::zero());
    assert_eq!(config.kusd_reward_total_paid_amount, Uint128::zero());
    assert_eq!(config.reward_per_token_stored, Uint128::zero());
    assert_eq!(config.exit_cycle, Uint64::from(2592000u64));
    assert_eq!(config.claim_able_time, Uint64::from(1687190400u64));
}

#[test]
fn test_update_fund_config() {
    let seilor_addr = Addr::unchecked("seilor".to_string());
    let ve_seilor_addr = Addr::unchecked("ve_seilor".to_string());
    let msg = mock_instantiate_msg(seilor_addr.clone(), ve_seilor_addr.clone());
    let (mut deps, _env, _info, _res) = mock_instantiate(msg);

    // Update the config
    let update_msg = UpdateConfigMsg {
        ve_seilor_addr: Option::from(Addr::unchecked("new_ve_seilor")),
        seilor_addr: Option::from(Addr::unchecked("new_seilor")),
        kusd_denom: Option::from("new_kusd".to_string()),
        kusd_reward_addr: Option::from(Addr::unchecked("new_kusd_reward")),
        claim_able_time: Option::from(Uint64::from(20u64)),
    };
    let info = mock_info("owner2", &[]);
    let res = update_fund_config(deps.as_mut(), info.clone(), update_msg.clone());
    assert!(res.is_err());
    let info = mock_info(CREATOR, &[]);
    let res = update_fund_config(deps.as_mut(), info.clone(), update_msg.clone());
    assert!(res.is_ok());
    let config: FundConfigResponse = fund_config(deps.as_ref()).unwrap();
    assert_eq!(
        config,
        FundConfigResponse {
            gov: Addr::unchecked(CREATOR.to_string()),
            ve_seilor_addr: Option::from(update_msg.ve_seilor_addr.unwrap()).unwrap(),
            seilor_addr: Option::from(update_msg.seilor_addr.unwrap()).unwrap(),
            kusd_denom: Option::from(update_msg.kusd_denom.unwrap()).unwrap(),
            kusd_reward_addr: Option::from(update_msg.kusd_reward_addr.unwrap()).unwrap(),
            kusd_reward_total_amount: Uint128::zero(),
            kusd_reward_total_paid_amount: Uint128::zero(),
            reward_per_token_stored: Uint128::zero(),
            exit_cycle: Uint64::from(2592000u64),
            claim_able_time: Option::from(update_msg.claim_able_time.unwrap()).unwrap(),
            new_gov: None,
        }
    );
}
