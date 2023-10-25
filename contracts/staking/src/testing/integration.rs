use crate::constract::{execute, instantiate, query};
use crate::msg::Cw20HookMsg::Stake;
use crate::msg::ExecuteMsg::NotifyRewardAmount;
use crate::msg::{
    BalanceOfResponse, EarnedResponse, ExecuteMsg, GetBoostResponse,
    GetUserRewardPerTokenPaidResponse, GetUserUpdatedAtResponse, LastTimeRewardApplicableResponse,
    QueryMsg, RewardPerTokenResponse, StakingConfigResponse, StakingStateResponse,
};
use crate::testing::mock_fn::{mock_instantiate_msg, CREATOR, REWARD_CONTROLLER_ADDR};
use crate::testing::mock_third_fn::{
    mock_boost_instance_msg, mock_fund_instance_msg, mock_seilor_instance_msg,
    mock_staking_token_instance_msg, mock_ve_seilor_instance_msg,
};
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{to_binary, Addr, Coin, Timestamp, Uint128};
use cw20::BalanceResponse;
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

fn store_staking_token_contract(app: &mut App) -> u64 {
    let staking_token_contract = Box::new(ContractWrapper::new_with_empty(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    ));
    app.store_code(staking_token_contract)
}

fn store_ve_seilor_token_contract(app: &mut App) -> u64 {
    let ve_seilor_token_contract = Box::new(ContractWrapper::new_with_empty(
        ve_seilor::contract::execute,
        ve_seilor::contract::instantiate,
        ve_seilor::contract::query,
    ));
    app.store_code(ve_seilor_token_contract)
}

fn store_boost_contract(app: &mut App) -> u64 {
    let ve_boost_contract = Box::new(ContractWrapper::new_with_empty(
        boost::contract::execute,
        boost::contract::instantiate,
        boost::contract::query,
    ));
    app.store_code(ve_boost_contract)
}

fn store_seilor_contract(app: &mut App) -> u64 {
    let seilor_contract = Box::new(ContractWrapper::new_with_empty(
        seilor::contract::execute,
        seilor::contract::instantiate,
        seilor::contract::query,
    ));
    app.store_code(seilor_contract)
}

fn store_fund_contract(app: &mut App) -> u64 {
    let fund_contract = Box::new(ContractWrapper::new_with_empty(
        fund::contract::execute,
        fund::contract::instantiate,
        fund::contract::query,
    ));
    app.store_code(fund_contract)
}

fn store_staking_reward_contract(app: &mut App) -> u64 {
    let staking_reward_contract =
        Box::new(ContractWrapper::new_with_empty(execute, instantiate, query));
    app.store_code(staking_reward_contract)
}

fn staking_token_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let staking_token_code_id = store_staking_token_contract(&mut app);
    let staking_token_instance_msg = mock_staking_token_instance_msg();
    let staking_token = app
        .instantiate_contract(
            staking_token_code_id,
            creator.clone(),
            &staking_token_instance_msg,
            &[], // no funds
            String::from("Staking Token"),
            None,
        )
        .unwrap();
    staking_token
}

fn boost_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let boost_code_id = store_boost_contract(&mut app);
    let boost_instance_msg = mock_boost_instance_msg();
    let boost = app
        .instantiate_contract(
            boost_code_id,
            creator.clone(),
            &boost_instance_msg,
            &[], // no funds
            String::from("Boost"),
            None,
        )
        .unwrap();
    boost
}

fn ve_seilor_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let ve_seilor_code_id = store_ve_seilor_token_contract(&mut app);
    let ve_seilor_instance_msg = mock_ve_seilor_instance_msg();
    let ve_seilor = app
        .instantiate_contract(
            ve_seilor_code_id,
            creator.clone(),
            &ve_seilor_instance_msg,
            &[], // no funds
            String::from("Ve Seilor"),
            None,
        )
        .unwrap();
    ve_seilor
}

fn seilor_instance(creator: &Addr, mut app: &mut App) -> Addr {
    let seilor_code_id = store_seilor_contract(&mut app);
    let seilor_instance_msg = mock_seilor_instance_msg();
    let seilor = app
        .instantiate_contract(
            seilor_code_id,
            creator.clone(),
            &seilor_instance_msg,
            &[], // no funds
            String::from("Seilor"),
            None,
        )
        .unwrap();
    seilor
}

fn fund_instance(creator: &Addr, mut app: &mut App, ve_seilor: &Addr, seilor: &Addr) -> Addr {
    let fund_code_id = store_fund_contract(&mut app);
    let fund_instance_msg = mock_fund_instance_msg(ve_seilor, seilor);
    let fund = app
        .instantiate_contract(
            fund_code_id,
            creator.clone(),
            &fund_instance_msg,
            &[], // no funds
            String::from("Fund"),
            None,
        )
        .unwrap();
    fund
}

fn starking_reward_instance(
    creator: &Addr,
    mut app: &mut App,
    staking_token: &Addr,
    ve_seilor: &Addr,
    boost: &Addr,
    fund: &Addr,
) -> Addr {
    let staking_reward_code_id = store_staking_reward_contract(&mut app);
    let instance_msg = mock_instantiate_msg(
        staking_token.clone(),
        ve_seilor.clone(),
        boost.clone(),
        fund.clone(),
    );
    let staking_reward = app
        .instantiate_contract(
            staking_reward_code_id,
            creator.clone(),
            &instance_msg,
            &[], // no funds
            String::from("Staking"),
            None,
        )
        .unwrap();
    staking_reward
}

#[test]
fn test_integration() {
    let creator = Addr::unchecked(CREATOR);
    let block_time = 10000000u64;
    let mut app = mock_app(creator.clone(), vec![], Some(block_time.clone()));

    let tom_address = Addr::unchecked("tom".to_string());

    // deploy staking token contract
    let staking_token = staking_token_instance(&creator, &mut app);
    println!("staking_token: {:?}", staking_token);

    // deploy ve_seilor token contract
    let ve_seilor = ve_seilor_instance(&creator, &mut app);
    println!("ve_seilor: {:?}", ve_seilor);

    // deploy seilor token contract
    let seilor = seilor_instance(&creator, &mut app);
    println!("seilor: {:?}", seilor);

    // deploy ve seilor boost contract
    let boost = boost_instance(&creator, &mut app);
    println!("boost: {:?}", boost);

    // deploy seilor fund contract
    let fund = fund_instance(&creator, &mut app, &ve_seilor, &seilor);
    println!("fund: {:?}", fund);

    // deploy staking reward contract
    let staking_reward = starking_reward_instance(
        &creator,
        &mut app,
        &staking_token,
        &ve_seilor,
        &boost,
        &fund,
    );

    // set seilor,ve_seilor fund contract address
    set_seilor_to_fund(&creator, &mut app, &seilor, &fund);

    set_ve_seilor_to_fund(&creator, &mut app, &ve_seilor, &fund);

    // set ve_seilor miner
    set_ve_seilor_miners(
        &creator,
        &mut app,
        &ve_seilor,
        vec![staking_reward.clone()],
        vec![true],
    );

    // send reward
    let reward_amount = Uint128::from(1000000u128);
    let kusd_reward_addr = Addr::unchecked(REWARD_CONTROLLER_ADDR.clone().to_string());
    notify_reward_amount(&kusd_reward_addr, &mut app, &staking_reward, &reward_amount);

    // query token staking token balance
    let balance = cw20_balance(&mut app, &staking_token, &tom_address);
    assert_eq!(balance.balance, Uint128::zero());

    let stake_amount = Uint128::from(1000000u128);

    stake(
        &tom_address,
        &mut app,
        &staking_token,
        &staking_reward,
        &stake_amount,
    );

    // send staking token to tom
    cw20_transfer(
        &mut app,
        &staking_token,
        &creator,
        &tom_address,
        Uint128::from(1000000u128),
    );
    // query token staking token balance
    let balance = cw20_balance(&mut app, &staking_token, &tom_address);
    assert_eq!(balance.balance, Uint128::from(1000000u128));
    // stake method
    stake(
        &tom_address,
        &mut app,
        &staking_token,
        &staking_reward,
        &stake_amount,
    );

    // user balance of
    let user_balance_of = balance_of(&mut app, &staking_reward, &tom_address);
    assert_eq!(user_balance_of, Uint128::from(1000000u128));

    // query token staking token balance
    let balance = cw20_balance(&mut app, &staking_token, &tom_address);
    assert_eq!(balance.balance, Uint128::zero());

    // check staking reward contract balance
    let balance = cw20_balance(&mut app, &staking_token, &staking_reward);
    assert_eq!(balance.balance, Uint128::from(1000000u128));

    // query reward per token
    let reward_per_token = reward_per_token(&mut app, &staking_reward);
    println!("reward_per_token: {:?}", reward_per_token);

    // query last time reward applicable
    let last_time_reward_applicable = last_time_reward_applicable(&mut app, &staking_reward);
    println!(
        "last_time_reward_applicable: {:?}",
        last_time_reward_applicable
    );
    // update block time
    app.update_block(|block| {
        block.time = block.time.plus_seconds(1000000u64);
        block.height += 1000000u64;
    });

    // get boost
    let boost = get_boost(&mut app, &staking_reward, &tom_address);
    assert_eq!(boost, Uint128::from(100000000u128));

    // query staking config
    let staking_config = query_staking_config(&mut app, &staking_reward);
    println!("staking_config: {:?}", staking_config);

    // query staking state
    let staking_state = query_staking_state(&mut app, &staking_reward);
    println!("staking_state: {:?}", staking_state);

    // get user updated at
    let user_update_at = get_user_updated_at(&mut app, &staking_reward, &tom_address);
    println!("user_update_at: {:?}", user_update_at);

    // get user reward per token paid
    let user_reward_per_token_paid =
        get_user_reward_per_token_paid(&mut app, &staking_reward, &tom_address);
    println!(
        "user_reward_per_token_paid: {:?}",
        user_reward_per_token_paid
    );
    //earned
    let query_earned_1 = earned(&mut app, &staking_reward, &tom_address);
    println!("query_earned: {:?}", query_earned_1);

    // check tom ve_seilor balance
    let balance = cw20_balance(&mut app, &ve_seilor, &tom_address);
    assert_eq!(balance.balance, Uint128::zero());

    // tom get reward
    get_reward(&tom_address, &mut app, &staking_reward);
    // check tom ve_seilor balance
    let balance = cw20_balance(&mut app, &ve_seilor, &tom_address);
    assert_eq!(balance.balance, query_earned_1);

    // withdraw stake
    let withdraw_amount = Uint128::from(500000u128);
    withdraw(&tom_address, &mut app, &staking_reward, &withdraw_amount);

    // user balance of
    let user_balance_of = balance_of(&mut app, &staking_reward, &tom_address);
    assert_eq!(user_balance_of, Uint128::from(500000u128));

    // update block time
    app.update_block(|block| {
        block.time = block.time.plus_seconds(1592000u64);
        block.height += 1000000u64;
    });

    let query_earned_2 = earned(&mut app, &staking_reward, &tom_address);
    println!("query_earned_2: {:?}", query_earned_2);

    // tom get reward
    get_reward(&tom_address, &mut app, &staking_reward);
    // check tom ve_seilor balance
    let balance = cw20_balance(&mut app, &ve_seilor, &tom_address);
    assert_eq!(balance.balance - query_earned_1, query_earned_2);

    // update block time
    app.update_block(|block| {
        block.time = block.time.plus_seconds(1592000u64);
        block.height += 1000000u64;
    });

    let query_earned = earned(&mut app, &staking_reward, &tom_address);
    assert_eq!(query_earned, Uint128::zero());

    // query staking state
    let staking_state = query_staking_state(&mut app, &staking_reward);
    println!("staking_state: {:?}", staking_state);

    notify_reward_amount(&kusd_reward_addr, &mut app, &staking_reward, &reward_amount);

    // query staking state
    let staking_state = query_staking_state(&mut app, &staking_reward);
    println!("staking_state: {:?}", staking_state);

    // update block time
    app.update_block(|block| {
        block.time = block.time.plus_seconds(1592000u64);
        block.height += 1000000u64;
    });

    let query_earned_3 = earned(&mut app, &staking_reward, &tom_address);
    assert_eq!(query_earned_3, query_earned_2);
}

fn reward_per_token(app: &mut App, staking_reward: &Addr) -> Uint128 {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::RewardPerToken {},
    );
    let res: RewardPerTokenResponse = res.unwrap();
    res.reward_per_token
}

fn last_time_reward_applicable(app: &mut App, staking_reward: &Addr) -> Uint128 {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::LastTimeRewardApplicable {},
    );
    let res: LastTimeRewardApplicableResponse = res.unwrap();
    res.last_time_reward_applicable
}

fn get_boost(app: &mut App, staking_reward: &Addr, user: &Addr) -> Uint128 {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::GetBoost {
            account: user.clone(),
        },
    );
    let res: GetBoostResponse = res.unwrap();
    res.boost
}

fn earned(app: &mut App, staking_reward: &Addr, user: &Addr) -> Uint128 {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::Earned {
            account: user.clone(),
        },
    );
    let res: EarnedResponse = res.unwrap();
    res.earned
}

fn query_staking_config(app: &mut App, staking_reward: &Addr) -> StakingConfigResponse {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::QueryStakingConfig {},
    );
    let res: StakingConfigResponse = res.unwrap();
    res
}

fn query_staking_state(app: &mut App, staking_reward: &Addr) -> StakingStateResponse {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::QueryStakingState {},
    );
    let res: StakingStateResponse = res.unwrap();
    res
}
fn get_user_updated_at(app: &mut App, staking_reward: &Addr, user: &Addr) -> Uint128 {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::GetUserUpdatedAt {
            account: user.clone(),
        },
    );
    let res: GetUserUpdatedAtResponse = res.unwrap();
    res.updated_at
}
fn get_user_reward_per_token_paid(app: &mut App, staking_reward: &Addr, user: &Addr) -> Uint128 {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::GetUserRewardPerTokenPaid {
            account: user.clone(),
        },
    );
    let res: GetUserRewardPerTokenPaidResponse = res.unwrap();
    res.reward_per_token_paid
}

fn balance_of(app: &mut App, staking_reward: &Addr, user: &Addr) -> Uint128 {
    let res = app.wrap().query_wasm_smart(
        staking_reward.clone().to_string(),
        &QueryMsg::BalanceOf {
            account: user.clone(),
        },
    );
    let res: BalanceOfResponse = res.unwrap();
    res.balance_of
}

fn set_ve_seilor_miners(
    creator: &Addr,
    app: &mut App,
    ve_seilor: &Addr,
    contracts: Vec<Addr>,
    is_minter: Vec<bool>,
) {
    let ve_seilor_miner_msg = ve_seilor::msg::ExecuteMsg::SetMinters {
        contracts,
        is_minter,
    };
    let res = app.execute_contract(
        creator.clone(),
        ve_seilor.clone(),
        &ve_seilor_miner_msg,
        &[],
    );
    assert!(res.is_ok());
}

fn set_ve_seilor_to_fund(creator: &Addr, app: &mut App, ve_seilor: &Addr, fund: &Addr) {
    let ve_seilor_config = ve_seilor::msg::ExecuteMsg::UpdateConfig {
        max_minted: None,
        fund: Some(fund.clone()),
        gov: None,
    };
    let res = app.execute_contract(creator.clone(), ve_seilor.clone(), &ve_seilor_config, &[]);
    assert!(res.is_ok());
}

fn set_seilor_to_fund(creator: &Addr, app: &mut App, seilor: &Addr, fund: &Addr) {
    let update_seilor_fund_msg = seilor::msg::ExecuteMsg::UpdateConfig {
        fund: Some(fund.clone()),
        distribute: None,
    };
    let res = app.execute_contract(
        creator.clone(),
        seilor.clone(),
        &update_seilor_fund_msg,
        &[],
    );
    assert!(res.is_ok());
}

fn notify_reward_amount(
    creator: &Addr,
    app: &mut App,
    staking_reward: &Addr,
    reward_amount: &Uint128,
) {
    let reward_msg = NotifyRewardAmount {
        amount: reward_amount.clone(),
    };
    let res = app.execute_contract(
        creator.clone(),
        staking_reward.clone(),
        &reward_msg,
        &[], // no funds
    );
    assert!(res.is_ok());
}

fn get_reward(user: &Addr, app: &mut App, staking_reward: &Addr) {
    let res = app.execute_contract(
        user.clone(),
        staking_reward.clone(),
        &ExecuteMsg::GetReward {},
        &[], // no funds
    );
    assert!(res.is_ok());
}

fn withdraw(user: &Addr, app: &mut App, staking_reward: &Addr, amount: &Uint128) {
    let res = app.execute_contract(
        user.clone(),
        staking_reward.clone(),
        &ExecuteMsg::Withdraw {
            amount: amount.clone(),
        },
        &[], // no funds
    );
    println!("{:?}", res);
    assert!(res.is_ok());
}

fn cw20_balance(app: &mut App, cw20_token: &Addr, user: &Addr) -> BalanceResponse {
    let query_msg = cw20_base::msg::QueryMsg::Balance {
        address: user.clone().to_string(),
    };

    let res: BalanceResponse = app
        .wrap()
        .query_wasm_smart(cw20_token.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn cw20_transfer(
    app: &mut App,
    cw20_token: &Addr,
    sender: &Addr,
    recipient: &Addr,
    amount: Uint128,
) {
    let transfer_msg = cw20_base::msg::ExecuteMsg::Transfer {
        recipient: recipient.clone().to_string(),
        amount,
    };
    let res = app.execute_contract(sender.clone(), cw20_token.clone(), &transfer_msg, &[]);
    assert!(res.is_ok());
}

fn stake(
    creator: &Addr,
    app: &mut App,
    staking_token: &Addr,
    staking_reward: &Addr,
    stake_amount: &Uint128,
) {
    let stake_send_seilor_msg = cw20_base::msg::ExecuteMsg::Send {
        contract: staking_reward.clone().to_string(),
        amount: stake_amount.clone(),
        msg: to_binary(&Stake {}).unwrap(),
    };
    let res = app.execute_contract(
        creator.clone(),
        staking_token.clone(),
        &stake_send_seilor_msg,
        &[],
    );
    if res.is_err() {
        println!("{:?}", res);
        assert!(res.is_err());
    } else {
        println!("stake success");
        assert!(res.is_ok());
    }
}
