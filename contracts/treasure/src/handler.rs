use crate::error::ContractError;
use crate::helper::BASE_RATE_6;
use crate::msg::{Cw20HookMsg, TreasureConfigMsg};
use crate::querier::compute_user_dust;
use crate::random_rules::get_winning;
use crate::state::{
    generate_next_global_id, read_treasure_config, read_treasure_state, read_treasure_user_state,
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

    if let Some(lock_token) = config_msg.lock_token {
        deps.api.addr_validate(lock_token.clone().as_str())?;
        config.lock_token = lock_token.clone();
        attrs.push(attr("lock_token", lock_token.to_string()));
    }
    if let Some(start_lock_time) = config_msg.start_lock_time {
        config.start_lock_time = start_lock_time.clone();
        attrs.push(attr("start_lock_time", start_lock_time.to_string()));
    }
    if let Some(end_lock_time) = config_msg.end_lock_time {
        config.end_lock_time = end_lock_time.clone();
        attrs.push(attr("end_lock_time", end_lock_time.to_string()));
    }
    if let Some(dust_reward_per_second) = config_msg.dust_reward_per_second {
        config.dust_reward_per_second = dust_reward_per_second.clone();
        attrs.push(attr(
            "dust_reward_per_second",
            dust_reward_per_second.to_string(),
        ));
    }
    if let Some(withdraw_delay_duration) = config_msg.withdraw_delay_duration {
        config.withdraw_delay_duration = withdraw_delay_duration.clone();
        attrs.push(attr(
            "withdraw_delay_duration",
            withdraw_delay_duration.to_string(),
        ));
    }
    if let Some(no_delay_punish_coefficient) = config_msg.no_delay_punish_coefficient {
        config.no_delay_punish_coefficient = no_delay_punish_coefficient.clone();
        attrs.push(attr(
            "no_delay_punish_coefficient",
            no_delay_punish_coefficient.to_string(),
        ));
    }
    if let Some(punish_receiver) = config_msg.punish_receiver {
        deps.api.addr_validate(punish_receiver.clone().as_str())?;
        config.punish_receiver = punish_receiver.clone();
        attrs.push(attr("punish_receiver", punish_receiver.to_string()));
    }

    if let Some(nft_start_pre_mint_time) = config_msg.nft_start_pre_mint_time {
        config.nft_start_pre_mint_time = nft_start_pre_mint_time.clone();
        attrs.push(attr(
            "nft_start_pre_mint_time",
            nft_start_pre_mint_time.to_string(),
        ));
    }
    if let Some(nft_end_pre_mint_time) = config_msg.nft_end_pre_mint_time {
        config.nft_end_pre_mint_time = nft_end_pre_mint_time.clone();
        attrs.push(attr(
            "nft_end_pre_mint_time",
            nft_end_pre_mint_time.to_string(),
        ));
    }
    if let Some(mint_nft_cost_dust) = config_msg.mint_nft_cost_dust {
        config.mint_nft_cost_dust = mint_nft_cost_dust.clone();
        attrs.push(attr("mint_nft_cost_dust", mint_nft_cost_dust.to_string()));
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
    if current_time < config.start_lock_time {
        return Err(ContractError::TreasureNotStart {});
    }
    if current_time > config.end_lock_time {
        return Err(ContractError::TreasureEnd {});
    }

    // user data
    let mut user_state = read_treasure_user_state(deps.storage, &user_addr)?;

    //compute reward dust
    let reward_dust_amount = compute_user_dust(
        current_time.clone(),
        user_state.last_lock_time.clone(),
        config.end_lock_time.clone(),
        user_state.current_locked_amount.clone(),
        config.dust_reward_per_second.clone(),
    )?;

    user_state.last_lock_time = current_time;
    user_state.current_locked_amount += lock_amount;
    user_state.current_dust_amount += reward_dust_amount;
    user_state.total_locked_amount += lock_amount;

    // global data
    let mut global_state = read_treasure_state(deps.storage)?;
    global_state.total_locked_amount += lock_amount;
    global_state.current_locked_amount += lock_amount;

    //save user data
    store_treasure_user_state(deps.storage, &user_addr, &user_state)?;

    // save global data
    crate::state::store_treasure_state(deps.storage, &global_state)?;

    let mut attrs = vec![];
    attrs.push(attr("action", "user_lock_hock"));
    attrs.push(attr("user_addr", user_addr.to_string()));
    attrs.push(attr("lock_amount", lock_amount.to_string()));
    attrs.push(attr("reward_dust_amount", reward_dust_amount.to_string()));
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

pub fn user_unlock(
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

    //compute reward dust
    let reward_dust_amount = compute_user_dust(
        current_time.clone(),
        user_state.last_lock_time.clone(),
        config.end_lock_time.clone(),
        user_state.current_locked_amount.clone(),
        config.dust_reward_per_second.clone(),
    )?;

    // update user state
    user_state.last_lock_time = current_time;
    user_state.last_unlock_time = current_time;
    user_state.current_locked_amount -= amount;
    user_state.current_unlock_amount += amount;
    user_state.current_dust_amount += reward_dust_amount;
    user_state.total_unlock_amount += amount;

    // global data
    let mut global_state = read_treasure_state(deps.storage)?;
    global_state.current_locked_amount -= amount;
    global_state.current_unlock_amount += amount;

    //save user data
    store_treasure_user_state(deps.storage, &info.sender, &user_state)?;

    // save global data
    crate::state::store_treasure_state(deps.storage, &global_state)?;

    let attrs = vec![
        attr("action", "user_unlock"),
        attr("user_addr", info.sender.to_string()),
        attr("unlock_amount", amount.to_string()),
        attr("reward_dust_amount", reward_dust_amount.to_string()),
    ];
    Ok(Response::default().add_attributes(attrs))
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

    // check user unlock amount
    if amount > user_state.current_unlock_amount {
        return Err(ContractError::InsufficientUnlockFunds {});
    }
    let mut global_state = read_treasure_state(deps.storage)?;

    let mut withdraw_amount = amount.clone();
    let mut punish_amount = Uint128::zero();
    let mut transfer_msgs = vec![];
    // check user lock time , if user lock time is not end , punish user
    if current_time < (user_state.last_unlock_time + config.withdraw_delay_duration) {
        punish_amount = amount * config.no_delay_punish_coefficient / Uint128::from(BASE_RATE_6);
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

    // user data
    user_state.current_unlock_amount -= amount;
    user_state.total_withdraw_amount += amount;

    // global data
    global_state.current_unlock_amount -= amount;

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

    // check mint time
    if current_time < config.nft_start_pre_mint_time {
        return Err(ContractError::PreMintTimeNotReach {});
    }
    if current_time > config.nft_end_pre_mint_time {
        return Err(ContractError::PreMintTimeEnd {});
    }
    if current_time < config.end_lock_time {
        return Err(ContractError::LockTimeNotEnd {});
    }

    let mut user_state = read_treasure_user_state(deps.storage, &info.sender)?;

    //compute reward dust
    let reward_dust_amount = compute_user_dust(
        current_time.clone(),
        user_state.last_lock_time.clone(),
        config.end_lock_time.clone(),
        user_state.current_locked_amount.clone(),
        config.dust_reward_per_second.clone(),
    )?;
    user_state.current_dust_amount += reward_dust_amount;

    // check user dust amount
    let mint_dust_amount = config.mint_nft_cost_dust * Uint128::from(mint_num);

    if mint_dust_amount > user_state.current_dust_amount {
        return Err(ContractError::InsufficientIntegralFunds {});
    }

    let mut win_nft_num = 0u64;
    let mut lost_nft_num = 0u64;
    let record_id = generate_next_global_id(deps.storage)?;
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

    // user data
    user_state.current_dust_amount -= mint_dust_amount;
    user_state.total_cost_dust_amount += mint_dust_amount;
    user_state.last_lock_time = current_time;

    user_state.win_nft_num += win_nft_num;
    user_state.lose_nft_num += lost_nft_num;

    // global data
    global_state.total_cost_dust_amount += mint_dust_amount;
    global_state.total_win_nft_num += win_nft_num;
    global_state.total_lose_nft_num += lost_nft_num;

    // save user data
    store_treasure_user_state(deps.storage, &info.sender, &user_state)?;
    // save global data
    crate::state::store_treasure_state(deps.storage, &global_state)?;

    let mut attrs = vec![];
    attrs.push(attr("action", "pre_mint_nft"));
    attrs.push(attr("user_addr", info.sender));
    attrs.push(attr("mint_dust_amount", mint_dust_amount.to_string()));
    attrs.push(attr("win_nft_num", win_nft_num.to_string()));

    Ok(Response::default().add_attributes(attrs))
}

pub fn set_gov(deps: DepsMut, info: MessageInfo, gov: Addr) -> Result<Response, ContractError> {
    let mut config = read_treasure_config(deps.storage)?;
    if config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    deps.api.addr_validate(gov.clone().as_str())?;

    config.new_gov = Some(gov.clone());
    store_treasure_config(deps.storage, &config)?;
    Ok(Response::default().add_attributes(vec![
        attr("action", "set_gov"),
        attr("gov", gov.to_string()),
    ]))
}

pub fn accept_gov(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut config = read_treasure_config(deps.storage)?;
    if config.new_gov.is_none() {
        return Err(ContractError::NoNewGov {});
    }
    if info.sender != config.new_gov.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    config.gov = info.sender.clone();
    config.new_gov = None;
    store_treasure_config(deps.storage, &config)?;
    Ok(Response::default().add_attributes(vec![
        attr("action", "accept_gov"),
        attr("gov", config.gov.to_string()),
        attr("new_gov", ""),
    ]))
}
