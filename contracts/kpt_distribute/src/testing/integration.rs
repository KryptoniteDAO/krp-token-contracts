use crate::contract::{execute, instantiate, query};
use crate::helper::BASE_RATE_12;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryClaimableInfoResponse, QueryMsg, QueryRuleInfoResponse,
};
use crate::testing::mock_fn::{
    mock_instantiate_msg, AIRDROP_OWNER, CREATOR, DAO_OWNER, LOOT_BOX_OWNER, MINING_OWNER,
    MM_OWNER, RESERVE_OWNER, SHO_OWNER, TEAM_OWNER,
};
use crate::testing::mock_third_fn::mock_kpt_instantiate_msg;
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use cw20::{BalanceResponse, TokenInfoResponse};
use cw_multi_test::{App, AppBuilder, ContractWrapper, Executor};

fn mock_app(owner: Addr, coins: Vec<Coin>, block_time: Option<u64>) -> App {
    let mut block = mock_env().block;
    if let Some(time) = block_time {
        block.time = Timestamp::from_seconds(time);
    }
    AppBuilder::new()
        .with_block(block)
        .build(|router, _, storage| router.bank.init_balance(storage, &owner, coins).unwrap())
}

fn store_kpt_contract(app: &mut App) -> u64 {
    let kpt_contract = Box::new(ContractWrapper::new_with_empty(
        kpt::contract::execute,
        kpt::contract::instantiate,
        kpt::contract::query,
    ));
    app.store_code(kpt_contract)
}

fn store_kpt_distribute_contract(app: &mut App) -> u64 {
    let kpt_distribute_contract =
        Box::new(ContractWrapper::new_with_empty(execute, instantiate, query));
    app.store_code(kpt_distribute_contract)
}

#[test]
fn test_integration_claim_all() {
    let creator = Addr::unchecked(CREATOR);
    let mut app = mock_app(creator.clone(), vec![], None);
    //deploy kpt contract
    let kpt_token = kpt_contract_instance(&creator, &mut app);

    // deploy kpt_distribute contract
    let kpt_distribute = kpt_distribute_contract_instance(&creator, &kpt_token, &mut app);

    // update kpt token mint role
    update_distribute_contract_to_kpt(&creator, &mut app, &kpt_token, &kpt_distribute);

    // update block time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1996315269u64);
        block.height += 1000000u64;
    });

    // loot_box
    let rule_type = "loot_box".to_string();
    let loot_box_owner = Addr::unchecked(LOOT_BOX_OWNER.clone().to_string());
    check_rule_type(
        &mut app,
        &kpt_token,
        &kpt_distribute,
        &rule_type,
        &loot_box_owner,
        20000000000000u128,
        80000000000000u128,
    );

    // team
    let rule_type = "team".to_string();
    let team_owner = Addr::unchecked(TEAM_OWNER.clone().to_string());
    check_rule_type(
        &mut app,
        &kpt_token,
        &kpt_distribute,
        &rule_type,
        &team_owner,
        0u128,
        120000000000000u128,
    );

    // sho
    let rule_type = "sho".to_string();
    let sho_owner = Addr::unchecked(SHO_OWNER.clone().to_string());
    check_rule_type(
        &mut app,
        &kpt_token,
        &kpt_distribute,
        &rule_type,
        &sho_owner,
        5000000000000u128,
        5000000000000u128,
    );

    // dao
    let rule_type = "dao".to_string();
    let dao_owner = Addr::unchecked(DAO_OWNER.clone().to_string());
    check_rule_type(
        &mut app,
        &kpt_token,
        &kpt_distribute,
        &rule_type,
        &dao_owner,
        0u128,
        100000000000000u128,
    );

    // mining
    let rule_type = "mining".to_string();
    let mining_owner = Addr::unchecked(MINING_OWNER.clone().to_string());
    check_rule_type(
        &mut app,
        &kpt_token,
        &kpt_distribute,
        &rule_type,
        &mining_owner,
        0u128,
        500000000000000u128,
    );

    // mm
    let rule_type = "mm".to_string();
    let mm_owner = Addr::unchecked(MM_OWNER.clone().to_string());
    check_rule_type(
        &mut app,
        &kpt_token,
        &kpt_distribute,
        &rule_type,
        &mm_owner,
        0u128,
        50000000000000u128,
    );

    //reserve
    let rule_type = "reserve".to_string();
    let reserve_owner = Addr::unchecked(RESERVE_OWNER.clone().to_string());
    check_rule_type(
        &mut app,
        &kpt_token,
        &kpt_distribute,
        &rule_type,
        &reserve_owner,
        0u128,
        110000000000000u128,
    );
    //airdrop
    let rule_type = "airdrop".to_string();
    let airdrop_owner = Addr::unchecked(AIRDROP_OWNER.clone().to_string());
    check_rule_type(
        &mut app,
        &kpt_token,
        &kpt_distribute,
        &rule_type,
        &airdrop_owner,
        0u128,
        10000000000000u128,
    );

    let res = get_kpt_token_info(&mut app, &kpt_token);
    assert_eq!(res.total_supply, Uint128::from(1000000000000000u128));
}

fn check_rule_type(
    mut app: &mut App,
    kpt_token: &Addr,
    kpt_distribute: &Addr,
    rule_type: &String,
    owner: &Addr,
    start_release_amount: u128,
    unlock_linear_release_amount: u128,
) {
    let res = query_claimable_info(&mut app, &kpt_distribute, &rule_type);
    assert_eq!(
        res.can_claim_amount,
        start_release_amount + unlock_linear_release_amount
    );
    // claim

    claim(&owner, &mut app, &kpt_distribute, &rule_type);

    println!("owner:{}", owner.clone().to_string());
    // check balance
    let loot_box_balance = get_kpt_balance(&mut app, &kpt_token, &owner);
    assert_eq!(
        loot_box_balance.balance,
        Uint128::from(start_release_amount + unlock_linear_release_amount)
    );

    // check claimable
    let res = query_claimable_info(&mut app, &kpt_distribute, &rule_type);
    assert_eq!(res.can_claim_amount, 0u128);
}

#[test]
fn test_integration() {
    let creator = Addr::unchecked(CREATOR);
    let mut app = mock_app(creator.clone(), vec![], None);
    //deploy kpt contract
    let kpt_token = kpt_contract_instance(&creator, &mut app);

    // deploy kpt_distribute contract
    let kpt_distribute = kpt_distribute_contract_instance(&creator, &kpt_token, &mut app);

    // update kpt token mint role
    update_distribute_contract_to_kpt(&creator, &mut app, &kpt_token, &kpt_distribute);

    // query loot_box claimable
    let rule_type = "loot_box".to_string();

    // query loot_box config

    let rule_config_data = query_rule_info(&mut app, &kpt_distribute, &rule_type);
    let rule_config = rule_config_data.rule_config;
    let rule_config_state = rule_config_data.rule_config_state;

    // update block time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(rule_config.lock_start_time + 1000000u64);
        block.height += 1000000u64;
    });

    let res = query_claimable_info(&mut app, &kpt_distribute, &rule_type);

    assert_eq!(res.can_claim_amount, rule_config.start_release_amount);
    assert_eq!(res.release_amount, rule_config.start_release_amount);

    // update block to lock end time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(rule_config.start_linear_release_time + 1000000u64);
        block.height += 1000000u64;
    });
    let per_release_second = rule_config.unlock_linear_release_amount * BASE_RATE_12
        / (rule_config.end_linear_release_time - rule_config.start_linear_release_time) as u128;

    assert_eq!(per_release_second, rule_config.linear_release_per_second);
    let res = query_claimable_info(&mut app, &kpt_distribute, &rule_type);
    let cal_total_release_amount = per_release_second * 1000000u64 as u128 / BASE_RATE_12;
    assert_eq!(res.linear_release_amount, cal_total_release_amount);
    assert_eq!(
        res.can_claim_amount,
        rule_config.start_release_amount + cal_total_release_amount
    );
    let loot_box_owner = Addr::unchecked(LOOT_BOX_OWNER.clone().to_string());

    // claim
    claim(&creator, &mut app, &kpt_distribute, &rule_type); //error

    let res = get_kpt_balance(&mut app, &kpt_token, &loot_box_owner);
    assert_eq!(res.balance.u128(), 0u128);

    claim(&loot_box_owner, &mut app, &kpt_distribute, &rule_type); //success

    let res = get_kpt_balance(&mut app, &kpt_token, &loot_box_owner);

    assert_eq!(
        res.balance.u128(),
        rule_config.start_release_amount + cal_total_release_amount
    );

    let res = query_claimable_info(&mut app, &kpt_distribute, &rule_type);
    assert_eq!(res.can_claim_amount, 0u128);
    // update block to end time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(rule_config.start_linear_release_time + 31622400u64);
        block.height += 1000000u64;
    });
    let res_end_time = query_claimable_info(&mut app, &kpt_distribute, &rule_type);
    // update block to end time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(
            rule_config.start_linear_release_time + 31622399u64 + 1000000u64,
        );
        block.height += 1000000u64;
    });
    let res_end_time_2 = query_claimable_info(&mut app, &kpt_distribute, &rule_type);
    assert_eq!(
        res_end_time.can_claim_amount,
        res_end_time_2.can_claim_amount
    );
    assert_eq!(res_end_time.release_amount, res_end_time_2.release_amount);
    assert_eq!(
        res_end_time.linear_release_amount,
        res_end_time_2.linear_release_amount
    );

    let res = get_kpt_token_info(&mut app, &kpt_token);
    assert_eq!(
        res.total_supply.u128(),
        rule_config.start_release_amount + cal_total_release_amount
    );
}

fn get_kpt_token_info(app: &mut App, kpt_token: &Addr) -> TokenInfoResponse {
    let query_msg = kpt::msg::QueryMsg::TokenInfo {};
    let res: cw20::TokenInfoResponse = app
        .wrap()
        .query_wasm_smart(kpt_token.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn get_kpt_balance(app: &mut App, kpt_token: &Addr, loot_box_owner: &Addr) -> BalanceResponse {
    let query_msg = kpt::msg::QueryMsg::Balance {
        address: loot_box_owner.clone().to_string(),
    };

    let res: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(kpt_token.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn claim(sender: &Addr, app: &mut App, kpt_distribute: &Addr, rule_type: &String) {
    let claim_msg = ExecuteMsg::Claim {
        rule_type: rule_type.clone(),
        msg: None,
    };
    let res = app.execute_contract(
        sender.clone(),
        kpt_distribute.clone(),
        &claim_msg,
        &[], // no funds
    );
    if res.is_ok() {
        println!("claim success");
    } else {
        println!("claim error:{:?}", res.err());
    }
}

fn query_rule_info(
    app: &mut App,
    kpt_distribute: &Addr,
    rule_type: &String,
) -> QueryRuleInfoResponse {
    let query_msg = QueryMsg::QueryRuleInfo {
        rule_type: rule_type.clone(),
    };

    let res: QueryRuleInfoResponse = app
        .wrap()
        .query_wasm_smart(kpt_distribute.clone(), &query_msg)
        .unwrap();
    res
}

fn query_claimable_info(
    app: &mut App,
    kpt_distribute: &Addr,
    rule_type: &String,
) -> QueryClaimableInfoResponse {
    let query_msg = QueryMsg::QueryClaimableInfo {
        rule_type: rule_type.clone(),
    };
    let res: QueryClaimableInfoResponse = app
        .wrap()
        .query_wasm_smart(kpt_distribute.clone(), &query_msg)
        .unwrap();
    res
}

fn update_distribute_contract_to_kpt(
    creator: &Addr,
    app: &mut App,
    kpt_token: &Addr,
    kpt_distribute: &Addr,
) {
    let update_kpt_config_msg = kpt::msg::ExecuteMsg::UpdateConfig {
        kpt_fund: None,
        gov: None,
        kpt_distribute: Some(kpt_distribute.clone()),
    };
    let res = app.execute_contract(
        creator.clone(),
        kpt_token.clone(),
        &update_kpt_config_msg,
        &[], // no funds
    );
    assert!(res.is_ok());
}

fn kpt_distribute_contract_instance(creator: &Addr, kpt_token: &Addr, mut app: &mut App) -> Addr {
    let kpt_distribute_code_id = store_kpt_distribute_contract(&mut app);
    let kpt_distribute_instance_msg: InstantiateMsg = mock_instantiate_msg(kpt_token.clone());
    let kpt_distribute_token = app
        .instantiate_contract(
            kpt_distribute_code_id,
            creator.clone(),
            &kpt_distribute_instance_msg,
            &[], // no funds
            String::from("KPT_DISTRIBUTE"),
            None,
        )
        .unwrap();
    kpt_distribute_token
}

fn kpt_contract_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let kpt_code_id = store_kpt_contract(&mut app);
    let kpt_instance_msg: kpt::msg::InstantiateMsg = mock_kpt_instantiate_msg();
    let kpt_token = app
        .instantiate_contract(
            kpt_code_id,
            creator.clone(),
            &kpt_instance_msg,
            &[], // no funds
            String::from("KPT"),
            None,
        )
        .unwrap();
    kpt_token
}
