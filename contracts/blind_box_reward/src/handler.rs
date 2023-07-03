use cosmwasm_std::{Addr, attr, CosmosMsg, DepsMut, Env, from_binary, MessageInfo, QueryRequest, Response, StdError, to_binary, Uint128, WasmMsg, WasmQuery};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw721::OwnerOfResponse;
use crate::error::ContractError;
use crate::helper::{calc_claim_amount, calc_claim_index, is_empty_str};
use crate::msg::Cw20HookMsg;
use crate::querier::{query_box_claimable_infos};
use crate::random_role::find_random_rule;
use crate::state::{BoxOpenInfo, get_box_open_info, get_box_reward_config, get_box_reward_config_state, get_reward_config, is_box_open, RandomBoxRewardRuleConfigState, set_box_open_info, set_box_reward_config, set_box_reward_config_state, set_reward_config};
use crate::third_msg::{BlindBoxInfoResponse, BlindBoxQueryMsg, DistributeExecuteMsg};

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

    let ntf_owner = _get_owner_by_token_id(&deps, &token_id, config.nft_contract.clone().to_string())?;
    if user.ne(&ntf_owner) {
        return Err(ContractError::Std(StdError::generic_err("user is not nft owner.")));
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
        is_random_box: nft_level_info_resp.is_random_box,
        is_reward_box: nft_level_info_resp.is_reward_box,
        reward_claim_index: 0,
        reward_claimed_amount: 0,
    };

    set_box_open_info(deps.storage, token_id, &box_info)?;

    // set config state
    set_box_reward_config_state(deps.storage, &box_config_state)?;

    Ok(())
}

fn _get_owner_by_token_id(deps: &DepsMut, token_id: &String, nft_contract: String) -> Result<String, ContractError> {
    let owner_of_nft_resp: OwnerOfResponse = deps.querier.clone().query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_contract,
        msg: to_binary(&cw721::Cw721QueryMsg::OwnerOf { token_id: token_id.clone(), include_expired: None })?,
    }))?;
    Ok(owner_of_nft_resp.owner)
}


pub fn user_claim_nft_reward(deps: DepsMut, info: MessageInfo, token_ids: Vec<String>) -> Result<Response, ContractError> {
    let user = info.sender;
    let config = get_reward_config(deps.storage)?;
    let nft_contract = config.nft_contract.clone().to_string();
    let box_config = get_box_reward_config(deps.storage)?;
    let box_reward_distribute_addr = box_config.box_reward_distribute_addr;
    if is_empty_str(&box_reward_distribute_addr.to_string()) || is_empty_str(&box_config.box_reward_distribute_rule_type) {
        return Err(ContractError::Std(StdError::generic_err("box reward distribute addr not set.")));
    }

    // check tokens owner
    for token_id in token_ids.iter().clone() {
        let ntf_owner = _get_owner_by_token_id(&deps, &token_id, nft_contract.to_string())?;
        if user.ne(&ntf_owner) {
            return Err(ContractError::Std(StdError::generic_err("user is not nft owner.")));
        }
    }

    let box_claimable_infos = query_box_claimable_infos(deps.as_ref(), token_ids.clone())?;

    // claim reward from distribute contract
    let mut cosmos_msgs = vec![];
    if box_claimable_infos.total_claimable_amount > 0u128 {
        // send the claim amount to user
        let claim_msg = DistributeExecuteMsg::Claim {
            rule_type: box_config.box_reward_distribute_rule_type
        };
        let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: box_reward_distribute_addr.to_string(),
            msg: to_binary(&claim_msg)?,
            funds: vec![],
        });
        cosmos_msgs.push(mint_msg);
    }

    Ok(Response::new()
        .add_messages(cosmos_msgs)
        .add_attributes([
            attr("action", "user_claim_nft_reward"),
            attr("user", user.as_str()),
            attr("claimable_amount", box_claimable_infos.total_claimable_amount.to_string().as_str()),
        ]))
}

pub fn update_claim_nft_reward(deps: DepsMut, claimed_amount: u128, token_ids: Vec<String>) -> Result<Response, ContractError> {
    let config = get_reward_config(deps.storage)?;
    let nft_contract = config.nft_contract.clone().to_string();
    let box_config = get_box_reward_config(deps.storage)?;
    if is_empty_str(&box_config.box_reward_distribute_addr.to_string()) || is_empty_str(&box_config.box_reward_distribute_rule_type) {
        return Err(ContractError::Std(StdError::generic_err("box reward distribute addr not set.")));
    }
    let config_state = get_box_reward_config_state(deps.storage)?;

    let global_claim_index = calc_claim_index(claimed_amount, box_config.global_reward_total_amount, config_state.global_reward_claim_index)?;


    // check tokens owner
    let mut attrs = vec![
        attr("action", "update_claim_nft_reward"),
        attr("global_claim_index", &global_claim_index.to_string())];
    let mut cosmos_msgs = vec![];
    for token_id in token_ids.iter() {
        let nft_owner = _get_owner_by_token_id(&deps, &token_id, nft_contract.to_string())?;
        let mut box_open_info = get_box_open_info(deps.storage, token_id.clone())?;
        let nft_claimable_amount = calc_claim_amount(global_claim_index,
                                                     box_open_info.open_reward_amount, box_open_info.reward_claim_index)?;
        if nft_claimable_amount > 0u128 {
            // save box open info
            box_open_info.reward_claim_index = global_claim_index;
            box_open_info.reward_claimed_amount += nft_claimable_amount;

            if box_open_info.reward_claimed_amount > box_open_info.open_reward_amount {
                return Err(ContractError::Std(StdError::generic_err("reward claimed amount is greater than reward total amount.")));
            }

            set_box_open_info(deps.storage, token_id.clone(), &box_open_info)?;
            attrs.push(attr(token_id, nft_claimable_amount.to_string().as_str()));
            // msg to user

            cosmos_msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: box_config.box_reward_token.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: nft_owner.to_string(),
                    amount: Uint128::from(nft_claimable_amount),
                })?,
                funds: vec![],
            }));
        }
    }


    attrs.push(attr("claimed_amount", claimed_amount.to_string()));
    // save config state
    let mut config_state = get_box_reward_config_state(deps.storage)?;
    config_state.global_reward_claim_index = global_claim_index;
    config_state.global_reward_claim_total_amount += claimed_amount;

    Ok(Response::new()
        .add_messages(cosmos_msgs)
        .add_attributes(attrs))
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
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let contract_addr = info.sender.clone();
    // let msg_sender = deps.api.addr_validate(&cw20_msg.sender).unwrap();
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::ClaimNftReward { token_ids }) => {
            let box_config = get_box_reward_config(deps.storage)?;
            if box_config.box_reward_token.ne(&contract_addr) {
                return Err(ContractError::Std(StdError::generic_err("invalid box reward token.")));
            }
            update_claim_nft_reward(deps, cw20_msg.amount.u128(), token_ids)
        }
        Err(_) => Err(ContractError::Std(StdError::generic_err(
            "data should be given",
        ))),
    }
}