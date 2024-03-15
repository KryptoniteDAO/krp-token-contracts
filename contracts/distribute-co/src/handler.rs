use crate::msg::UserPeriodConfigMsg;
use crate::state::{
    has_period_config, has_user_period_config, read_config, read_period_config,
    read_user_period_config, read_user_status, store_config, store_period_config,
    store_user_period_config, store_user_status, PeriodConfig, UserPeriodClaimedDetails,
    UserPeriodConfig,
};
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
    WasmMsg,
};

pub fn add_period_configs(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    period_configs: Vec<PeriodConfig>,
) -> StdResult<Response> {
    let own_res = cw_ownable::assert_owner(deps.storage, &info.sender);
    if own_res.is_err() {
        return Err(StdError::generic_err(own_res.err().unwrap().to_string()));
    }
    let mut config = read_config(deps.storage)?;
    let mut period_total_amount = Uint128::zero();
    for period_config in period_configs {
        let period_id = period_config.period_id;
        let has_period = has_period_config(deps.storage, &period_id);
        if has_period {
            return Err(StdError::generic_err(format!(
                "Period {} already exists",
                period_id
            )));
        }
        period_total_amount += period_config.period_total_amount;
        store_period_config(deps.storage, &period_config)?;
    }
    config.total_distribute_amount += period_total_amount;
    store_config(deps.storage, &config)?;

    Ok(Response::default().add_attributes(vec![
        ("action", "add_period_configs"),
        ("owner", info.sender.as_str()),
    ]))
}

pub fn add_user_period_configs(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_period_configs_msg: Vec<UserPeriodConfigMsg>,
) -> StdResult<Response> {
    let own_res = cw_ownable::assert_owner(deps.storage, &info.sender);
    if own_res.is_err() {
        return Err(StdError::generic_err(own_res.err().unwrap().to_string()));
    }
    let mut config = read_config(deps.storage)?;
    for user_period_config_msg in user_period_configs_msg {
        let user_address = user_period_config_msg.user_address.clone();
        let has_user = has_user_period_config(deps.storage, &user_address);
        if has_user {
            return Err(StdError::generic_err(format!(
                "User {} already exists",
                user_address
            )));
        }
        let user_period_config = UserPeriodConfig {
            user_per_period_amount: user_period_config_msg.user_per_period_amount,
            user_total_claimed_amount: Uint128::zero(),
            user_total_amount: user_period_config_msg.user_total_amount,
            user_claimed_periods: user_period_config_msg.user_claimed_periods,
        };
        config.user_register_amount += user_period_config_msg.user_total_amount;
        if config.user_register_amount > config.total_distribute_amount {
            return Err(StdError::generic_err(format!(
                "User {} total amount exceeds total distribute amount",
                user_address
            )));
        }
        store_user_period_config(deps.storage, &user_address, &user_period_config)?;
        store_user_status(deps.storage, &user_address, true)?;
    }
    store_config(deps.storage, &config)?;
    Ok(Response::default().add_attributes(vec![
        ("action", "add_user_period_configs"),
        ("owner", info.sender.as_str()),
    ]))
}

pub fn update_user_status(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_address: Addr,
    status: bool,
) -> StdResult<Response> {
    let own_res = cw_ownable::assert_owner(deps.storage, &info.sender);
    if own_res.is_err() {
        return Err(StdError::generic_err(own_res.err().unwrap().to_string()));
    }
    store_user_status(deps.storage, &user_address, status)?;
    Ok(Response::default().add_attributes(vec![
        ("action", "update_user_status"),
        ("owner", info.sender.as_str()),
    ]))
}

pub fn user_claim_periods(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    period_ids: Vec<u64>,
) -> StdResult<Response> {
    let config = read_config(deps.storage)?;
    let user_address = info.sender.clone();
    let user_status = read_user_status(deps.storage, &user_address)?;
    if !user_status {
        return Err(StdError::generic_err(format!(
            "User {} is not allowed to claim",
            user_address
        )));
    };

    let mut user_period_config = read_user_period_config(deps.storage, &user_address)?;
    let user_per_period_amount = user_period_config.user_per_period_amount.clone();

    let current_time = env.block.time.seconds();
    let mut user_claimed_periods = user_period_config.user_claimed_periods.clone();
    let mut user_total_claimed_amount = Uint128::zero();
    let mut msgs: Vec<CosmosMsg> = vec![];
    for period_id in period_ids {
        let has_claimed = user_claimed_periods.contains_key(&period_id);
        if has_claimed {
            return Err(StdError::generic_err(format!(
                "User {} already claimed period {}",
                user_address, period_id
            )));
        }

        let mut period_config = read_period_config(deps.storage, &period_id)?;
        if current_time < period_config.period_claimed_time {
            return Err(StdError::generic_err(format!(
                "Period {} is not claimable yet",
                period_id
            )));
        }
        period_config.period_claimed_amount += user_per_period_amount.clone();
        if period_config.period_claimed_amount > period_config.period_total_amount {
            return Err(StdError::generic_err(format!(
                "Period {} claimed amount exceeds total amount",
                period_id
            )));
        }

        if !period_config.claimed_from_distribute {
            let claim_from_distribute_msg = distribute::msg::ExecuteMsg::Claim {
                rule_type: "co".to_string(),
                msg: None,
            };
            let claim_distribute_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: config.token_distribute_address.clone().to_string(),
                msg: to_binary(&claim_from_distribute_msg)?,
                funds: vec![],
            });

            msgs.push(claim_distribute_msg);
            period_config.claimed_from_distribute = true;
        }

        store_period_config(deps.storage, &period_config)?;

        user_claimed_periods.insert(
            period_id,
            UserPeriodClaimedDetails {
                claimed_amount: user_per_period_amount.clone(),
                claimed_time: current_time,
            },
        );
        user_total_claimed_amount += user_per_period_amount.clone();
    }
    user_period_config.user_claimed_periods = user_claimed_periods;
    user_period_config.user_total_claimed_amount += user_total_claimed_amount.clone();

    if user_period_config.user_total_claimed_amount > user_period_config.user_total_amount {
        return Err(StdError::generic_err(format!(
            "User {} claimed amount exceeds total amount",
            user_address
        )));
    }

    // transfer token to user
    let transfer_msg = cw20::Cw20ExecuteMsg::Transfer {
        recipient: user_address.clone().into(),
        amount: user_total_claimed_amount,
    };
    let transfer_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.token_address.clone().to_string(),
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    });
    msgs.push(transfer_token_msg);

    store_user_period_config(deps.storage, &user_address, &user_period_config)?;

    Ok(Response::default().add_messages(msgs).add_attributes(vec![
        ("action", "user_claim_period"),
        ("user", user_address.as_str()),
    ]))
}
