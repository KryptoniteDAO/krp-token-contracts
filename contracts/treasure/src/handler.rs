use crate::error::ContractError;
use crate::helper::BASE_RATE_6;
use crate::msg::{Cw20HookMsg, TreasureConfigMsg};
use crate::random_rules::get_winning;
use crate::state::{
    generate_next_record_id, read_treasure_config, read_treasure_state, read_treasure_user_state,
    store_treasure_config, store_treasure_user_state,
};
use cosmwasm_std::{
    attr, from_binary, to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128,
    WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    config_msg: TreasureConfigMsg,
) -> Result<Response, ContractError> {
    let mut config = read_treasure_config(deps.storage)?;
    if config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    let mut attrs = vec![];
    attrs.push(attr("action", "update_config"));
    if let Some(gov) = config_msg.gov {
        deps.api.addr_validate(gov.clone().as_str())?;
        config.gov = gov.clone();
        attrs.push(attr("gov", gov.to_string()));
    }
    if let Some(lock_token) = config_msg.lock_token {
        deps.api.addr_validate(lock_token.clone().as_str())?;
        config.lock_token = lock_token.clone();
        attrs.push(attr("lock_token", lock_token.to_string()));
    }
    if let Some(start_time) = config_msg.start_time {
        config.start_time = start_time.clone();
        attrs.push(attr("start_time", start_time.to_string()));
    }
    if let Some(end_time) = config_msg.end_time {
        config.end_time = end_time.clone();
        attrs.push(attr("end_time", end_time.to_string()));
    }
    if let Some(integral_reward_coefficient) = config_msg.integral_reward_coefficient {
        config.integral_reward_coefficient = integral_reward_coefficient.clone();
        attrs.push(attr(
            "integral_reward_coefficient",
            integral_reward_coefficient.to_string(),
        ));
    }
    if let Some(lock_duration) = config_msg.lock_duration {
        config.lock_duration = lock_duration.clone();
        attrs.push(attr("lock_duration", lock_duration.to_string()));
    }
    if let Some(punish_coefficient) = config_msg.punish_coefficient {
        config.punish_coefficient = punish_coefficient.clone();
        attrs.push(attr("punish_coefficient", punish_coefficient.to_string()));
    }
    if let Some(mint_nft_cost_integral) = config_msg.mint_nft_cost_integral {
        config.mint_nft_cost_integral = mint_nft_cost_integral.clone();
        attrs.push(attr(
            "mint_nft_cost_integral",
            mint_nft_cost_integral.to_string(),
        ));
    }
    if let Some(winning_num) = config_msg.winning_num {
        config.winning_num = winning_num.clone();
        let set_string = winning_num
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        attrs.push(attr("winning_num", set_string));
    }
    if let Some(mod_num) = config_msg.mod_num {
        config.mod_num = mod_num.clone();
        attrs.push(attr("mod_num", mod_num.to_string()));
    }
    if let Some(punish_receiver) = config_msg.punish_receiver {
        deps.api.addr_validate(punish_receiver.clone().as_str())?;
        config.punish_receiver = punish_receiver.clone();
        attrs.push(attr("punish_receiver", punish_receiver.to_string()));
    }
    store_treasure_config(deps.storage, &config)?;

    Ok(Response::default().add_attributes(attrs))
}

pub fn user_lock_hook(
    deps: DepsMut,
    env: Env,
    user_addr: Addr,
    lock_amount: Uint128,
) -> Result<Response, ContractError> {
    let config = read_treasure_config(deps.storage)?;
    let current_time = env.block.time.seconds();
    if current_time < config.start_time {
        return Err(ContractError::TreasureNotStart {});
    }
    if current_time > config.end_time {
        return Err(ContractError::TreasureEnd {});
    }
    let reward_integral_amount = lock_amount * config.integral_reward_coefficient;
    let user_end_time = current_time + config.lock_duration;

    // user data
    let mut user_state = read_treasure_user_state(deps.storage, &user_addr)?;
    if user_state.start_lock_time == 0 {
        user_state.start_lock_time = current_time;
    }
    user_state.end_lock_time = user_end_time;
    user_state.current_locked_amount += lock_amount;
    user_state.current_integral_amount += reward_integral_amount;
    user_state.total_locked_amount += lock_amount;

    // user lock record
    let record_id = generate_next_record_id(deps.storage)?;
    let record = crate::state::UserLockRecord {
        record_id: record_id.clone(),
        user_addr: user_addr.clone(),
        lock_amount,
        integral_reward_coefficient: config.integral_reward_coefficient.clone(),
        lock_duration: config.lock_duration.clone(),
        start_lock_time: current_time,
        end_lock_time: user_end_time,
    };

    // global data
    let mut global_state = read_treasure_state(deps.storage)?;
    global_state.total_locked_amount += lock_amount;
    global_state.current_locked_amount += lock_amount;
    global_state.current_integral_amount += reward_integral_amount;

    //save user lock record
    crate::state::store_user_lock_record(deps.storage, &record)?;
    //save user data
    store_treasure_user_state(deps.storage, &user_addr, &user_state)?;

    // save global data
    crate::state::store_treasure_state(deps.storage, &global_state)?;

    let mut attrs = vec![];
    attrs.push(attr("action", "user_lock_hock"));
    attrs.push(attr("user_addr", user_addr.clone()));
    attrs.push(attr("lock_amount", lock_amount.to_string()));
    attrs.push(attr(
        "reward_integral_amount",
        reward_integral_amount.to_string(),
    ));
    Ok(Response::default().add_attributes(attrs))
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
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let contract_addr = info.sender.clone();
    let msg_sender = deps.api.addr_validate(&cw20_msg.sender)?;
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::UserLockHook {}) => {
            let config = read_treasure_config(deps.storage)?;
            if contract_addr.ne(&config.lock_token) {
                return Err(ContractError::InvalidLockToken {});
            }
            user_lock_hook(deps, env, msg_sender, cw20_msg.amount)
        }
        Err(_) => Err(ContractError::InvalidCw20HookMsg {}),
    }
}

pub fn user_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = read_treasure_config(deps.storage)?;
    let current_time = env.block.time.seconds();
    let mut user_state = read_treasure_user_state(deps.storage, &info.sender)?;

    // check user locked amount
    if amount > user_state.current_locked_amount {
        return Err(ContractError::InsufficientLockFunds {});
    }
    let mut global_state = read_treasure_state(deps.storage)?;

    let mut withdraw_amount = amount.clone();
    let mut punish_amount = Uint128::zero();
    let mut transfer_msgs = vec![];
    // check user lock time , if user lock time is not end , punish user
    if current_time < user_state.end_lock_time {
        punish_amount = amount * config.punish_coefficient / Uint128::from(BASE_RATE_6);
        withdraw_amount -= punish_amount;
        // user data
        user_state.total_punish_amount += punish_amount;
        // global data
        global_state.total_punish_amount += punish_amount;

        // transfer to punish token
        transfer_msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.lock_token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: config.punish_receiver.to_string(),
                amount: punish_amount,
            })?,
            funds: vec![],
        }));
    }

    // user withdraw record

    let record_id = generate_next_record_id(deps.storage)?;
    let record = crate::state::UserWithdrawRecord {
        record_id: record_id.clone(),
        user_addr: info.sender.clone(),
        withdraw_amount: withdraw_amount.clone(),
        punish_amount: punish_amount.clone(),
        withdraw_time: current_time,
    };

    // user data
    user_state.current_locked_amount -= amount;

    // global data
    global_state.current_locked_amount -= amount;

    // save user withdraw record
    crate::state::store_user_withdraw_record(deps.storage, &record)?;
    // save user data
    store_treasure_user_state(deps.storage, &info.sender, &user_state)?;
    // save global data
    crate::state::store_treasure_state(deps.storage, &global_state)?;

    // transfer lock token to user
    transfer_msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.lock_token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount: withdraw_amount,
        })?,
        funds: vec![],
    }));

    let mut attrs = vec![];
    attrs.push(attr("action", "user_withdraw"));
    attrs.push(attr("user_addr", info.sender));
    attrs.push(attr("withdraw_amount", withdraw_amount.to_string()));
    attrs.push(attr("punish_amount", punish_amount.to_string()));
    Ok(Response::default()
        .add_attributes(attrs)
        .add_messages(transfer_msgs))
}

pub fn pre_mint_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mint_num: u64,
) -> Result<Response, ContractError> {
    if mint_num < 1u64 {
        return Err(ContractError::InvalidMintNum {});
    }

    let config = read_treasure_config(deps.storage)?;
    let current_time = env.block.time.seconds();
    let mut user_state = read_treasure_user_state(deps.storage, &info.sender)?;

    // check user integral amount
    let mint_integral_amount = config.mint_nft_cost_integral * Uint128::from(mint_num);

    if mint_integral_amount > user_state.current_integral_amount {
        return Err(ContractError::InsufficientIntegralFunds {});
    }
    let mut win_nft_num = 0u64;
    let mut lost_nft_num = 0u64;
    let record_id = generate_next_record_id(deps.storage)?;
    let winning_num = &config.winning_num;
    let mod_num = &config.mod_num;
    for i in 0..mint_num {
        let unique_factor = record_id + i;
        let winning = get_winning(
            env.clone(),
            unique_factor.to_string(),
            vec![],
            winning_num,
            mod_num,
        )?;
        if winning {
            win_nft_num += 1;
        } else {
            lost_nft_num += 1;
        }
    }

    let mut global_state = read_treasure_state(deps.storage)?;

    // user withdraw record
    let record_id = generate_next_record_id(deps.storage)?;
    let record = crate::state::UserMintNftRecord {
        record_id: record_id.clone(),
        user_addr: info.sender.clone(),
        mint_nft_num: mint_num.clone(),
        mint_time: current_time,
        mint_nft_cost_integral_amount: mint_integral_amount.clone(),
        win_nft_num: win_nft_num.clone(),
    };

    // user data
    user_state.current_integral_amount -= mint_integral_amount;
    user_state.total_cost_integral_amount += mint_integral_amount;

    user_state.win_nft_num += win_nft_num;
    user_state.lose_nft_num += lost_nft_num;

    // global data
    global_state.current_integral_amount -= mint_integral_amount;

    global_state.total_win_nft_num += win_nft_num;
    global_state.total_lose_nft_num += lost_nft_num;

    // save user withdraw record
    crate::state::store_user_mint_nft_record(deps.storage, &record)?;
    // save user data
    store_treasure_user_state(deps.storage, &info.sender, &user_state)?;
    // save global data
    crate::state::store_treasure_state(deps.storage, &global_state)?;

    let mut attrs = vec![];
    attrs.push(attr("action", "pre_mint_nft"));
    attrs.push(attr("user_addr", info.sender));
    attrs.push(attr(
        "mint_integral_amount",
        mint_integral_amount.to_string(),
    ));
    attrs.push(attr("win_nft_num", win_nft_num.to_string()));

    Ok(Response::default().add_attributes(attrs))
}
