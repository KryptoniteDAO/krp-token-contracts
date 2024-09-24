use cosmwasm_std::{DepsMut, Env, from_json, MessageInfo, Response, StdError, StdResult, to_json_binary, Uint128, WasmMsg};
use crate::msg::{BatchTransferMsg, Cw20ExecuteMsg, Cw20HookMsg, UpdateConfigMsg};
use crate::receiver::Cw20ReceiveMsg;
use crate::state::{read_batch_transfer_config, store_batch_transfer_config};

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let contract_addr = info.sender.clone();
    // let msg_sender = deps.api.addr_validate(&cw20_msg.sender)?;
    match from_json(&cw20_msg.msg) {
        Ok(Cw20HookMsg::BatchTransfer { transfer_infos }) => {
            let config = read_batch_transfer_config(deps.storage)?;
            if contract_addr.ne(&config.token) {
                return Err(StdError::generic_err("Error: Invalid token address"));
            }
            batch_transfer_token(deps, env, info, cw20_msg.amount, transfer_infos)
        }
        Err(_) => Err(StdError::generic_err("data should be given")),
    }
}

pub fn batch_transfer_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    transfer_infos: Vec<BatchTransferMsg>,
) -> StdResult<Response> {
    let mut config = read_batch_transfer_config(deps.storage)?;

    let mut total_amount = Uint128::zero();
    let mut total_user_amount = Uint128::zero();
    let total_burn_fee_percent = config.burn_fee_percent;

    let mut sub_msgs = vec![];
    for transfer_info in transfer_infos {
        total_amount += transfer_info.amount;
        let recipient = deps.api.addr_validate(&transfer_info.recipient.to_string())?;
        let burn_fee = transfer_info.amount.checked_mul(total_burn_fee_percent)?.checked_div(Uint128::new(10000))?;
        let user_amount = transfer_info.amount.checked_sub(burn_fee)?;
        total_user_amount += user_amount;
        let transfer_msg = Cw20ExecuteMsg::Transfer {
            recipient: recipient.to_string(),
            amount: user_amount,
        };
        let transfer = WasmMsg::Execute {
            contract_addr: config.token.to_string(),
            msg: to_json_binary(&transfer_msg)?,
            funds: vec![],
        };
        sub_msgs.push(transfer);
    }

    if amount != total_amount {
        return Err(StdError::generic_err("Error: Invalid amount"));
    }

    let total_burn_fee = total_amount.checked_sub(total_user_amount)?;
    let burn_msg = Cw20ExecuteMsg::Burn {
        amount: total_burn_fee,
    };
    let burn = WasmMsg::Execute {
        contract_addr: config.token.to_string(),
        msg: to_json_binary(&burn_msg)?,
        funds: vec![],
    };
    sub_msgs.push(burn);

    config.burn_total_burn += total_burn_fee;
    store_batch_transfer_config(deps.storage, &config)?;

    Ok(Response::default()
        .add_messages(sub_msgs)
        .add_attributes(vec![
            ("action", "batch_transfer"),
            ("sender", info.sender.as_str()),
            ("recipient", "batch_transfer"),
            ("amount", total_amount.to_string().as_str()),
            ("total_burn_fee", total_burn_fee.to_string().as_str()),
        ]))
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    update_msg: UpdateConfigMsg,
) -> StdResult<Response> {
    let mut config = read_batch_transfer_config(deps.storage)?;
    if info.sender != config.gov {
        return Err(StdError::generic_err("unauthorized"));
    }
    if let Some(burn_fee_percent) = update_msg.burn_fee_percent {
        config.burn_fee_percent = burn_fee_percent;
    }
    if let Some(gov) = update_msg.gov {
        config.gov = gov;
    }

    if let Some(token) = update_msg.token {
        config.token = token;
    }

    store_batch_transfer_config(deps.storage, &config)?;
    Ok(Response::default())
}