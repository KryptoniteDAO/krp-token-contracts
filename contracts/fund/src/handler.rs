use crate::helper::{BASE_RATE_12, BASE_RATE_6};
use crate::msg::{Cw20HookMsg, UpdateConfigMsg};
use crate::querier::{
    earned, get_claim_able_seilor, get_reserved_seilor_for_vesting, total_staked,
};
use crate::state::{
    read_fund_config, read_rewards, read_time2full_redemption, read_unstake_rate,
    store_fund_config, store_last_withdraw_time, store_rewards, store_time2full_redemption,
    store_unstake_rate, store_user_reward_per_token_paid, FundConfig,
};
use cosmwasm_std::{
    attr, coin, from_binary, to_binary, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, SubMsg, Uint128, Uint256, Uint64, WasmMsg,
};
use cw20::Cw20ReceiveMsg;

/**
 * This is a function that updates the configuration of a SEILOR Fund contract.
 * The function takes in several optional parameters, including the address of the VE-SEILOR contract,
 * the address of the SEILOR contract, the denomination of the KUSD token, the reward per token stored,
 * the exit cycle, and the claimable time. If the sender is not authorized to update the configuration,
 * an error will be returned. The function then updates the configuration with the new values and stores it in the contract's storage.
 * Finally, it returns a response with attributes indicating the action taken and the parameters updated.
 */
pub fn update_fund_config(
    deps: DepsMut,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> StdResult<Response> {
    let mut config: FundConfig = read_fund_config(deps.storage)?;
    if info.sender != config.gov {
        return Err(StdError::generic_err("unauthorized"));
    }
    let mut attrs = vec![
        attr("action", "update_fund_config"),
        attr("sender", info.sender.to_string()),
    ];
    if let Some(ve_seilor_addr) = msg.ve_seilor_addr {
        deps.api.addr_validate(ve_seilor_addr.clone().as_str())?;
        config.ve_seilor_addr = ve_seilor_addr.clone();
        attrs.push(attr("ve_seilor_addr", ve_seilor_addr.to_string()));
    }
    if let Some(seilor_addr) = msg.seilor_addr {
        deps.api.addr_validate(seilor_addr.clone().as_str())?;
        config.seilor_addr = seilor_addr.clone();
        attrs.push(attr("seilor_addr", seilor_addr.to_string()));
    }
    if let Some(kusd_denom) = msg.kusd_denom {
        config.kusd_denom = kusd_denom.clone();
        attrs.push(attr("kusd_denom", kusd_denom));
    }
    if let Some(kusd_reward_addr) = msg.kusd_reward_addr {
        deps.api.addr_validate(kusd_reward_addr.clone().as_str())?;
        config.kusd_reward_addr = kusd_reward_addr.clone();
        attrs.push(attr("kusd_reward_addr", kusd_reward_addr.to_string()));
    }
    if let Some(claim_able_time) = msg.claim_able_time {
        config.claim_able_time = claim_able_time.clone();
        attrs.push(attr("claim_able_time", claim_able_time.to_string()));
    }
    store_fund_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(attrs))
}

fn _update_reward(deps: DepsMut, account: Addr) -> StdResult<()> {
    let user_rewards = earned(deps.as_ref(), account.clone())?.amount;
    store_rewards(deps.storage, account.clone(), &user_rewards)?;
    let config = read_fund_config(deps.storage)?;
    store_user_reward_per_token_paid(
        deps.storage,
        account.clone(),
        &config.reward_per_token_stored,
    )?;
    Ok(())
}

/**
 * This is a function that updates the reward of a user.
 * The function takes in the address of the user as a parameter.
 * The function then updates the reward of the user and stores it in the contract's storage.
 * Finally, it returns a response with attributes indicating the action taken and the user's address.
 */
pub fn refresh_reward(deps: DepsMut, account: Addr) -> StdResult<Response> {
    _update_reward(deps, account.clone())?;
    Ok(Response::new().add_attributes(vec![
        attr("action", "refresh_reward"),
        attr("account", account.to_string()),
    ]))
}

pub fn stake(mut deps: DepsMut, sender: Addr, amount: Uint128) -> StdResult<Response> {
    refresh_reward(deps.branch(), sender.clone())?;
    let config = read_fund_config(deps.storage)?;
    let mut sub_msgs = vec![];
    let seilor_burn_msg = seilor::msg::ExecuteMsg::Burn {
        amount: amount.clone(),
    };
    let sub_burn_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.seilor_addr.to_string(),
        msg: to_binary(&seilor_burn_msg)?,
        funds: vec![],
    }));
    sub_msgs.push(sub_burn_msg);

    let seilor_mint_msg = ve_seilor::msg::ExecuteMsg::Mint {
        recipient: sender.clone().to_string(),
        amount: amount.clone(),
    };

    let sub_mint_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.ve_seilor_addr.to_string(),
        msg: to_binary(&seilor_mint_msg)?,
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
    let sender = info.sender;
    refresh_reward(deps.branch(), sender.clone())?;

    let config: FundConfig = read_fund_config(deps.storage)?;
    let current_time = Uint64::from(env.block.time.seconds());
    if current_time.le(&config.claim_able_time) {
        return Err(StdError::generic_err("It is not yet time to claim."));
    }

    let mut sub_msgs = vec![];
    let ve_seilor_burn_msg = ve_seilor::msg::ExecuteMsg::Burn {
        user: sender.clone().to_string(),
        amount: amount.clone(),
    };
    let sub_burn_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.ve_seilor_addr.to_string(),
        msg: to_binary(&ve_seilor_burn_msg)?,
        funds: vec![],
    }));
    sub_msgs.push(sub_burn_msg);

    withdraw(deps.branch(), env.clone(), sender.clone())?;

    let mut total = Uint256::from(amount.clone());
    let time2full_redemption_user = read_time2full_redemption(deps.storage, sender.clone());
    if time2full_redemption_user.gt(&current_time) {
        let unstake_rate_user = read_unstake_rate(deps.storage, sender.clone());
        let diff_time = time2full_redemption_user.checked_sub(current_time)?;
        total = total.checked_add(
            unstake_rate_user.multiply_ratio(Uint256::from(diff_time), Uint256::from(BASE_RATE_12)),
        )?;
    }

    // let user_new_unstake_rate = total.checked_div(Uint128::from(config.exit_cycle)).unwrap();
    let user_new_unstake_rate = total.multiply_ratio(
        Uint256::from(BASE_RATE_12),
        Uint256::from(config.exit_cycle),
    );
    let user_new_time2full_redemption = current_time.checked_add(config.exit_cycle)?;

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
 * This is a function that allows a user to withdraw their claimable SEILOR tokens.
 * First, it retrieves the amount of claimable tokens using the get_claim_able_seilor function.
 * If there is an error, it returns a generic error message.
 * If there are tokens to withdraw, it reads the SEILOR fund configuration and creates a SeilorExecuteMsg to mint the tokens to the user's address.
 * This message is added as a sub-message to the response.
 * Finally, the function stores the current block time as the user's last withdrawal time and returns a response with attributes indicating the action, user, and amount withdrawn.
 */
pub fn withdraw(deps: DepsMut, env: Env, user: Addr) -> StdResult<Response> {
    let current_time = Uint64::from(env.block.time.seconds());
    let claim_able_res = get_claim_able_seilor(deps.as_ref(), env, user.clone())?;

    let amount = claim_able_res.amount;
    let mut sub_msgs = vec![];
    if amount.gt(&Uint128::zero()) {
        let config = read_fund_config(deps.storage)?;
        let seilor_mint_msg = seilor::msg::ExecuteMsg::Mint {
            recipient: user.clone().to_string(),
            amount: amount.clone(),
            contract: None,
            msg: None,
        };
        let sub_mint_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.seilor_addr.to_string(),
            msg: to_binary(&seilor_mint_msg)?,
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

pub fn re_stake(mut deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let sender = info.sender;
    _update_reward(deps.branch(), sender.clone())?;

    let mut sub_msgs = vec![];
    let config = read_fund_config(deps.storage)?;
    let claim_able_res = get_claim_able_seilor(deps.as_ref(), env.clone(), sender.clone())?;
    let reserve_seilor_res =
        get_reserved_seilor_for_vesting(deps.as_ref(), env.clone(), sender.clone())?;
    let claim_able = claim_able_res.amount;
    let reserve_seilor = reserve_seilor_res.amount;
    let total = claim_able.checked_add(reserve_seilor)?;
    if total.gt(&Uint128::zero()) {
        let ve_seilor_mint_msg = ve_seilor::msg::ExecuteMsg::Mint {
            recipient: sender.clone().to_string(),
            amount: total.clone(),
        };
        let sub_mint_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.ve_seilor_addr.to_string(),
            msg: to_binary(&ve_seilor_mint_msg)?,
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

        let mut config = read_fund_config(deps.storage)?;

        let send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: sender.to_string(),
            amount: vec![coin(reward.u128(), config.kusd_denom.to_string())],
        });

        messages.push(send_msg);

        config.kusd_reward_total_paid_amount = config
            .kusd_reward_total_paid_amount
            .checked_add(reward.clone())?;

        store_fund_config(deps.storage, &config)?;
    }
    return Ok(Response::new().add_messages(messages).add_attributes(vec![
        attr("action", "get_reward"),
        attr("sender", sender.to_string()),
        attr("reward", reward.to_string()),
    ]));
}

/**
 * @dev The amount of KUSD acquiered from the sender is euitably distributed to SEILOR stakers.
 * Calculate share by amount, and calculate the shares could claim by per unit of staked Sei.
 * Add into rewardPerTokenStored.
 */
pub fn notify_reward_amount(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender = info.sender;
    let mut config = read_fund_config(deps.storage)?;
    if sender.clone().ne(&config.kusd_reward_addr) {
        return Err(StdError::generic_err(
            "only kusd reward addr can notify reward amount",
        ));
    }

    let total_staked = total_staked(deps.as_ref())?;
    if total_staked.eq(&Uint128::zero()) {
        return Ok(Response::new());
    }
    let payment = info
        .funds
        .iter()
        .find(|x| x.denom.eq(&config.kusd_denom))
        .ok_or_else(|| StdError::generic_err("kusd denom not found"))?;

    let amount = payment.amount;

    if amount.le(&Uint128::zero()) {
        return Err(StdError::generic_err("kusd amount is zero"));
    }

    let inc_reward_per_token = amount.multiply_ratio(Uint128::new(BASE_RATE_6), total_staked);
    config.reward_per_token_stored = config
        .reward_per_token_stored
        .checked_add(inc_reward_per_token)
        .unwrap();
    config.kusd_reward_total_amount = config.kusd_reward_total_amount.checked_add(amount).unwrap();

    store_fund_config(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "notify_reward_amount"),
        attr("amount", amount.to_string()),
        attr("sender", sender.to_string()),
    ]))
}

/// ## Description
/// Receives a message of type [`Cw20ReceiveMsg`] and processes it depending on the received template.
/// If the template is not found in the received message, then an [`ContractError`] is returned,
/// otherwise it returns the [`Response`] with the specified attributes if the operation was successful.
/// ## Params
/// * **deps** is an object of type [`DepsMut`].
///
/// * **env** is an object of type [`Env`].
///
/// * **info** is an object of type [`MessageInfo`].
///
/// * **cw20_msg** is an object of type [`Cw20ReceiveMsg`]. This is the CW20 message that has to be processed.
pub fn receive_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let contract_addr = info.sender.clone();
    let msg_sender = deps.api.addr_validate(&cw20_msg.sender)?;
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Stake {}) => {
            let config = read_fund_config(deps.storage)?;
            if contract_addr.ne(&config.seilor_addr) {
                return Err(StdError::generic_err("not staking token"));
            }
            stake(deps, msg_sender, cw20_msg.amount)
        }
        Err(_) => Err(StdError::generic_err("data should be given")),
    }
}

pub fn set_gov(deps: DepsMut, info: MessageInfo, gov: Addr) -> StdResult<Response> {
    let mut config = read_fund_config(deps.storage)?;
    if config.gov != info.sender {
        return Err(StdError::generic_err("unauthorized"));
    }
    deps.api.addr_validate(gov.clone().as_str())?;

    config.new_gov = Some(gov.clone());
    store_fund_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        attr("action", "set_gov"),
        attr("gov", gov.to_string()),
    ]))
}

pub fn accept_gov(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut config = read_fund_config(deps.storage)?;
    if config.new_gov.is_none() {
        return Err(StdError::generic_err("no new gov"));
    }
    if info.sender != config.new_gov.unwrap() {
        return Err(StdError::generic_err("unauthorized"));
    }
    config.gov = info.sender.clone();
    config.new_gov = None;
    store_fund_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        attr("action", "accept_gov"),
        attr("gov", config.gov.to_string()),
        attr("new_gov", ""),
    ]))
}
