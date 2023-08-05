use cw20_base::ContractError;
use crate::helper::is_empty_str;
use crate::mint_receiver::Cw20MintReceiveMsg;
use crate::state::{read_seilor_config, store_seilor_config};
use cosmwasm_std::{attr, Addr, Binary, DepsMut, Env, MessageInfo, Response, StdError, Uint128};
use cw20_base::contract::{execute_burn, execute_mint};

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    fund: Option<Addr>,
    gov: Option<Addr>,
    distribute: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut seilor_config = read_seilor_config(deps.storage)?;

    if info.sender != seilor_config.gov {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![
        attr("action", "update_config"),
        attr("sender", info.sender.to_string()),
    ];

    if let Some(fund) = fund {
        seilor_config.fund = fund.clone();
        attrs.push(attr("fund", fund.to_string()));
    }
    if let Some(gov) = gov {
        seilor_config.gov = gov.clone();
        attrs.push(attr("gov", gov.to_string()));
    }
    if let Some(distribute) = distribute {
        seilor_config.distribute = distribute.clone();
        attrs.push(attr("distribute", distribute.to_string()));
    }

    store_seilor_config(deps.storage, &seilor_config)?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: Addr,
    amount: Uint128,
    contract: Option<String>,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    let msg_sender = info.sender;
    let seilor_config = read_seilor_config(deps.storage)?;
    let fund = seilor_config.fund;
    let distribute = seilor_config.distribute;

    if is_empty_str(fund.as_str()) && is_empty_str(distribute.as_str()) {
        return Err(ContractError::Std(StdError::generic_err("Fund or distribute contract must to be configured")));
    }

    if msg_sender.ne(&fund.clone()) && msg_sender.ne(&distribute) {
        return Err(ContractError::Unauthorized {});
    }

    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };

    let mut cw20_res = execute_mint(
        deps.branch(),
        env,
        sub_info,
        user.clone().to_string(),
        amount.clone(),
    )?;
    // if cw20_res.is_err() {
    //     return Err(ContractError::Std(StdError::generic_err(
    //         cw20_res.err().unwrap().to_string(),
    //     )));
    // }

    // let mut res = cw20_res.unwrap();

    if let Some(contract) = contract {
        if let Some(msg) = msg {
            cw20_res = cw20_res.add_message(
                Cw20MintReceiveMsg {
                    sender: msg_sender.into(),
                    amount,
                    msg,
                }
                .into_cosmos_msg(contract)?,
            );
        }
    }

    Ok(cw20_res)
}

pub fn burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: Addr,
    amount: u128,
) -> Result<Response, ContractError> {
    let seilor_config = read_seilor_config(deps.storage)?;
    let msg_sender = info.sender;
    let fund = seilor_config.fund;

    if msg_sender != fund.clone() {
        return Err(ContractError::Unauthorized {});
    }

    let sub_info = MessageInfo {
        sender: user,
        funds: vec![],
    };
    execute_burn(deps, env.clone(), sub_info, Uint128::from(amount))
}
