use crate::error::ContractError;
use crate::msg::FundMsg;
use crate::state::{is_minter, read_vote_config, store_minters, store_vote_config};
use crate::ve_handler::{ve_burn, ve_mint};
use cosmwasm_std::{
    attr, to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, SubMsg,
    Uint128, WasmMsg,
};
use std::ops::{Add, Sub};

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    max_minted: Option<Uint128>,
    fund: Option<Addr>,
    gov: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut vote_config = read_vote_config(deps.storage)?;

    if info.sender != vote_config.gov {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![
        attr("action", "update_config"),
        attr("sender", info.sender.to_string()),
    ];
    if let Some(max_minted) = max_minted {
        // verify that max_minted should be greater than total_minted.
        if max_minted < vote_config.total_minted {
            return Err(ContractError::Std(StdError::generic_err(
                "max_minted should be greater than total_minted".to_string(),
            )));
        }
        vote_config.max_minted = max_minted.clone();
        attrs.push(attr("max_minted", max_minted.to_string()));
    }
    if let Some(fund) = fund {
        vote_config.fund = fund.clone();
        attrs.push(attr("fund", fund.to_string()));
    }
    if let Some(gov) = gov {
        vote_config.gov = gov.clone();
        attrs.push(attr("gov", gov.to_string()));
    }

    store_vote_config(deps.storage, &vote_config)?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn set_minters(
    deps: DepsMut,
    info: MessageInfo,
    contracts: Vec<Addr>,
    is_minter: Vec<bool>,
) -> Result<Response, ContractError> {
    let vote_config = read_vote_config(deps.storage)?;

    if info.sender != vote_config.gov {
        return Err(ContractError::Unauthorized {});
    }
    if contracts.len() != is_minter.len() {
        return Err(ContractError::InvalidInput {});
    }
    let mut attrs = vec![];
    attrs.push(("action", "set_minters"));
    for i in 0..contracts.len() {
        let contract = contracts[i].clone();
        let _ = store_minters(deps.storage, contract.clone(), &is_minter[i]);
    }
    Ok(Response::new().add_attributes(attrs))
}

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: Addr,
    amount: u128,
) -> Result<Response, ContractError> {
    let mut vote_config = read_vote_config(deps.storage)?;
    let msg_sender = info.sender.clone();
    let fund = vote_config.fund.clone();

    if msg_sender.ne(&fund.clone()) && !is_minter(deps.storage, msg_sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    if 0 == amount {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid zero amount".to_string(),
        )));
    }

    let mut reward = amount;
    let mut sub_msgs: Vec<SubMsg> = vec![];
    if msg_sender.ne(&fund) {
        let refresh_reward_msg = FundMsg::RefreshReward {
            account: user.clone(),
        };
        let sub_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: fund.clone().to_string(),
            msg: to_binary(&refresh_reward_msg)?,
            funds: vec![],
        }));
        sub_msgs.push(sub_msg);

        if vote_config.total_minted.clone().add(Uint128::from(reward))
            > vote_config.max_minted.clone()
        {
            reward = vote_config
                .max_minted
                .clone()
                .sub(vote_config.total_minted.clone())
                .u128();
        }
        vote_config.total_minted = vote_config.total_minted.clone().add(Uint128::from(reward));
        store_vote_config(deps.storage, &vote_config)?;
    }

    let ve_res = ve_mint(deps, env, user, reward)?;

    Ok(Response::new()
        .add_submessages(sub_msgs)
        .add_attributes(ve_res.attributes))
}

pub fn burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: Addr,
    amount: u128,
) -> Result<Response, ContractError> {
    let vote_config = read_vote_config(deps.storage)?;
    let msg_sender = info.sender;
    let fund = vote_config.fund;

    if msg_sender != fund.clone() && !is_minter(deps.storage, msg_sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    let mut sub_msgs: Vec<SubMsg> = vec![];
    if msg_sender.ne(&fund) {
        let refresh_reward_msg = FundMsg::RefreshReward {
            account: user.clone(),
        };
        let sub_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: fund.clone().to_string(),
            msg: to_binary(&refresh_reward_msg)?,
            funds: vec![],
        }));
        sub_msgs.push(sub_msg);
    }
    let ve_res = ve_burn(deps, env, user, amount)?;

    Ok(Response::new()
        .add_submessages(sub_msgs)
        .add_attributes(ve_res.attributes))
}
