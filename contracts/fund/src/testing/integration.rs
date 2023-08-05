use cosmwasm_std::{Addr, Coin, coin, Timestamp, Uint128};
use cosmwasm_std::testing::mock_env;
use cw20::BalanceResponse;
use cw_multi_test::{App, AppBuilder, ContractWrapper, Executor};
use crate::contract::{execute, instantiate, query};
use crate::msg::ExecuteMsg::Stake;
use crate::msg::{ExecuteMsg, GetClaimAbleKptResponse, GetClaimAbleKusdResponse, QueryMsg};
use crate::testing::mock_fn::{CREATOR, KUSD_DENOM, KUSD_REWARD_ADDR, mock_instantiate_msg};
use crate::testing::mock_third_fn::{mock_kpt_instantiate_msg, mock_ve_kpt_instantiate_msg};

fn mock_app(owner: Addr, coins: Vec<Coin>, block_time: Option<u64>) -> App {
    let mut block = mock_env().block;
    if let Some(time) = block_time {
        block.time = Timestamp::from_seconds(time);
    }
    AppBuilder::new()
        .with_block(block)
        .build(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &owner, coins)
                .unwrap()
        })
}

fn store_kpt_contract(app: &mut App) -> u64 {
    let kpt_contract = Box::new(ContractWrapper::new_with_empty(
        seilor::contract::execute,
        seilor::contract::instantiate,
        seilor::contract::query,
    ));
    app.store_code(kpt_contract)
}

fn store_ve_kpt_contract(app: &mut App) -> u64 {
    let ve_kpt_contract = Box::new(ContractWrapper::new_with_empty(
        ve_seilor::contract::execute,
        ve_seilor::contract::instantiate,
        ve_seilor::contract::query,
    ));
    app.store_code(ve_kpt_contract)
}

fn store_kpt_fun_contract(app: &mut App) -> u64 {
    let kpt_fun_contract = Box::new(ContractWrapper::new_with_empty(
        execute,
        instantiate,
        query,
    ));
    app.store_code(kpt_fun_contract)
}

#[test]
fn test_integration() {
    let block_time = 1688105053u64;
    let creator = Addr::unchecked(CREATOR);
    let kusd_reward_addr = Addr::unchecked(KUSD_REWARD_ADDR);
    let mut app = mock_app(creator.clone(), vec![Coin { denom: KUSD_DENOM.clone().to_string(), amount: Uint128::from(20000000u128) }],
                           Option::Some(block_time));

    // set kusd_reward_addr's balances
    app.send_tokens(creator.clone(),
                    kusd_reward_addr.clone(),
                    &[
                        coin(20000000u128, KUSD_DENOM.clone().to_string()),
                    ]).unwrap();

    //deploy kpt && ve kpt
    let seilor_token = kpt_contract_instance(&creator, &mut app);

    let ve_seilor_token = ve_kpt_contract_instance(&creator, &mut app);

    //deploy kpt_fund
    let test_contract_addr = kpt_fun_contract_instance(&creator, &mut app, &seilor_token, &ve_seilor_token);

    // kpt & ve_seilor set minter role
    add_seilor_fun_to_seilor_and_ve_seilor_role(&creator, &mut app, &seilor_token, &ve_seilor_token, &test_contract_addr);


    // stake
    let stake_amount = 100000000u128;
    stake(&creator, &mut app, &test_contract_addr, stake_amount.clone());

    // Query kusd balance
    let kusd_balance = app.wrap().query_balance(&creator, KUSD_DENOM.to_string()).unwrap();
    assert_eq!(kusd_balance.amount, Uint128::from(0u128));

    let send_kusd_amount = 200000u128;
    // send kusd reward
    notify_reward_amount(&kusd_reward_addr, &mut app, &test_contract_addr, &send_kusd_amount);

    // query kusd reward
    let query_res = get_claimable_kusd(&creator, &mut app, &test_contract_addr);
    assert_eq!(query_res.amount, Uint128::from(send_kusd_amount.clone()));

    // get  kusd reward
    get_kusd_reward(&creator, &mut app, &test_contract_addr);

    // Query kusd balance
    let kusd_balance = app.wrap().query_balance(&creator, KUSD_DENOM.to_string()).unwrap();
    assert_eq!(kusd_balance.amount, Uint128::from(200000u128));


    // query ve_seilor balance
    let query_res = get_ve_kpt_balance(&creator, &mut app, &ve_seilor_token);
    assert_eq!(query_res.balance, Uint128::from(stake_amount.clone()));


    // query claimable kpt
    let query_msg = get_claimable_kpt(&creator, &mut app, &test_contract_addr);
    assert_eq!(query_msg.amount, Uint128::from(0u128));

    // unstake
    let unstake_amount = 2592000u128;
    unstake(&creator, &mut app, &test_contract_addr, &Uint128::from(unstake_amount));

    app.update_block(|block| {
        block.time = block.time.plus_seconds(1000000u64);
        block.height += 1000000u64;
    });

    // query claimable kpt
    let query_msg = get_claimable_kpt(&creator, &mut app, &test_contract_addr);
    assert_eq!(query_msg.amount, Uint128::from(1000000u128));

    // query ve_seilor balance
    let query_res = get_ve_kpt_balance(&creator, &mut app, &ve_seilor_token);
    assert_eq!(query_res.balance, Uint128::from(stake_amount.clone() - unstake_amount.clone()));


    // restake
    re_stake(&creator, &mut app, &test_contract_addr);

    // query ve_seilor balance
    let query_res = get_ve_kpt_balance(&creator, &mut app, &ve_seilor_token);
    assert_eq!(query_res.balance, Uint128::from(stake_amount.clone()));


    // unstake
    let unstake_amount = 2592000u128;
    unstake(&creator, &mut app, &test_contract_addr, &Uint128::from(unstake_amount));

    app.update_block(|block| {
        block.time = block.time.plus_seconds(1000000u64);
        block.height += 1000000u64;
    });

    // query claimable kpt
    let query_msg = get_claimable_kpt(&creator, &mut app, &test_contract_addr);
    assert_eq!(query_msg.amount, Uint128::from(1000000u128));

    // query kpt balance
    let query_res = get_kpt_balance(&creator, &mut app, &seilor_token);
    assert_eq!(query_res.balance, Uint128::from(199999900000000u128));

    // withdraw
    withdraw(&creator, &mut app, &test_contract_addr);

    // query claimable kpt
    let query_msg = get_claimable_kpt(&creator, &mut app, &test_contract_addr);
    assert_eq!(query_msg.amount, Uint128::from(0u128));

    // query ve_seilor balance
    let query_res = get_ve_kpt_balance(&creator, &mut app, &ve_seilor_token);
    assert_eq!(query_res.balance, Uint128::from(stake_amount.clone() - unstake_amount.clone()));

    // query kpt balance
    let query_res = get_kpt_balance(&creator, &mut app, &seilor_token);
    assert_eq!(query_res.balance, Uint128::from(199999901000000u128));
}

fn get_kusd_reward(creator: &Addr, app: &mut App, test_contract_addr: &Addr) {
    let get_reward_msg = ExecuteMsg::GetReward {};
    let res = app
        .execute_contract(
            creator.clone(),
            test_contract_addr.clone(),
            &get_reward_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn get_claimable_kusd(creator: &Addr, app: &mut App, test_contract_addr: &Addr) -> GetClaimAbleKusdResponse {
    let query_kusd_reward_msg = QueryMsg::GetClaimAbleKusd {
        account: creator.clone(),
    };
    let query_res: GetClaimAbleKusdResponse = app.wrap().query_wasm_smart(
        test_contract_addr.clone(),
        &query_kusd_reward_msg,
    ).unwrap();
    query_res
}

fn notify_reward_amount(kusd_reward_addr: &Addr, app: &mut App, test_contract_addr: &Addr, send_kusd_amount: &u128) {
    let notify_reward_amount_msg = ExecuteMsg::NotifyRewardAmount {};
    let res = app
        .execute_contract(
            kusd_reward_addr.clone(),
            test_contract_addr.clone(),
            &notify_reward_amount_msg,
            &[
                coin(send_kusd_amount.clone(), KUSD_DENOM.clone().to_string()),
            ],
        );
    assert!(res.is_ok());
}

fn re_stake(creator: &Addr, app: &mut App, test_contract_addr: &Addr) {
    let re_stake_msg = ExecuteMsg::ReStake {};
    let res = app
        .execute_contract(
            creator.clone(),
            test_contract_addr.clone(),
            &re_stake_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn withdraw(creator: &Addr, app: &mut App, test_contract_addr: &Addr) {
    let withdraw_msg = ExecuteMsg::Withdraw { user: creator.clone() };
    let res = app
        .execute_contract(
            creator.clone(),
            test_contract_addr.clone(),
            &withdraw_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn unstake(creator: &Addr, app: &mut App, test_contract_addr: &Addr, unstake_amount: &Uint128) {
    let unstake_msg = ExecuteMsg::Unstake { amount: unstake_amount.clone() };
    let res = app
        .execute_contract(
            creator.clone(),
            test_contract_addr.clone(),
            &unstake_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn stake(creator: &Addr, app: &mut App, test_contract_addr: &Addr, stake_amount: u128) {
    let stake_msg = Stake {
        amount: Uint128::from(stake_amount.clone()),
    };
    let res = app
        .execute_contract(
            creator.clone(),
            test_contract_addr.clone(),
            &stake_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn add_seilor_fun_to_seilor_and_ve_seilor_role(creator: &Addr, app: &mut App, seilor_token: &Addr, ve_seilor_token: &Addr, fund: &Addr) {
    let update_config = seilor::msg::ExecuteMsg::UpdateConfig {
        fund: Some(fund.clone()),
        gov: None,
        distribute: None,
    };

    let res = app
        .execute_contract(
            creator.clone(),
            seilor_token.clone(),
            &update_config,
            &[],
        );
    assert!(res.is_ok());

    let update_config = ve_seilor::msg::ExecuteMsg::UpdateConfig {
        max_minted: None,
        fund: Some(fund.clone()),
        gov: None,
    };

    let res = app
        .execute_contract(
            creator.clone(),
            ve_seilor_token.clone(),
            &update_config,
            &[],
        );
    assert!(res.is_ok());
}

fn kpt_fun_contract_instance(creator: &Addr, mut app: &mut App, seilor_token: &Addr, ve_seilor_token: &Addr) -> Addr {
    let kpt_fun_code_id = store_kpt_fun_contract(&mut app);
    let kpt_fun_instance_msg = mock_instantiate_msg(seilor_token.clone(), ve_seilor_token.clone());
    // kpt_fun_instance_msg.kusd_reward_addr = Addr::unchecked(CREATOR.clone().to_string());
    let test_contract_addr = app.instantiate_contract(
        kpt_fun_code_id,
        creator.clone(),
        &kpt_fun_instance_msg,
        &[], // no funds
        String::from("KPT_FUN"),
        None,
    ).unwrap();
    test_contract_addr
}

fn ve_kpt_contract_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let ve_kpt_code_id = store_ve_kpt_contract(&mut app);
    let ve_kpt_instance_msg: ve_seilor::msg::InstantiateMsg = mock_ve_kpt_instantiate_msg();
    let ve_seilor_token = app.instantiate_contract(
        ve_kpt_code_id,
        creator.clone(),
        &ve_kpt_instance_msg,
        &[], // no funds
        String::from("VE_KPT"),
        None,
    ).unwrap();
    ve_seilor_token
}

fn kpt_contract_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let kpt_code_id = store_kpt_contract(&mut app);
    let kpt_instance_msg: seilor::msg::InstantiateMsg = mock_kpt_instantiate_msg();
    let seilor_token = app.instantiate_contract(
        kpt_code_id,
        creator.clone(),
        &kpt_instance_msg,
        &[], // no funds
        String::from("KPT"),
        None,
    ).unwrap();
    seilor_token
}

fn get_kpt_balance(creator: &Addr, app: &mut App, seilor_token: &Addr) -> BalanceResponse {
    let query_balance_msg = seilor::msg::QueryMsg::Balance {
        address: creator.clone().to_string(),
    };
    let query_res: cw20::BalanceResponse = app.wrap().query_wasm_smart(
        seilor_token.clone(),
        &query_balance_msg,
    ).unwrap();
    query_res
}

fn get_ve_kpt_balance(creator: &Addr, app: &mut App, ve_seilor_token: &Addr) -> BalanceResponse {
    let query_balance_msg = ve_seilor::msg::QueryMsg::Balance {
        address: creator.clone().to_string(),
    };
    let query_res: cw20::BalanceResponse = app.wrap().query_wasm_smart(
        ve_seilor_token.clone(),
        &query_balance_msg,
    ).unwrap();
    query_res
}

fn get_claimable_kpt(creator: &Addr, app: &mut App, test_contract_addr: &Addr) -> GetClaimAbleKptResponse {
    let query_claimable_seilor_msg = QueryMsg::GetClaimAbleSeilor {
        user: creator.clone(),
    };
    let query_msg: GetClaimAbleKptResponse = app.wrap().query_wasm_smart(
        test_contract_addr.clone(),
        &query_claimable_seilor_msg,
    ).unwrap();
    query_msg
}