use cosmwasm_std::{Addr, Uint128, Uint64};
use cosmwasm_std::testing::mock_info;
use crate::handler::update_kpt_fund_config;
use crate::msg::{KptFundConfigResponse, UpdateConfigMsg};
use crate::querier::kpt_fund_config;
use crate::state::{read_kpt_fund_config};
use crate::testing::mock_fn::{CREATOR, KUSD_DENOM, KUSD_REWARD_ADDR, mock_instantiate, mock_instantiate_msg};

#[test]
fn test_instantiate() {
    let kpt_addr = Addr::unchecked("kpt".to_string());
    let ve_kpt_addr = Addr::unchecked("ve_kpt".to_string());
    let msg = mock_instantiate_msg(kpt_addr.clone(), ve_kpt_addr.clone());
    let (deps, _env, _info, res) = mock_instantiate(msg);
    assert_eq!(res.attributes, vec![
        ("action", "instantiate"),
        ("owner", "creator"),
    ]);

    let config = read_kpt_fund_config(deps.as_ref().storage).unwrap();
    assert_eq!(config.ve_kpt_addr.to_string(), ve_kpt_addr.to_string());
    assert_eq!(config.kpt_addr.to_string(), kpt_addr.to_string());
    assert_eq!(config.kusd_denom, KUSD_DENOM.to_string());
    assert_eq!(config.kusd_reward_addr.to_string(), KUSD_REWARD_ADDR.to_string());
    assert_eq!(config.kusd_reward_total_amount, Uint128::zero());
    assert_eq!(config.kusd_reward_total_paid_amount, Uint128::zero());
    assert_eq!(config.reward_per_token_stored, Uint128::zero());
    assert_eq!(config.exit_cycle, Uint64::from(2592000u64));
    assert_eq!(config.claim_able_time, Uint64::from(1687190400u64));

}

#[test]
fn test_update_kpt_fund_config(){
    let kpt_addr = Addr::unchecked("kpt".to_string());
    let ve_kpt_addr = Addr::unchecked("ve_kpt".to_string());
    let msg = mock_instantiate_msg(kpt_addr.clone(), ve_kpt_addr.clone());
    let (mut deps, _env, _info, _res) = mock_instantiate(msg);

    // Update the config
    let update_msg = UpdateConfigMsg {
        gov: Option::from(Addr::unchecked("new_gov")),
        ve_kpt_addr: Option::from(Addr::unchecked("new_ve_kpt")),
        kpt_addr: Option::from(Addr::unchecked("new_kpt")),
        kusd_denom: Option::from("new_kusd".to_string()),
        kusd_reward_addr: Option::from(Addr::unchecked("new_kusd_reward")),
        claim_able_time: Option::from(Uint64::from(20u64)),
    };
    let info = mock_info("owner2", &[]);
    let res = update_kpt_fund_config(deps.as_mut(), info.clone(), update_msg.clone());
    assert!(res.is_err());
    let info = mock_info(CREATOR, &[]);
    let res = update_kpt_fund_config(deps.as_mut(), info.clone(), update_msg.clone());
    assert!(res.is_ok());
    let config: KptFundConfigResponse = kpt_fund_config(deps.as_ref()).unwrap();
    assert_eq!(config, KptFundConfigResponse {
        gov: Option::from(update_msg.gov.unwrap()).unwrap(),
        ve_kpt_addr: Option::from(update_msg.ve_kpt_addr.unwrap()).unwrap(),
        kpt_addr: Option::from(update_msg.kpt_addr.unwrap()).unwrap(),
        kusd_denom: Option::from(update_msg.kusd_denom.unwrap()).unwrap(),
        kusd_reward_addr: Option::from(update_msg.kusd_reward_addr.unwrap()).unwrap(),
        kusd_reward_total_amount: Uint128::zero(),
        kusd_reward_total_paid_amount: Uint128::zero(),
        reward_per_token_stored: Uint128::zero(),
        exit_cycle: Uint64::from(2592000u64),
        claim_able_time: Option::from(update_msg.claim_able_time.unwrap()).unwrap(),
    });
}