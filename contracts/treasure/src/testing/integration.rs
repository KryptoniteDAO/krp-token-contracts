use crate::helper::BASE_RATE_12;
use crate::msg::{ConfigInfosResponse, UserInfosResponse};
use crate::testing::mock_fn::{CREATOR, PUNISH_RECEIVER};
use crate::testing::mock_third_fn::mock_cw20_instantiate_msg;
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{to_binary, Addr, Coin, Response, StdError, StdResult, Timestamp, Uint128};
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

#[test]
fn test_integration() {
    let block_time = 1688128676u64;
    let creator = Addr::unchecked(CREATOR);
    let tom_address = Addr::unchecked("tom");
    let punish_receiver_address = Addr::unchecked(PUNISH_RECEIVER.clone());
    let mut app = mock_app(creator.clone(), vec![], Some(block_time));

    // init cw20 token
    let cw20_contract_id = store_cw20_contract(&mut app);
    let cw20instance_msg: cw20_base::msg::InstantiateMsg = mock_cw20_instantiate_msg();
    let cw20_token = app
        .instantiate_contract(
            cw20_contract_id,
            creator.clone(),
            &cw20instance_msg,
            &[], // no funds
            String::from("cw20_token"),
            None,
        )
        .unwrap();

    // init treasure contract
    let treasure_contract_id = store_treasure_contract(&mut app);
    let treasure_instance_msg: crate::msg::InstantiateMsg =
        crate::testing::mock_fn::mock_instantiate_msg(cw20_token.clone());
    let treasure_contact = app
        .instantiate_contract(
            treasure_contract_id,
            creator.clone(),
            &treasure_instance_msg,
            &[], // no funds
            String::from("treasure_contact"),
            None,
        )
        .unwrap();

    // tom lock 0 token
    let res = user_lock(
        &tom_address,
        &mut app,
        &cw20_token,
        &treasure_contact,
        &Uint128::zero(),
    );
    assert!(res.is_err());

    let tom_balance = get_token_balance(&mut app, &cw20_token, &tom_address);
    assert_eq!(tom_balance.balance, Uint128::zero());
    // transfer tom 100_000_000_000 token
    let transfer_amount = Uint128::from(10_000_000_000_000u128);
    let res = transfer_token(
        &creator,
        &tom_address,
        &mut app,
        &cw20_token,
        transfer_amount.clone(),
    );
    assert!(res.is_ok());
    let tom_balance = get_token_balance(&mut app, &cw20_token, &tom_address);
    assert_eq!(tom_balance.balance, transfer_amount);

    let tom_lock_amount = Uint128::from(100_000_000u128);
    // tom lock 100_000_000 token not start time
    let res = user_lock(
        &tom_address,
        &mut app,
        &cw20_token,
        &treasure_contact,
        &tom_lock_amount,
    );
    assert!(res.is_err());

    // update block time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1688128677 + 1000u64);
        block.height += 1000000u64;
    });

    // tom lock 100_000_000 token
    let res = user_lock(
        &tom_address,
        &mut app,
        &cw20_token,
        &treasure_contact,
        &tom_lock_amount,
    );
    assert!(res.is_ok());

    // check tom balance
    let tom_balance = get_token_balance(&mut app, &cw20_token, &tom_address);
    assert_eq!(tom_balance.balance, transfer_amount - tom_lock_amount);

    // check treasure contract balance
    let treasure_balance = get_token_balance(&mut app, &cw20_token, &treasure_contact);
    assert_eq!(treasure_balance.balance, tom_lock_amount);

    let user_state = query_user_infos(&mut app, &treasure_contact, &tom_address).user_state;
    assert_eq!(user_state.current_dust_amount, Uint128::zero());
    assert_eq!(user_state.current_locked_amount, tom_lock_amount);
    assert_eq!(user_state.total_locked_amount, tom_lock_amount);
    assert_eq!(user_state.last_lock_time, 1688128677 + 1000u64);

    let global_state = query_config_infos(&mut app, &treasure_contact).state;
    assert_eq!(global_state.total_locked_amount, tom_lock_amount);
    assert_eq!(global_state.current_locked_amount, tom_lock_amount);

    // tom mint nft error,not enough locked token
    let res = pre_mint_nft(&mut app, &treasure_contact, &tom_address, 1);
    assert!(res.is_err());

    // tom lock 9_000_000_000 token
    let tom_lock_2_amount = Uint128::from(9_000_000_000u128);
    let res = user_lock(
        &tom_address,
        &mut app,
        &cw20_token,
        &treasure_contact,
        &tom_lock_2_amount,
    );
    assert!(res.is_ok());

    // check tom balance
    let tom_balance = get_token_balance(&mut app, &cw20_token, &tom_address);
    assert_eq!(
        tom_balance.balance,
        transfer_amount - tom_lock_amount - tom_lock_2_amount
    );

    // check treasure contract balance
    let treasure_balance = get_token_balance(&mut app, &cw20_token, &treasure_contact);
    assert_eq!(
        treasure_balance.balance,
        tom_lock_amount + tom_lock_2_amount
    );

    // check global info
    let global_info = query_config_infos(&mut app, &treasure_contact);
    assert_eq!(
        global_info.state.total_locked_amount,
        tom_lock_amount + tom_lock_2_amount
    );

    assert_eq!(
        global_info.state.current_locked_amount,
        tom_lock_amount + tom_lock_2_amount
    );
    assert_eq!(global_info.state.total_lose_nft_num, 0u64);
    assert_eq!(global_info.state.total_win_nft_num, 0u64);
    assert_eq!(global_info.state.total_withdraw_amount, Uint128::zero());
    assert_eq!(global_info.state.total_punish_amount, Uint128::zero());

    //check tom state

    let tom_state = query_user_infos(&mut app, &treasure_contact, &tom_address);
    let user_state = tom_state.user_state;
    assert_eq!(
        user_state.current_locked_amount,
        tom_lock_amount + tom_lock_2_amount
    );
    assert_eq!(user_state.current_dust_amount, Uint128::zero());
    assert_eq!(
        user_state.total_locked_amount,
        tom_lock_amount + tom_lock_2_amount
    );
    assert_eq!(user_state.total_cost_dust_amount, Uint128::zero());

    assert_eq!(user_state.total_withdraw_amount, Uint128::zero());
    assert_eq!(user_state.total_withdraw_amount, Uint128::zero());
    assert_eq!(user_state.win_nft_num, 0u64);
    assert_eq!(user_state.lose_nft_num, 0u64);
    assert_eq!(user_state.last_lock_time, 1688128677 + 1000u64);

    // tom withdraw 1_000_000_000 token error not unlock
    let tom_withdraw_amount = Uint128::from(1_000_000_000u128);
    let res = user_withdraw(
        &mut app,
        &treasure_contact,
        &tom_address,
        &tom_withdraw_amount,
    );
    assert!(res.is_err());

    // update block time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1688128677 + 2000u64);
        block.height += 1000000u64;
    });

    // tom unlock 100_000_000 token
    let tom_unlock_amount = Uint128::from(1_000_000_000u128);
    let res = user_unlock(
        &mut app,
        &treasure_contact,
        &tom_address,
        &tom_unlock_amount,
    );
    assert!(res.is_ok());

    let user_state = query_user_infos(&mut app, &treasure_contact, &tom_address).user_state;
    assert_eq!(
        user_state.current_locked_amount,
        tom_lock_amount + tom_lock_2_amount - tom_unlock_amount
    );
    assert_eq!(
        user_state.current_dust_amount,
        global_info.config.dust_reward_per_second
            * Uint128::from(1000u128)
            * (tom_lock_amount + tom_lock_2_amount)
            / Uint128::from(BASE_RATE_12)
    );
    assert_eq!(user_state.current_unlock_amount, tom_unlock_amount);

    let global_info = query_config_infos(&mut app, &treasure_contact);
    assert_eq!(
        global_info.state.current_locked_amount,
        tom_lock_amount + tom_lock_2_amount - tom_unlock_amount
    );
    assert_eq!(global_info.state.current_unlock_amount, tom_unlock_amount);

    // tom withdraw 100_000_000 token will punish
    let tom_withdraw_amount = Uint128::from(100_000_000u128);

    let res = user_withdraw(
        &mut app,
        &treasure_contact,
        &tom_address,
        &tom_withdraw_amount,
    );
    assert!(res.is_ok());
    // check user state
    let user_state = query_user_infos(&mut app, &treasure_contact, &tom_address).user_state;
    let punish_amount = tom_withdraw_amount * global_info.config.no_delay_punish_coefficient
        / Uint128::from(1_000_000u128);
    assert_eq!(
        user_state.current_locked_amount,
        tom_lock_amount + tom_lock_2_amount - tom_unlock_amount
    );
    assert_eq!(
        user_state.current_unlock_amount,
        tom_unlock_amount - tom_withdraw_amount
    );
    assert_eq!(user_state.total_punish_amount, punish_amount);
    // check global state
    let global_info = query_config_infos(&mut app, &treasure_contact);
    assert_eq!(
        global_info.state.current_locked_amount,
        tom_lock_amount + tom_lock_2_amount - tom_unlock_amount
    );
    assert_eq!(
        global_info.state.current_unlock_amount,
        tom_unlock_amount - tom_withdraw_amount
    );
    assert_eq!(global_info.state.total_punish_amount, punish_amount);

    // check punish receiver balance
    let punish_receiver_balance =
        get_token_balance(&mut app, &cw20_token, &punish_receiver_address);
    assert_eq!(punish_receiver_balance.balance, punish_amount);

    // update block time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1688128677 + 2000u64 + 86400 * 14);
        block.height += 1000000u64;
    });

    // tom withdraw 200_000_000 token will not punish
    let tom_withdraw_amount_2 = Uint128::from(200_000_000u128);
    let res = user_withdraw(
        &mut app,
        &treasure_contact,
        &tom_address,
        &tom_withdraw_amount_2,
    );
    assert!(res.is_ok());
    // check user state
    let user_state = query_user_infos(&mut app, &treasure_contact, &tom_address).user_state;
    assert_eq!(
        user_state.current_unlock_amount,
        tom_unlock_amount - tom_withdraw_amount - tom_withdraw_amount_2
    );
    assert_eq!(user_state.total_punish_amount, punish_amount);

    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1690720710 + 2000u64);
        block.height += 1000000u64;
    });
    let user_state = query_user_infos(&mut app, &treasure_contact, &tom_address).user_state;
    let tom_current_dust_amount = user_state.current_dust_amount;
    println!("tom_current_dust_amount: {}", tom_current_dust_amount);
    // pre nft mint
    let pre_nft_mint_amount = 1u64;
    let res = pre_mint_nft(
        &mut app,
        &treasure_contact,
        &tom_address,
        pre_nft_mint_amount.clone(),
    );
    assert!(res.is_ok());
    // check user state
    let user_state = query_user_infos(&mut app, &treasure_contact, &tom_address).user_state;
    let global_info = query_config_infos(&mut app, &treasure_contact);
    let pre_nft_mint_cost_dust = global_info.config.mint_nft_cost_dust * Uint128::one();
    assert_eq!(
        user_state.current_dust_amount,
        tom_current_dust_amount - pre_nft_mint_cost_dust
    );
    assert_eq!(user_state.lose_nft_num, 1u64);
    assert_eq!(
        user_state.total_cost_dust_amount,
        global_info.state.total_cost_dust_amount
    );
}

fn transfer_token(
    from: &Addr,
    to: &Addr,
    app: &mut App,
    cw20_token: &Addr,
    amount: Uint128,
) -> StdResult<Response> {
    let send_msg = cw20_base::msg::ExecuteMsg::Transfer {
        recipient: to.to_string(),
        amount,
    };
    let res = app.execute_contract(
        from.clone(),
        cw20_token.clone(),
        &send_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("transfer_token error: {:?}", res);
        Err(StdError::generic_err("transfer_token error"))
    } else {
        Ok(Response::default())
    }
}

fn user_unlock(
    app: &mut App,
    treasure_contact: &Addr,
    user: &Addr,
    amount: &Uint128,
) -> StdResult<Response> {
    let user_withdraw_msg = crate::msg::ExecuteMsg::UserUnlock {
        amount: amount.clone(),
    };
    let res = app.execute_contract(
        user.clone(),
        treasure_contact.clone(),
        &user_withdraw_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("user_unlock error: {:?}", res);
        Err(StdError::generic_err("user_unlock error"))
    } else {
        Ok(Response::default())
    }
}
fn user_withdraw(
    app: &mut App,
    treasure_contact: &Addr,
    user: &Addr,
    amount: &Uint128,
) -> StdResult<Response> {
    let user_withdraw_msg = crate::msg::ExecuteMsg::UserWithdraw {
        amount: amount.clone(),
    };
    let res = app.execute_contract(
        user.clone(),
        treasure_contact.clone(),
        &user_withdraw_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("user_withdraw error: {:?}", res);
        Err(StdError::generic_err("user_withdraw error"))
    } else {
        Ok(Response::default())
    }
}

fn pre_mint_nft(
    app: &mut App,
    treasure_contact: &Addr,
    user: &Addr,
    mint_num: u64,
) -> StdResult<Response> {
    let pre_mint_nft_msg = crate::msg::ExecuteMsg::PreMintNft { mint_num };
    let res = app.execute_contract(
        user.clone(),
        treasure_contact.clone(),
        &pre_mint_nft_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("pre_mint_nft error: {:?}", res);
        Err(StdError::generic_err("pre_mint_nft error"))
    } else {
        Ok(Response::default())
    }
}

fn user_lock(
    user: &Addr,
    app: &mut App,
    cw20_token: &Addr,
    treasure_contact: &Addr,
    amount: &Uint128,
) -> StdResult<Response> {
    let send_msg = cw20_base::msg::ExecuteMsg::Send {
        contract: treasure_contact.clone().to_string(),
        amount: amount.clone(),
        msg: to_binary(&crate::msg::Cw20HookMsg::UserLockHook {}).unwrap(),
    };
    let res = app.execute_contract(
        user.clone(),
        cw20_token.clone(),
        &send_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("user_lock error: {:?}", res);
        Err(StdError::generic_err("user_lock error"))
    } else {
        Ok(Response::default())
    }
}

fn get_token_balance(app: &mut App, token: &Addr, user: &Addr) -> BalanceResponse {
    let query_msg = cw20_base::msg::QueryMsg::Balance {
        address: user.clone().to_string(),
    };

    let res: BalanceResponse = app
        .wrap()
        .query_wasm_smart(token.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn query_config_infos(app: &mut App, treasure_contact: &Addr) -> ConfigInfosResponse {
    let query_msg = crate::msg::QueryMsg::QueryConfigInfos {};
    let res: ConfigInfosResponse = app
        .wrap()
        .query_wasm_smart(treasure_contact.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn query_user_infos(app: &mut App, treasure_contact: &Addr, user: &Addr) -> UserInfosResponse {
    let query_msg = crate::msg::QueryMsg::QueryUserInfos { user: user.clone() };
    let res: UserInfosResponse = app
        .wrap()
        .query_wasm_smart(treasure_contact.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn store_treasure_contract(app: &mut App) -> u64 {
    let contract = Box::new(ContractWrapper::new_with_empty(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    ));
    app.store_code(contract)
}

fn store_cw20_contract(app: &mut App) -> u64 {
    let contract = Box::new(ContractWrapper::new_with_empty(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    ));
    app.store_code(contract)
}
