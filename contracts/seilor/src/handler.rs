use crate::helper::is_empty_str;
use crate::mint_receiver::Cw20MintReceiveMsg;
use crate::state::{read_seilor_config, store_seilor_config};
use cosmwasm_std::{attr, Addr, Binary, DepsMut, Env, MessageInfo, Response, StdError, Uint128};
use cw20_base::contract::execute_mint;
use cw20_base::ContractError;

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    fund: Option<Addr>,
    distribute: Option<Addr>,
    cross_chain_swap_contract: Option<Addr>,
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
        deps.api.addr_validate(fund.clone().as_str())?;
        seilor_config.fund = fund.clone();
        attrs.push(attr("fund", fund.to_string()));
    }
    if let Some(distribute) = distribute {
        deps.api.addr_validate(distribute.clone().as_str())?;
        seilor_config.distribute = distribute.clone();
        attrs.push(attr("distribute", distribute.to_string()));
    }
    if let Some(cross_chain_swap_contract) = cross_chain_swap_contract {
        deps.api
            .addr_validate(cross_chain_swap_contract.clone().as_str())?;
        seilor_config.cross_chain_swap_contract = Some(cross_chain_swap_contract.clone());
        attrs.push(attr(
            "cross_chain_swap_contract",
            cross_chain_swap_contract.to_string(),
        ));
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
    let cross_chain_swap_contract = seilor_config.cross_chain_swap_contract;

    if is_empty_str(fund.as_str()) && is_empty_str(distribute.as_str()) {
        return Err(ContractError::Std(StdError::generic_err(
            "Fund or distribute contract must to be configured",
        )));
    }

    if msg_sender.ne(&fund) && msg_sender.ne(&distribute) {
        if (cross_chain_swap_contract.clone().is_some()
            && msg_sender.ne(&cross_chain_swap_contract.clone().unwrap()))
            || cross_chain_swap_contract.is_none()
        {
            return Err(ContractError::Unauthorized {});
        }
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

// Burn has been modified to directly inherit the standard, and this modification will add gas to the VE module stacking. And complexity.
// pub fn burn(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     user: Addr,
//     amount: u128,
// ) -> Result<Response, ContractError> {
//     let seilor_config = read_seilor_config(deps.storage)?;
//     let msg_sender = info.sender;
//     let fund = seilor_config.fund;
//
//     if msg_sender != fund.clone() {
//         return Err(ContractError::Unauthorized {});
//     }
//
//     let sub_info = MessageInfo {
//         sender: user,
//         funds: vec![],
//     };
//     execute_burn(deps, env.clone(), sub_info, Uint128::from(amount))
// }

pub fn set_gov(deps: DepsMut, info: MessageInfo, gov: Addr) -> Result<Response, ContractError> {
    let mut seilor_config = read_seilor_config(deps.storage)?;
    if seilor_config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    deps.api.addr_validate(gov.clone().as_str())?;

    seilor_config.new_gov = Some(gov.clone());
    store_seilor_config(deps.storage, &seilor_config)?;
    Ok(Response::default().add_attributes(vec![
        attr("action", "set_gov"),
        attr("gov", gov.to_string()),
    ]))
}

pub fn accept_gov(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut seilor_config = read_seilor_config(deps.storage)?;
    if seilor_config.new_gov.is_none() {
        return Err(ContractError::Std(StdError::generic_err(
            "No new gov to accept",
        )));
    }
    if info.sender != seilor_config.new_gov.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    seilor_config.gov = info.sender.clone();
    seilor_config.new_gov = None;
    store_seilor_config(deps.storage, &seilor_config)?;
    Ok(Response::default().add_attributes(vec![
        attr("action", "accept_gov"),
        attr("gov", seilor_config.gov.clone().to_string()),
    ]))
}
