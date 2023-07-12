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
        kpt::contract::execute,
        kpt::contract::instantiate,
        kpt::contract::query,
    ));
    app.store_code(kpt_contract)
}

fn store_ve_kpt_contract(app: &mut App) -> u64 {
    let ve_kpt_contract = Box::new(ContractWrapper::new_with_empty(
        ve_kpt::contract::execute,
        ve_kpt::contract::instantiate,
        ve_kpt::contract::query,
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
    let kpt_token = kpt_contract_instance(&creator, &mut app);

    let ve_kpt_token = ve_kpt_contract_instance(&creator, &mut app);

    //deploy kpt_fund
    let kpt_fun_token = kpt_fun_contract_instance(&creator, &mut app, &kpt_token, &ve_kpt_token);

    // kpt & ve_kpt set minter role
    add_kpt_fun_to_kpt_and_ve_kpt_role(&creator, &mut app, &kpt_token, &ve_kpt_token, &kpt_fun_token);


    // stake
    let stake_amount = 100000000u128;
    stake(&creator, &mut app, &kpt_fun_token, stake_amount.clone());

    // Query kusd balance
    let kusd_balance = app.wrap().query_balance(&creator, KUSD_DENOM.to_string()).unwrap();
    assert_eq!(kusd_balance.amount, Uint128::from(0u128));

    let send_kusd_amount = 200000u128;
    // send kusd reward
    notify_reward_amount(&kusd_reward_addr, &mut app, &kpt_fun_token, &send_kusd_amount);

    // query kusd reward
    let query_res = get_claimable_kusd(&creator, &mut app, &kpt_fun_token);
    assert_eq!(query_res.amount, Uint128::from(send_kusd_amount.clone()));

    // get  kusd reward
    get_kusd_reward(&creator, &mut app, &kpt_fun_token);

    // Query kusd balance
    let kusd_balance = app.wrap().query_balance(&creator, KUSD_DENOM.to_string()).unwrap();
    assert_eq!(kusd_balance.amount, Uint128::from(200000u128));


    // query ve_kpt balance
    let query_res = get_ve_kpt_balance(&creator, &mut app, &ve_kpt_token);
    assert_eq!(query_res.balance, Uint128::from(stake_amount.clone()));


    // query claimable kpt
    let query_msg = get_claimable_kpt(&creator, &mut app, &kpt_fun_token);
    assert_eq!(query_msg.amount, Uint128::from(0u128));

    // unstake
    let unstake_amount = 2592000u128;
    unstake(&creator, &mut app, &kpt_fun_token, &Uint128::from(unstake_amount));

    app.update_block(|block| {
        block.time = block.time.plus_seconds(1000000u64);
        block.height += 1000000u64;
    });

    // query claimable kpt
    let query_msg = get_claimable_kpt(&creator, &mut app, &kpt_fun_token);
    assert_eq!(query_msg.amount, Uint128::from(1000000u128));

    // query ve_kpt balance
    let query_res = get_ve_kpt_balance(&creator, &mut app, &ve_kpt_token);
    assert_eq!(query_res.balance, Uint128::from(stake_amount.clone() - unstake_amount.clone()));


    // restake
    re_stake(&creator, &mut app, &kpt_fun_token);

    // query ve_kpt balance
    let query_res = get_ve_kpt_balance(&creator, &mut app, &ve_kpt_token);
    assert_eq!(query_res.balance, Uint128::from(stake_amount.clone()));


    // unstake
    let unstake_amount = 2592000u128;
    unstake(&creator, &mut app, &kpt_fun_token, &Uint128::from(unstake_amount));

    app.update_block(|block| {
        block.time = block.time.plus_seconds(1000000u64);
        block.height += 1000000u64;
    });

    // query claimable kpt
    let query_msg = get_claimable_kpt(&creator, &mut app, &kpt_fun_token);
    assert_eq!(query_msg.amount, Uint128::from(1000000u128));

    // query kpt balance
    let query_res = get_kpt_balance(&creator, &mut app, &kpt_token);
    assert_eq!(query_res.balance, Uint128::from(199999900000000u128));

    // withdraw
    withdraw(&creator, &mut app, &kpt_fun_token);

    // query claimable kpt
    let query_msg = get_claimable_kpt(&creator, &mut app, &kpt_fun_token);
    assert_eq!(query_msg.amount, Uint128::from(0u128));

    // query ve_kpt balance
    let query_res = get_ve_kpt_balance(&creator, &mut app, &ve_kpt_token);
    assert_eq!(query_res.balance, Uint128::from(stake_amount.clone() - unstake_amount.clone()));

    // query kpt balance
    let query_res = get_kpt_balance(&creator, &mut app, &kpt_token);
    assert_eq!(query_res.balance, Uint128::from(199999901000000u128));
}

fn get_kusd_reward(creator: &Addr, app: &mut App, kpt_fun_token: &Addr) {
    let get_reward_msg = ExecuteMsg::GetReward {};
    let res = app
        .execute_contract(
            creator.clone(),
            kpt_fun_token.clone(),
            &get_reward_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn get_claimable_kusd(creator: &Addr, app: &mut App, kpt_fun_token: &Addr) -> GetClaimAbleKusdResponse {
    let query_kusd_reward_msg = QueryMsg::GetClaimAbleKusd {
        account: creator.clone(),
    };
    let query_res: GetClaimAbleKusdResponse = app.wrap().query_wasm_smart(
        kpt_fun_token.clone(),
        &query_kusd_reward_msg,
    ).unwrap();
    query_res
}

fn notify_reward_amount(kusd_reward_addr: &Addr, app: &mut App, kpt_fun_token: &Addr, send_kusd_amount: &u128) {
    let notify_reward_amount_msg = ExecuteMsg::NotifyRewardAmount {};
    let res = app
        .execute_contract(
            kusd_reward_addr.clone(),
            kpt_fun_token.clone(),
            &notify_reward_amount_msg,
            &[
                coin(send_kusd_amount.clone(), KUSD_DENOM.clone().to_string()),
            ],
        );
    assert!(res.is_ok());
}

fn re_stake(creator: &Addr, app: &mut App, kpt_fun_token: &Addr) {
    let re_stake_msg = ExecuteMsg::ReStake {};
    let res = app
        .execute_contract(
            creator.clone(),
            kpt_fun_token.clone(),
            &re_stake_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn withdraw(creator: &Addr, app: &mut App, kpt_fun_token: &Addr) {
    let withdraw_msg = ExecuteMsg::Withdraw { user: creator.clone() };
    let res = app
        .execute_contract(
            creator.clone(),
            kpt_fun_token.clone(),
            &withdraw_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn unstake(creator: &Addr, app: &mut App, kpt_fun_token: &Addr, unstake_amount: &Uint128) {
    let unstake_msg = ExecuteMsg::Unstake { amount: unstake_amount.clone() };
    let res = app
        .execute_contract(
            creator.clone(),
            kpt_fun_token.clone(),
            &unstake_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn stake(creator: &Addr, app: &mut App, kpt_fun_token: &Addr, stake_amount: u128) {
    let stake_msg = Stake {
        amount: Uint128::from(stake_amount.clone()),
    };
    let res = app
        .execute_contract(
            creator.clone(),
            kpt_fun_token.clone(),
            &stake_msg,
            &[],
        );
    assert!(res.is_ok());
}

fn add_kpt_fun_to_kpt_and_ve_kpt_role(creator: &Addr, app: &mut App, kpt_token: &Addr, ve_kpt_token: &Addr, kpt_fun_token: &Addr) {
    let update_config = kpt::msg::ExecuteMsg::UpdateConfig {
        kpt_fund: Some(kpt_fun_token.clone()),
        gov: None,
        kpt_distribute: None,
    };

    let res = app
        .execute_contract(
            creator.clone(),
            kpt_token.clone(),
            &update_config,
            &[],
        );
    assert!(res.is_ok());

    let update_config = ve_kpt::msg::ExecuteMsg::UpdateConfig {
        max_minted: None,
        kpt_fund: Some(kpt_fun_token.clone()),
        gov: None,
    };

    let res = app
        .execute_contract(
            creator.clone(),
            ve_kpt_token.clone(),
            &update_config,
            &[],
        );
    assert!(res.is_ok());
}

fn kpt_fun_contract_instance(creator: &Addr, mut app: &mut App, kpt_token: &Addr, ve_kpt_token: &Addr) -> Addr {
    let kpt_fun_code_id = store_kpt_fun_contract(&mut app);
    let kpt_fun_instance_msg = mock_instantiate_msg(kpt_token.clone(), ve_kpt_token.clone());
    // kpt_fun_instance_msg.kusd_reward_addr = Addr::unchecked(CREATOR.clone().to_string());
    let kpt_fun_token = app.instantiate_contract(
        kpt_fun_code_id,
        creator.clone(),
        &kpt_fun_instance_msg,
        &[], // no funds
        String::from("KPT_FUN"),
        None,
    ).unwrap();
    kpt_fun_token
}

fn ve_kpt_contract_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let ve_kpt_code_id = store_ve_kpt_contract(&mut app);
    let ve_kpt_instance_msg: ve_kpt::msg::InstantiateMsg = mock_ve_kpt_instantiate_msg();
    let ve_kpt_token = app.instantiate_contract(
        ve_kpt_code_id,
        creator.clone(),
        &ve_kpt_instance_msg,
        &[], // no funds
        String::from("VE_KPT"),
        None,
    ).unwrap();
    ve_kpt_token
}

fn kpt_contract_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let kpt_code_id = store_kpt_contract(&mut app);
    let kpt_instance_msg: kpt::msg::InstantiateMsg = mock_kpt_instantiate_msg();
    let kpt_token = app.instantiate_contract(
        kpt_code_id,
        creator.clone(),
        &kpt_instance_msg,
        &[], // no funds
        String::from("KPT"),
        None,
    ).unwrap();
    kpt_token
}

fn get_kpt_balance(creator: &Addr, app: &mut App, kpt_token: &Addr) -> BalanceResponse {
    let query_balance_msg = kpt::msg::QueryMsg::Balance {
        address: creator.clone().to_string(),
    };
    let query_res: cw20::BalanceResponse = app.wrap().query_wasm_smart(
        kpt_token.clone(),
        &query_balance_msg,
    ).unwrap();
    query_res
}

fn get_ve_kpt_balance(creator: &Addr, app: &mut App, ve_kpt_token: &Addr) -> BalanceResponse {
    let query_balance_msg = ve_kpt::msg::QueryMsg::Balance {
        address: creator.clone().to_string(),
    };
    let query_res: cw20::BalanceResponse = app.wrap().query_wasm_smart(
        ve_kpt_token.clone(),
        &query_balance_msg,
    ).unwrap();
    query_res
}

fn get_claimable_kpt(creator: &Addr, app: &mut App, kpt_fun_token: &Addr) -> GetClaimAbleKptResponse {
    let query_claimable_kpt_msg = QueryMsg::GetClaimAbleKpt {
        user: creator.clone(),
    };
    let query_msg: GetClaimAbleKptResponse = app.wrap().query_wasm_smart(
        kpt_fun_token.clone(),
        &query_claimable_kpt_msg,
    ).unwrap();
    query_msg
}