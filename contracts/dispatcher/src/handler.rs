use crate::error::ContractError;
use crate::helper::is_empty_str;
use crate::msg::{AddUserMsg, UpdateGlobalConfigMsg};
use crate::state::{
    read_global_config, read_global_state, read_user_state, store_global_config,
    store_global_state, store_user_by_page, store_user_state, UserState,
};
use cosmwasm_std::{
    attr, to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, Uint256,
    Uint64, WasmMsg,
};
use cw20::Cw20ExecuteMsg;

pub fn update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: UpdateGlobalConfigMsg,
) -> Result<Response, ContractError> {
    let mut config = read_global_config(deps.storage)?;
    if config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![];
    attrs.push(attr("action", "update_config"));

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

    if let Some(claim_token) = msg.claim_token {
        config.claim_token = claim_token.clone();
        attrs.push(attr("claim_token", claim_token));
    }

    if let Some(start_lock_period_time) = msg.start_lock_period_time {
        // check current block time > start_lock_period_time
        if config.start_lock_period_time < env.block.time.seconds() {
            return Err(ContractError::InvalidStartLockPeriodTime {});
        }
        config.start_lock_period_time = start_lock_period_time.clone();
        attrs.push(attr(
            "start_lock_period_time",
            start_lock_period_time.to_string(),
        ));
    }

    store_global_config(deps.storage, &config)?;

    Ok(Response::default().add_attributes(attrs))
}

pub fn set_gov(deps: DepsMut, info: MessageInfo, gov: Addr) -> Result<Response, ContractError> {
    let mut config = read_global_config(deps.storage)?;
    if config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    deps.api.addr_validate(gov.clone().as_str())?;

    config.new_gov = Some(gov.clone());
    store_global_config(deps.storage, &config)?;
    Ok(Response::default().add_attributes(vec![
        attr("action", "set_gov"),
        attr("gov", gov.to_string()),
    ]))
}

pub fn accept_gov(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut config = read_global_config(deps.storage)?;
    if config.new_gov.is_none() {
        return Err(ContractError::NoNewGov {});
    }
    if info.sender != config.new_gov.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    config.gov = info.sender.clone();
    config.new_gov = None;
    store_global_config(deps.storage, &config)?;
    Ok(Response::default().add_attributes(vec![
        attr("action", "accept_gov"),
        attr("gov", config.gov.to_string()),
        attr("new_gov", ""),
    ]))
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
    if user_msg.lock_amount == Uint256::zero() {
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

        if user_state.claimed_lock_amount != Uint256::zero() {
            return Err(ContractError::UserAlreadyClaimed(user_msg.user.clone()));
        }

        global_state.total_user_lock_amount -= user_state.total_user_lock_amount;
    }

    // check if the user's lock amount is greater than the global lock amount
    global_state.total_user_lock_amount += user_msg.lock_amount;
    if global_state.total_user_lock_amount > global_config.total_lock_amount {
        return Err(ContractError::UserLockAmountTooLarge(user_msg.user.clone()));
    }

    let user_per_lock_amount = user_msg.lock_amount / Uint256::from(global_config.periods);

    let user_state = UserState {
        user: user_addr.clone(),
        total_user_lock_amount: user_msg.lock_amount,
        claimed_lock_amount: Uint256::zero(),
        last_claimed_period: 0u64,
        user_per_lock_amount,
    };
    store_user_state(deps.storage, &user_addr, &user_state)?;

    store_global_state(deps.storage, &global_state)?;

    Ok(())
}

pub fn user_claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = read_global_config(deps.storage)?;
    let current_time = env.block.time.seconds();
    if config.start_lock_period_time > current_time {
        return Err(ContractError::ClaimTimeIsNotArrived {});
    }
    let sender = info.sender.clone();

    let mut user_state = read_user_state(deps.storage, &sender)?;

    // check if the user already exists
    if is_empty_str(&user_state.user.to_string()) {
        return Err(ContractError::UserNotExists(sender.clone()));
    }
    let mut global_state = read_global_state(deps.storage)?;

    let mut claimable_amount = Uint256::zero();

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

    // validate that global_state.total_user_claimed_lock_amount <= global_state.total_user_lock_amount
    if global_state.total_user_claimed_lock_amount > global_state.total_user_lock_amount {
        return Err(ContractError::GlobalClaimLockAmountTooLarge {});
    }
    // check claimable amount is not zero
    if claimable_amount == Uint256::zero() {
        return Err(ContractError::UserClaimAmountIsZero(sender.clone()));
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
