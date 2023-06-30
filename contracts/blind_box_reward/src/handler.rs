use cosmwasm_std::{Addr, attr, DepsMut, Env, MessageInfo, QueryRequest, Response, StdError, to_binary, WasmQuery};
use cw721::OwnerOfResponse;
use crate::error::ContractError;
use crate::random_role::find_random_rule;
use crate::state::{BoxOpenInfo, get_box_reward_config, get_box_reward_config_state, get_reward_config, is_box_open, RandomBoxRewardRuleConfigState, set_box_open_info, set_box_reward_config, set_box_reward_config_state, set_reward_config};
use crate::third_msg::{BlindBoxInfoResponse, BlindBoxQueryMsg};

//config

pub fn update_reward_config(
    deps: DepsMut,
    info: MessageInfo,
    gov: Option<Addr>,
    nft_contract: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut config = get_reward_config(deps.storage)?;
    if config.gov.ne(&info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![
        attr("action", "update_reward_config"),
    ];

    if let Some(gov) = gov {
        config.gov = gov.clone();
        attrs.push(attr("gov", gov.as_str()));
    }

    if let Some(nft_contract) = nft_contract {
        config.nft_contract = nft_contract.clone();
        attrs.push(attr("nft_contract", nft_contract.as_str()));
    }
    set_reward_config(deps.storage, &config)?; // store config
    Ok(Response::new().add_attributes(attrs))
}

pub fn update_box_reward_config(
    deps: DepsMut,
    info: MessageInfo,
    box_reward_token: Option<Addr>,
    box_open_time: Option<u64>,
) -> Result<Response, ContractError> {
    let config = get_reward_config(deps.storage)?;
    if config.gov.ne(&info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }


    let mut box_config = get_box_reward_config(deps.storage)?;
    let mut attrs = vec![
        attr("action", "update_box_reward_config"),
    ];

    if let Some(box_reward_token) = box_reward_token {
        box_config.box_reward_token = box_reward_token.clone();
        attrs.push(attr("box_reward_token", box_reward_token.as_str()));
    }

    if let Some(box_open_time) = box_open_time {
        box_config.box_open_time = box_open_time.clone();
        attrs.push(attr("box_open_time", box_open_time.to_string()));
    }
    set_box_reward_config(deps.storage, &box_config)?; // store config
    Ok(Response::new().add_attributes(attrs))
}


// biz

pub fn open_blind_box(mut deps: DepsMut, env: Env, info: MessageInfo, token_ids: Vec<String>) -> Result<Response, ContractError> {

    // deps.querier.query(&QueryRequest::Bank())

    let block_time = env.block.time.seconds();
    let user = info.sender.clone();

    if token_ids.len() == 0 {
        return Err(ContractError::Std(StdError::generic_err("token_ids is empty.")));
    }
    for token_id in token_ids.clone() {
        _open_single_blind_box(deps.branch(), env.clone(), block_time, user.clone(), token_id)?;
    }

    Ok(Response::new().add_attributes(vec![
        attr("action", "open_blind_box"),
        attr("user", user.as_str()),
        attr("token_ids", token_ids.join(",").as_str()),
    ]))
}


fn _open_single_blind_box(deps: DepsMut, env: Env, block_time: u64, user: Addr, token_id: String) -> Result<(), ContractError> {
    if token_id.len() == 0 {
        return Err(ContractError::Std(StdError::generic_err("token_id is empty.")));
    }
    let is_box_open = is_box_open(deps.storage, token_id.clone())?;

    if is_box_open {
        return Err(ContractError::BoxAlreadyOpen {});
    }
    let config = get_reward_config(deps.storage)?;

    // check nft owner
    let owner_of_nft_resp: OwnerOfResponse = deps.querier.clone().query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.nft_contract.to_string(),
        msg: to_binary(&cw721::Cw721QueryMsg::OwnerOf { token_id: token_id.clone(), include_expired: None })?,
    }))?;

    if owner_of_nft_resp.owner.ne(&user) {
        return Err(ContractError::Std(StdError::generic_err("user is not owner of nft.")));
    }

    // check nft level info
    let nft_level_info_resp: BlindBoxInfoResponse = deps.querier.clone().query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.nft_contract.to_string(),
        msg: to_binary(&BlindBoxQueryMsg::QueryBlindBoxInfo { token_id: token_id.clone() })?,
    }))?;

    if nft_level_info_resp.block_number == 0u64 {
        return Err(ContractError::Std(StdError::generic_err("nft level info not found by token id.")));
    }

    let box_config = get_box_reward_config(deps.storage)?;
    if block_time < box_config.box_open_time {
        return Err(ContractError::Std(StdError::generic_err("box not open yet.")));
    }

    let level_index = nft_level_info_resp.level_index;
    let mut box_config_state = get_box_reward_config_state(deps.storage)?;


    let open_box_amount;
    if level_index != box_config.random_in_box_level_index {
        // ordinary box
        let ordinary_box_level_config = box_config.ordinary_box_reward_level_config.get(&level_index).unwrap();

        let ordinary_box_level_config_state =
            box_config_state.ordinary_box_reward_level_config_state.get_mut(&level_index).unwrap();
        if ordinary_box_level_config.max_reward_count <= ordinary_box_level_config_state.total_open_box_count {
            return Err(ContractError::Std(StdError::generic_err("reward count reach max.")));
        }
        open_box_amount = ordinary_box_level_config.reward_amount;
        ordinary_box_level_config_state.total_open_box_count += 1;
        ordinary_box_level_config_state.total_reward_amount += open_box_amount;
        box_config_state.ordinary_total_open_box_count += 1;
        box_config_state.ordinary_total_reward_amount += open_box_amount;
    } else {
        //random box
        let rules = box_config.random_box_reward_rule_config;
        let mut rules_state = box_config_state.random_box_reward_rule_config_state;
        let find_rule = find_random_rule(env, token_id.clone(), &rules, &rules_state)?;
        let find_rules_state_index = find_rule.random_box_index.clone();
        let find_rules_state: &mut RandomBoxRewardRuleConfigState =
            rules_state.get_mut(find_rules_state_index as usize).unwrap();
        if find_rule.max_reward_count <= find_rules_state.total_open_box_count {
            return Err(ContractError::Std(StdError::generic_err("reward count reach max.")));
        }
        open_box_amount = find_rule.random_reward_amount;
        find_rules_state.total_open_box_count += 1;
        find_rules_state.total_reward_amount += open_box_amount;
        box_config_state.random_total_open_box_count += 1;
        box_config_state.random_total_reward_amount += open_box_amount;
        rules_state[find_rule.random_box_index as usize] = find_rules_state.clone();
        box_config_state.random_box_reward_rule_config_state = rules_state;
    }

    //set box info
    let box_info = BoxOpenInfo {
        open_user: user.clone(),
        open_reward_amount: open_box_amount,
        open_box_time: block_time,
        is_random_box: nft_level_info_resp.is_random_box
    };

    set_box_open_info(deps.storage, token_id, &box_info)?;

    // set config state
    set_box_reward_config_state(deps.storage, &box_config_state)?;

    Ok(())
}


