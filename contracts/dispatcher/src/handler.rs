use crate::error::ContractError;
use crate::helper::is_empty_str;
use crate::msg::{AddUserMsg, UpdateGlobalConfigMsg};
use crate::state::{
    read_global_config, read_global_state, read_regret_info, read_user_state, store_global_config,
    store_global_state, store_regret_info, store_user_by_page, store_user_state, UserState,
};
use cosmwasm_std::{
    attr, to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, Uint256,
    Uint64, WasmMsg,
};
use cw20::Cw20ExecuteMsg;

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    msg: UpdateGlobalConfigMsg,
) -> Result<Response, ContractError> {
    let mut config = read_global_config(deps.storage)?;
    if config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![];
    attrs.push(attr("action", "update_config"));

    if let Some(gov) = msg.gov {
        config.gov = gov.clone();
        attrs.push(attr("gov", gov));
    }

    if let Some(total_lock_amount) = msg.total_lock_amount {
        let global_state = read_global_state(deps.storage)?;
        // Check if the new total_amount is less than the stored total_amount
        if total_lock_amount < global_state.total_user_lock_amount {
            // Return an error indicating an invalid total amount
            return Err(ContractError::InvalidTotalLockAmount {});
        }
        config.total_lock_amount = total_lock_amount.clone();
        attrs.push(attr("total_lock_amount", total_lock_amount.to_string()));
    }

    if let Some(total_unlock_amount) = msg.total_unlock_amount {
        let global_state = read_global_state(deps.storage)?;
        // Check if the new total_amount is less than the stored total_amount
        if total_unlock_amount < global_state.total_user_unlock_amount {
            // Return an error indicating an invalid total amount
            return Err(ContractError::InvalidTotalUnlockAmount {});
        }
        config.total_unlock_amount = total_unlock_amount.clone();
        attrs.push(attr("total_unlock_amount", total_unlock_amount.to_string()));
    }

    if let Some(claim_token) = msg.claim_token {
        config.claim_token = claim_token.clone();
        attrs.push(attr("claim_token", claim_token));
    }

    if let Some(start_time) = msg.start_time {
        config.start_time = start_time.clone();
        attrs.push(attr("start_time", start_time.to_string()));
    }

    if let Some(end_regret_time) = msg.end_regret_time {
        config.end_regret_time = end_regret_time.clone();
        attrs.push(attr("end_regret_time", end_regret_time.to_string()));
    }

    if let Some(regret_token_receiver) = msg.regret_token_receiver {
        config.regret_token_receiver = regret_token_receiver.clone();
        attrs.push(attr("regret_token_receiver", regret_token_receiver));
    }

    store_global_config(deps.storage, &config)?;

    Ok(Response::default().add_attributes(attrs))
}

pub fn add_users(
    mut deps: DepsMut,
    info: MessageInfo,
    msg: Vec<AddUserMsg>,
) -> Result<Response, ContractError> {
    let config = read_global_config(deps.storage)?;
    if config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![];
    attrs.push(attr("action", "add_user"));

    for user in msg {
        _add_single_user(deps.branch(), &user)?;
        attrs.push(attr("user", user.user.to_string()));
        attrs.push(attr("unlock_amount", user.unlock_amount.to_string()));
        attrs.push(attr("lock_amount", user.lock_amount.to_string()));
        attrs.push(attr("replace", user.replace.to_string()));
    }

    Ok(Response::default().add_attributes(attrs))
}

fn _add_single_user(deps: DepsMut, user_msg: &AddUserMsg) -> Result<(), ContractError> {
    let global_config = read_global_config(deps.storage)?;
    let mut global_state = read_global_state(deps.storage)?;
    let user_addr = user_msg.user.clone();
    // check if the user's amount is zero
    if user_msg.lock_amount == Uint256::zero() && user_msg.unlock_amount == Uint256::zero() {
        return Err(ContractError::UserAmountIsZero(user_msg.user.clone()));
    }

    let user_state = read_user_state(deps.storage, &user_msg.user)?;

    if !user_msg.replace {
        // check if the user already exists
        if !is_empty_str(&user_state.user.to_string()) {
            return Err(ContractError::UserAlreadyExists(user_msg.user.clone()));
        }
        store_user_by_page(deps.storage, &user_msg.user)?;
    } else {
        if is_empty_str(&user_state.user.to_string()) {
            return Err(ContractError::UserNotExists(user_msg.user.clone()));
        }

        if user_state.is_regret {
            return Err(ContractError::UserAlreadyRegret(user_msg.user.clone()));
        }

        if user_state.claimed_lock_amount != Uint256::zero()
            || user_state.claimed_unlock_amount != Uint256::zero()
        {
            return Err(ContractError::UserAlreadyClaimed(user_msg.user.clone()));
        }

        global_state.total_user_lock_amount -= user_state.total_user_lock_amount;
        global_state.total_user_unlock_amount -= user_state.total_user_unlock_amount;
    }

    global_state.total_user_unlock_amount += user_msg.unlock_amount;
    // check if the user's unlock amount is greater than the global unlock amount
    if global_state.total_user_unlock_amount > global_config.total_unlock_amount {
        return Err(ContractError::UserUnlockAmountTooLarge(
            user_msg.user.clone(),
        ));
    }

    // check if the user's lock amount is greater than the global lock amount
    global_state.total_user_lock_amount += user_msg.lock_amount;
    if global_state.total_user_lock_amount > global_config.total_lock_amount {
        return Err(ContractError::UserLockAmountTooLarge(user_msg.user.clone()));
    }

    let user_per_lock_amount = user_msg.lock_amount / Uint256::from(global_config.periods);

    let user_state = UserState {
        user: user_addr.clone(),
        total_user_unlock_amount: user_msg.unlock_amount,
        total_user_lock_amount: user_msg.lock_amount,
        claimed_unlock_amount: Uint256::zero(),
        claimed_lock_amount: Uint256::zero(),
        last_claimed_period: 0u64,
        user_per_lock_amount,
        is_regret: false,
        regret_time: 0u64,
    };
    store_user_state(deps.storage, &user_addr, &user_state)?;

    store_global_state(deps.storage, &global_state)?;

    Ok(())
}

pub fn user_regret(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = read_global_config(deps.storage)?;
    let current_time = env.block.time.seconds();

    if config.start_time > current_time {
        return Err(ContractError::RegretTimeNotStart {});
    }

    let sender = info.sender.clone();

    let mut user_state = read_user_state(deps.storage, &sender)?;
    // check if the user already exists
    if is_empty_str(&user_state.user.to_string()) {
        return Err(ContractError::UserNotExists(sender.clone()));
    }
    // check if the user already regret
    if user_state.is_regret {
        return Err(ContractError::UserAlreadyRegret(sender.clone()));
    }
    // check if the user already claimed
    if user_state.claimed_lock_amount != Uint256::zero()
        || user_state.claimed_unlock_amount != Uint256::zero()
    {
        return Err(ContractError::UserAlreadyClaimed(sender.clone()));
    }

    // check if the regret time is over
    if current_time >= config.start_lock_period_time || current_time >= config.end_regret_time {
        return Err(ContractError::RegretTimeIsOver {});
    }

    user_state.is_regret = true;
    user_state.regret_time = current_time;

    let mut regret_info = read_regret_info(deps.storage)?;
    regret_info.total_unlock_amount += user_state.total_user_unlock_amount;
    regret_info.total_lock_amount += user_state.total_user_lock_amount;
    regret_info.per_lock_amount += user_state.user_per_lock_amount;

    store_user_state(deps.storage, &sender, &user_state)?;
    store_regret_info(deps.storage, &regret_info)?;

    let mut attrs = vec![];
    attrs.push(attr("action", "regret"));
    attrs.push(attr("user", sender.to_string()));
    Ok(Response::default().add_attributes(attrs))
}

pub fn user_claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = read_global_config(deps.storage)?;
    let current_time = env.block.time.seconds();
    if config.start_time > current_time {
        return Err(ContractError::ClaimTimeIsNotArrived {});
    }
    let sender = info.sender.clone();

    let mut user_state = read_user_state(deps.storage, &sender)?;

    // check user regret
    if user_state.is_regret {
        return Err(ContractError::UserAlreadyRegret(sender.clone()));
    }

    // check if the user already exists
    if is_empty_str(&user_state.user.to_string()) {
        return Err(ContractError::UserNotExists(sender.clone()));
    }
    let mut global_state = read_global_state(deps.storage)?;
    // check user claim unlock amount
    let mut claimable_amount = Uint256::zero();

    if user_state.claimed_unlock_amount == Uint256::zero() {
        claimable_amount = user_state.total_user_unlock_amount;
        user_state.claimed_unlock_amount = user_state.total_user_unlock_amount;

        global_state.total_user_claimed_unlock_amount += user_state.total_user_unlock_amount;
    }

    // cal user claim lock amount
    if current_time > config.start_lock_period_time
        && user_state.last_claimed_period < Uint64::from(config.periods).u64()
    {
        let mut current_claim_period =
            (current_time - config.start_lock_period_time) / config.duration_per_period;
        // max claim period is periods
        if current_claim_period > config.periods {
            current_claim_period = config.periods;
        }

        let can_claim_period = current_claim_period - user_state.last_claimed_period;
        if can_claim_period > 0 {
            let can_claim_amount =
                user_state.user_per_lock_amount * Uint256::from(can_claim_period);
            claimable_amount += can_claim_amount;
            user_state.claimed_lock_amount += can_claim_amount;
            user_state.last_claimed_period = current_claim_period;

            global_state.total_user_claimed_lock_amount += can_claim_amount;
        }
    }
    if user_state.claimed_unlock_amount > user_state.total_user_unlock_amount {
        return Err(ContractError::UserClaimUnlockAmountTooLarge(sender.clone()));
    }

    if user_state.claimed_lock_amount > user_state.total_user_lock_amount {
        return Err(ContractError::UserClaimLockAmountTooLarge(sender.clone()));
    }

    store_user_state(deps.storage, &sender, &user_state)?;
    store_global_state(deps.storage, &global_state)?;

    // transfer token to user

    let cosmos_msg = _transfer_token(&config.claim_token, &sender, &claimable_amount)?;

    Ok(Response::default()
        .add_attributes(vec![
            attr("action", "user_claim"),
            attr("user", sender.to_string()),
            attr("claimable_amount", claimable_amount.to_string()),
        ])
        .add_message(cosmos_msg))
}

pub fn regret_claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let global_config = read_global_config(deps.storage)?;
    let mut global_state = read_global_state(deps.storage)?;
    let mut regret_info = read_regret_info(deps.storage)?;
    let regret_token_receiver = global_config.regret_token_receiver;

    if is_empty_str(regret_token_receiver.as_str()) {
        return Err(ContractError::RegretTokenReceiverNotSet {});
    }
    let current_time = env.block.time.seconds();
    if current_time < global_config.start_time {
        return Err(ContractError::ClaimTimeIsNotArrived {});
    }

    let mut claimable_amount = Uint256::zero();
    if regret_info.total_unlock_amount > regret_info.total_claimed_unlock_amount {
        claimable_amount =
            regret_info.total_unlock_amount - regret_info.total_claimed_unlock_amount;
        regret_info.total_claimed_unlock_amount = regret_info.total_unlock_amount;

        global_state.total_user_claimed_unlock_amount += claimable_amount;
    }

    // cal user claim lock amount
    if current_time > global_config.start_lock_period_time
        && regret_info.last_claimed_period < Uint64::from(global_config.periods).u64()
    {
        let mut current_claim_period = (current_time - global_config.start_lock_period_time)
            / global_config.duration_per_period;
        // max claim period is periods
        if current_claim_period > global_config.periods {
            current_claim_period = global_config.periods;
        }

        let can_claim_period = current_claim_period - regret_info.last_claimed_period;
        if can_claim_period > 0 {
            let can_claim_amount = regret_info.per_lock_amount * Uint256::from(can_claim_period);
            claimable_amount += can_claim_amount;
            regret_info.total_claimed_lock_amount += can_claim_amount;
            regret_info.last_claimed_period = current_claim_period;

            global_state.total_user_claimed_lock_amount += can_claim_amount;
        }
    }

    if regret_info.total_claimed_unlock_amount > regret_info.total_unlock_amount {
        return Err(ContractError::UserClaimUnlockAmountTooLarge(
            regret_token_receiver.clone(),
        ));
    }

    if regret_info.total_claimed_lock_amount > regret_info.total_claimed_lock_amount {
        return Err(ContractError::UserClaimLockAmountTooLarge(
            regret_token_receiver.clone(),
        ));
    }

    store_regret_info(deps.storage, &regret_info)?;
    store_global_state(deps.storage, &global_state)?;

    let cosmos_msg = _transfer_token(
        &global_config.claim_token,
        &regret_token_receiver,
        &claimable_amount,
    )?;
    Ok(Response::default()
        .add_attributes(vec![
            attr("action", "regret_claim"),
            attr("sender", info.sender.to_string()),
            attr("regret_token_receiver", regret_token_receiver.to_string()),
            attr("claimable_amount", claimable_amount.to_string()),
        ])
        .add_message(cosmos_msg))
}

fn _transfer_token(
    contract_addr: &Addr,
    sender: &Addr,
    claimable_amount: &Uint256,
) -> Result<CosmosMsg, ContractError> {
    let transfer_msg = Cw20ExecuteMsg::Transfer {
        recipient: sender.clone().to_string(),
        amount: Uint128::try_from(claimable_amount.clone())?,
    };

    let cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: contract_addr.clone().to_string(),
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    });
    Ok(cosmos_msg)
}
