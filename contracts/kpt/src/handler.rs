use cosmwasm_std::{Addr, attr, DepsMut, Env, MessageInfo, Response, StdError, Uint128};
use cw20_base::contract::{execute_burn, execute_mint};
use crate::error::ContractError;
use crate::state::{read_kpt_config, store_kpt_config};

pub fn update_config(deps: DepsMut, info: MessageInfo, max_supply: Option<Uint128>, kpt_fund: Option<Addr>, gov: Option<Addr>) -> Result<Response, ContractError> {
    let mut kpt_config = read_kpt_config(deps.storage)?;

    if info.sender != kpt_config.gov {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![attr("action", "update_config"), attr("sender", info.sender.to_string())];
    if let Some(max_supply) = max_supply {
        kpt_config.max_supply = max_supply.clone().u128();
        attrs.push(attr("max_supply", max_supply.to_string()));
    }
    if let Some(kpt_fund) = kpt_fund {
        kpt_config.kpt_fund = kpt_fund.clone();
        attrs.push(attr("kpt_fund", kpt_fund.to_string()));
    }
    if let Some(gov) = gov {
        kpt_config.gov = gov.clone();
        attrs.push(attr("gov", gov.to_string()));
    }

    store_kpt_config(deps.storage, &kpt_config)?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn mint(deps: DepsMut, env: Env, info: MessageInfo, user: Addr, amount: u128) -> Result<Response, ContractError> {
    let msg_sender = info.sender.clone();
    let kpt_fund = read_kpt_config(deps.storage)?.kpt_fund.clone();

    if msg_sender.ne(&kpt_fund.clone()) {
        return Err(ContractError::Unauthorized {});
    }
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };

    let cw20_res = execute_mint(deps, env, sub_info, user.clone().to_string(), Uint128::from(amount.clone()));
    if cw20_res.is_err() {
        return Err(ContractError::Std(StdError::generic_err(cw20_res.err().unwrap().to_string())));
    }


    Ok(Response::new()
        .add_attributes(cw20_res.unwrap().attributes))
}

pub fn burn(deps: DepsMut, env: Env, info: MessageInfo, user: Addr, amount: u128) -> Result<Response, ContractError> {
    let kpt_config = read_kpt_config(deps.storage)?;
    let msg_sender = info.sender.clone();
    let kpt_fund = kpt_config.kpt_fund;

    if msg_sender != kpt_fund.clone() {
        return Err(ContractError::Unauthorized {});
    }

    let sub_info = MessageInfo {
        sender: user.clone(),
        funds: vec![],
    };
    let cw20_res = execute_burn(deps, env.clone(), sub_info, Uint128::from(amount));
    if cw20_res.is_err() {
        return Err(ContractError::Std(StdError::generic_err(cw20_res.err().unwrap().to_string())));
    }

    Ok(Response::new()
        .add_attributes(cw20_res.unwrap().attributes))
}