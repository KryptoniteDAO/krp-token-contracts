use cosmwasm_std::{Addr, attr, BankMsg, coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult, SubMsg, to_binary, Uint128, Uint256, Uint64, WasmMsg};
use crate::msg::UpdateConfigMsg;
use crate::querier::{earned, get_claim_able_kpt, get_reserved_kpt_for_vesting, total_staked};
use crate::state::{KptFundConfig, read_kpt_fund_config, read_rewards, read_time2full_redemption, read_unstake_rate, store_kpt_fund_config, store_last_withdraw_time, store_rewards, store_time2full_redemption, store_unstake_rate, store_user_reward_per_token_paid};
use crate::third_msg::{KptExecuteMsg, VeKptExecuteMsg};

/**
 * This is a function that updates the configuration of a KPT Fund contract.
 * The function takes in several optional parameters, including the address of the VE-KPT contract,
 * the address of the KPT contract, the denomination of the KUSD token, the reward per token stored,
 * the exit cycle, and the claimable time. If the sender is not authorized to update the configuration,
 * an error will be returned. The function then updates the configuration with the new values and stores it in the contract's storage.
 * Finally, it returns a response with attributes indicating the action taken and the parameters updated.
 */
pub fn update_kpt_fund_config(
    deps: DepsMut,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> StdResult<Response> {
    let mut config: KptFundConfig = read_kpt_fund_config(deps.storage)?;
    if info.sender != config.gov {
        return Err(StdError::generic_err("unauthorized"));
    }
    let mut attrs = vec![attr("action", "update_kpt_fund_config"), attr("sender", info.sender.to_string())];
    if let Some(gov) = msg.gov {
        config.gov = gov.clone();
        attrs.push(attr("gov", gov.to_string()));
    }
    if let Some(ve_kpt_addr) = msg.ve_kpt_addr {
        config.ve_kpt_addr = ve_kpt_addr.clone();
        attrs.push(attr("ve_kpt_addr", ve_kpt_addr.to_string()));
    }
    if let Some(kpt_addr) = msg.kpt_addr {
        config.kpt_addr = kpt_addr.clone();
        attrs.push(attr("kpt_addr", kpt_addr.to_string()));
    }
    if let Some(kusd_denom) = msg.kusd_denom {
        config.kusd_denom = kusd_denom.clone();
        attrs.push(attr("kusd_denom", kusd_denom));
    }
    if let Some(kusd_reward_addr) = msg.kusd_reward_addr {
        config.kusd_reward_addr = kusd_reward_addr.clone();
        attrs.push(attr("kusd_reward_addr", kusd_reward_addr.to_string()));
    }

    if let Some(exit_cycle) = msg.exit_cycle {
        config.exit_cycle = exit_cycle.clone();
        attrs.push(attr("exit_cycle", exit_cycle.to_string()));
    }
    if let Some(claim_able_time) = msg.claim_able_time {
        config.claim_able_time = claim_able_time.clone();
        attrs.push(attr("claim_able_time", claim_able_time.to_string()));
    }
    store_kpt_fund_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(attrs))
}


fn _update_reward(deps: DepsMut, account: Addr) -> StdResult<()> {
    let user_rewards = earned(deps.as_ref(), account.clone()).unwrap().amount;
    store_rewards(deps.storage, account.clone(), &user_rewards)?;
    let config: KptFundConfig = read_kpt_fund_config(deps.storage).unwrap();
    store_user_reward_per_token_paid(deps.storage, account.clone(), &config.reward_per_token_stored)?;
    Ok(())
}

/**
 * This is a function that updates the reward of a user.
 * The function takes in the address of the user as a parameter.
 * The function then updates the reward of the user and stores it in the contract's storage.
 * Finally, it returns a response with attributes indicating the action taken and the user's address.
 */
pub fn refresh_reward(
    deps: DepsMut,
    account: Addr,
) -> StdResult<Response> {
    _update_reward(deps, account.clone())?;
    Ok(Response::new().add_attributes(vec![attr("action", "refresh_reward"), attr("account", account.to_string())]))
}

pub fn stake(
    mut deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    refresh_reward(deps.branch(), info.sender.clone())?;
    let sender = info.sender;
    let config: KptFundConfig = read_kpt_fund_config(deps.storage).unwrap();
    let mut sub_msgs = vec![];
    let kpt_burn_msg = KptExecuteMsg::Burn {
        user: sender.clone().to_string(),
        amount: amount.clone(),
    };
    let sub_burn_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.kpt_addr.to_string(),
        msg: to_binary(&kpt_burn_msg)?,
        funds: vec![],
    }));
    sub_msgs.push(sub_burn_msg);

    let kpt_mint_msg = VeKptExecuteMsg::Mint {
        recipient: sender.clone().to_string(),
        amount: amount.clone(),
    };

    let sub_mint_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.ve_kpt_addr.to_string(),
        msg: to_binary(&kpt_mint_msg)?,
        funds: vec![],
    }));
    sub_msgs.push(sub_mint_msg);

    Ok(Response::new()
        .add_submessages(sub_msgs)
        .add_attributes(vec![
            attr("action", "stake"),
            attr("sender", sender.to_string()),
            attr("amount", amount.to_string()),
        ]))
}

pub fn unstake(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    refresh_reward(deps.branch(), info.sender.clone())?;

    let sender = info.sender;
    let config: KptFundConfig = read_kpt_fund_config(deps.storage).unwrap();
    let current_time = Uint64::from(env.block.time.seconds());
    if current_time.le(&config.claim_able_time) {
        return Err(StdError::generic_err("It is not yet time to claim."));
    }

    let mut sub_msgs = vec![];
    let ve_kpt_burn_msg = VeKptExecuteMsg::Burn {
        user: sender.clone().to_string(),
        amount: amount.clone(),
    };
    let sub_burn_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.ve_kpt_addr.to_string(),
        msg: to_binary(&ve_kpt_burn_msg)?,
        funds: vec![],
    }));
    sub_msgs.push(sub_burn_msg);

    withdraw(deps.branch(), env.clone(), sender.clone())?;

    let mut total = Uint256::from(amount.clone());
    let time2full_redemption_user = read_time2full_redemption(deps.storage, sender.clone());
    if time2full_redemption_user.gt(&current_time) {
        let unstake_rate_user = read_unstake_rate(deps.storage, sender.clone());
        let diff_time = time2full_redemption_user.checked_sub(current_time).unwrap();
        total = total.checked_add(unstake_rate_user.multiply_ratio(Uint256::from(diff_time),Uint256::from(1000000000000u128))).unwrap();
    }

    // let user_new_unstake_rate = total.checked_div(Uint128::from(config.exit_cycle)).unwrap();
    let user_new_unstake_rate = total.multiply_ratio(Uint256::from(1000000000000u128), Uint256::from(config.exit_cycle));
    let user_new_time2full_redemption = current_time.checked_add(config.exit_cycle).unwrap();

    store_unstake_rate(deps.storage, sender.clone(), &user_new_unstake_rate)?;
    store_time2full_redemption(deps.storage, sender.clone(), &user_new_time2full_redemption)?;

    Ok(Response::new()
        .add_submessages(sub_msgs)
        .add_attributes(vec![
            attr("action", "unstake"),
            attr("sender", sender.to_string()),
            attr("amount", amount.to_string()),
        ]))
}

/**
 * This is a function that allows a user to withdraw their claimable KPT tokens.
 * First, it retrieves the amount of claimable tokens using the get_claim_able_kpt function.
 * If there is an error, it returns a generic error message.
 * If there are tokens to withdraw, it reads the KPT fund configuration and creates a KptExecuteMsg to mint the tokens to the user's address.
 * This message is added as a sub-message to the response.
 * Finally, the function stores the current block time as the user's last withdrawal time and returns a response with attributes indicating the action, user, and amount withdrawn.
 */
pub fn withdraw(
    deps: DepsMut,
    env: Env,
    user: Addr,
) -> StdResult<Response> {
    let current_time = Uint64::from(env.block.time.seconds());
    let claim_able_res = get_claim_able_kpt(deps.as_ref(), env, user.clone());
    if claim_able_res.is_err() {
        return Err(StdError::generic_err("get claim able kpt error"));
    }
    let amount = claim_able_res.unwrap().amount;
    let mut sub_msgs = vec![];
    if amount.gt(&Uint128::zero()) {
        let config: KptFundConfig = read_kpt_fund_config(deps.storage).unwrap();
        let kpt_mint_msg = KptExecuteMsg::Mint {
            recipient: user.clone().to_string(),
            amount: amount.clone(),
        };
        let sub_mint_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.kpt_addr.to_string(),
            msg: to_binary(&kpt_mint_msg)?,
            funds: vec![],
        }));
        sub_msgs.push(sub_mint_msg);
    }

    store_last_withdraw_time(deps.storage, user.clone(), &Uint64::from(current_time))?;

    Ok(Response::new()
        .add_submessages(sub_msgs)
        .add_attributes(vec![
            attr("action", "withdraw"),
            attr("user", user.to_string()),
            attr("amount", amount.to_string()),
        ]))
}

pub fn re_stake(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> StdResult<Response> {
    let sender = info.sender;
    _update_reward(deps.branch(), sender.clone())?;

    let mut sub_msgs = vec![];
    let config: KptFundConfig = read_kpt_fund_config(deps.storage).unwrap();
    let claim_able_res = get_claim_able_kpt(deps.as_ref(), env.clone(), sender.clone());
    if claim_able_res.is_err() {
        return Err(StdError::generic_err("get claim able kpt error"));
    }
    let reserve_kpt_res = get_reserved_kpt_for_vesting(deps.as_ref(), env.clone(), sender.clone());
    if reserve_kpt_res.is_err() {
        return Err(StdError::generic_err("get reserve kpt error"));
    }
    let claim_able = claim_able_res.unwrap().amount;
    let reserve_kpt = reserve_kpt_res.unwrap().amount;
    let total = claim_able.checked_add(reserve_kpt).unwrap();
    if total.gt(&Uint128::zero()) {
        let kpt_mint_msg = VeKptExecuteMsg::Mint {
            recipient: sender.clone().to_string(),
            amount: total.clone(),
        };
        let sub_mint_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.ve_kpt_addr.to_string(),
            msg: to_binary(&kpt_mint_msg)?,
            funds: vec![],
        }));
        sub_msgs.push(sub_mint_msg);
    }

    store_unstake_rate(deps.storage, sender.clone(), &Uint256::zero())?;
    store_time2full_redemption(deps.storage, sender.clone(), &Uint64::zero())?;

    Ok(Response::new()
        .add_submessages(sub_msgs)
        .add_attributes(vec![
            attr("action", "re_stake"),
            attr("sender", sender.to_string()),
            attr("amount", total.to_string()),
        ]))
}

pub fn get_reward(mut deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender = info.sender;
    _update_reward(deps.branch(), sender.clone())?;

    let reward = read_rewards(deps.storage, sender.clone());
    let mut messages = vec![];
    if reward.gt(&Uint128::zero()) {
        store_rewards(deps.storage, sender.clone(), &Uint128::zero())?;

        let mut config: KptFundConfig = read_kpt_fund_config(deps.storage).unwrap();

        let send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: sender.to_string(),
            amount: vec![coin(reward.u128(), config.kusd_denom.to_string())],
        });

        messages.push(send_msg);

        config.kusd_reward_total_paid_amount = config.kusd_reward_total_paid_amount.checked_add(reward.clone()).unwrap();

        store_kpt_fund_config(deps.storage, &config)?;
    }
    return Ok(Response::new()
        .add_messages(messages)
        .add_attributes(vec![
            attr("action", "get_reward"),
            attr("sender", sender.to_string()),
            attr("reward", reward.to_string()),
        ]));
}


/**
 * @dev The amount of KUSD acquiered from the sender is euitably distributed to KPT stakers.
 * Calculate share by amount, and calculate the shares could claim by per unit of staked Sei.
 * Add into rewardPerTokenStored.
 */
pub fn notify_reward_amount(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender = info.sender;
    let mut config = read_kpt_fund_config(deps.storage)?;
    if sender.clone().ne(&config.kusd_reward_addr) {
        return Err(StdError::generic_err("only kusd reward addr can notify reward amount"));
    }

    let total_staked = total_staked(deps.as_ref());
    if total_staked.eq(&Uint128::zero()) {
        return Ok(Response::new());
    }
    let payment = info.funds.iter()
        .find(|x| x.denom.eq(&config.kusd_denom))
        .ok_or_else(|| StdError::generic_err("kusd denom not found"))?;

    let amount = payment.amount;

    if amount.le(&Uint128::zero()) {
        return Err(StdError::generic_err("kusd amount is zero"));
    }

    let inc_reward_per_token = amount.multiply_ratio(Uint128::new(1000000u128), total_staked);
    config.reward_per_token_stored = config.reward_per_token_stored.checked_add(inc_reward_per_token).unwrap();
    config.kusd_reward_total_amount = config.kusd_reward_total_amount.checked_add(amount).unwrap();

    store_kpt_fund_config(deps.storage, &config)?;

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "notify_reward_amount"),
            attr("amount", amount.to_string()),
            attr("sender", sender.to_string()),
        ]))
}