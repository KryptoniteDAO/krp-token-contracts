use cosmwasm_std::{attr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, to_binary, Uint128, WasmMsg};
use crate::error::ContractError;
use crate::msg::{UpdateInviterRewardConfigMsg};
use crate::querier::{cal_can_claim_reward_token, cal_can_mint_reward_box};
use crate::state::{read_inviter_opt_detail, read_inviter_reward_config, read_inviter_reward_config_state, store_inviter_opt_detail, store_inviter_reward_config, store_inviter_reward_config_state};
use crate::third_msg::{BlindBoxExecuteMsg};

pub fn mint_reward_box(deps: DepsMut, env: Env, info: MessageInfo, level_index: u8, mint_num: u32) -> Result<Response, ContractError> {
    let user = info.sender;
    let block_time = env.block.time.seconds();
    let config = read_inviter_reward_config(deps.storage)?;
    if block_time < config.start_mint_box_time {
        return Err(ContractError::NotStartTime {});
    }
    if block_time > config.end_mint_box_time {
        return Err(ContractError::EndTimeOut {});
    }

    if mint_num == 0u32 {
        return Err(ContractError::Std(StdError::generic_err("mint_num must be greater than 0")));
    }

    let nft_contract = config.nft_contract;

    let user_can_mint_box_info = cal_can_mint_reward_box(deps.as_ref(), user.clone(), &level_index)?;

    if mint_num > user_can_mint_box_info.can_mint_box_quantity
        || user_can_mint_box_info.can_mint_box_quantity == 0u32 {
        return Err(ContractError::MintRewardBoxQuantityNotEnough {});
    }

    // save user state


    let mut user_opt_detail = read_inviter_opt_detail(deps.storage, &user)?;
    user_opt_detail.mint_box_count += mint_num;

    let mut zero = 0u32;

    let mut mint_box_level_detail = user_opt_detail.mint_box_level_detail;
    let user_level_quantity = mint_box_level_detail.get(&level_index).unwrap_or(&mut zero);

    mint_box_level_detail.insert(level_index, user_level_quantity.clone() + mint_num);


    user_opt_detail.mint_box_level_detail = mint_box_level_detail;

    //
    store_inviter_opt_detail(deps.storage, &user, &user_opt_detail)?;

    // save config state
    let mut config_state = read_inviter_reward_config_state(deps.storage)?;
    config_state.total_mint_box_count += mint_num;

    let mut zero = 0u32;
    let total_level_quantity = config_state.mint_box_level_detail.get_mut(&level_index).unwrap_or(&mut zero);
    *total_level_quantity += mint_num;

    store_inviter_reward_config_state(deps.storage, &config_state)?;

    let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: nft_contract.to_string(),
        msg: to_binary(&BlindBoxExecuteMsg::DoInviterRewardMint {
            inviter: user.clone(),
            level_index: level_index.clone(),
            mint_num: mint_num.clone(),
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attributes(vec![
            ("action", "mint_reward_box"),
            ("inviter", user.as_str()),
            ("level_index", level_index.to_string().as_str()),
            ("mint_num", mint_num.to_string().as_str()),
        ]))
}

pub fn claim_reward_token(deps: DepsMut, env: Env, info: MessageInfo, amount: Option<u128>) -> Result<Response, ContractError> {
    let user = info.sender;
    let block_time = env.block.time.seconds();
    let config = read_inviter_reward_config(deps.storage)?;

    if block_time < config.start_claim_token_time {
        return Err(ContractError::NotStartTime {});
    }
    if block_time > config.end_claim_token_time {
        return Err(ContractError::EndTimeOut {});
    }


    let claim_amount = amount.unwrap_or(0u128);

    if claim_amount == 0u128 {
        return Err(ContractError::Std(StdError::generic_err("claim_amount must be greater than 0")));
    }

    let user_can_claim_token_info = cal_can_claim_reward_token(deps.as_ref(), user.clone())?;
    if claim_amount > user_can_claim_token_info.can_claim_token_quantity
        || user_can_claim_token_info.can_claim_token_quantity == 0u128 {
        return Err(ContractError::ClaimRewardTokenQuantityNotEnough {});
    }

    // save user state
    let mut user_opt_detail = read_inviter_opt_detail(deps.storage, &user)?;
    user_opt_detail.claim_token_quantity += claim_amount;

    store_inviter_opt_detail(deps.storage, &user, &user_opt_detail)?;

    // save config state
    let mut config_state = read_inviter_reward_config_state(deps.storage)?;
    config_state.total_claim_token_quantity += claim_amount;

    store_inviter_reward_config_state(deps.storage, &config_state)?;

    let transfer_reawrd_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: user.to_string(),
        amount: vec![
            Coin {
                denom: config.reward_native_token,
                amount: Uint128::from(claim_amount.clone()),
            }],
    });

    Ok(Response::new()
        .add_message(transfer_reawrd_msg)
        .add_attributes(vec![
            ("action", "claim_reward_token"),
            ("inviter", user.as_str()),
            ("amount", claim_amount.to_string().as_str()),
        ]))
}


pub fn update_config(deps: DepsMut, info: MessageInfo, update_msg: UpdateInviterRewardConfigMsg) -> Result<Response, ContractError> {
    let mut config = read_inviter_reward_config(deps.storage)?;

    if info.sender.ne(&config.gov) {
        return Err(ContractError::Unauthorized {});
    }
    let mut attrs = vec![
        attr("action", "update_config"),
    ];

    if update_msg.gov.is_some() {
        config.gov = update_msg.gov.unwrap();
        attrs.push(attr("gov", config.gov.to_string().as_str()));
    }
    if update_msg.nft_contract.is_some() {
        config.nft_contract = update_msg.nft_contract.unwrap();
        attrs.push(attr("nft_contract", config.nft_contract.to_string().as_str()));
    }
    if update_msg.reward_native_token.is_some() {
        config.reward_native_token = update_msg.reward_native_token.unwrap();
        attrs.push(attr("reward_native_token", config.reward_native_token.to_string().as_str()));
    }

    if update_msg.start_mint_box_time.is_some() {
        config.start_mint_box_time = update_msg.start_mint_box_time.unwrap();
        attrs.push(attr("start_mint_box_time", config.start_mint_box_time.to_string().as_str()));
    }
    if update_msg.end_mint_box_time.is_some() {
        config.end_mint_box_time = update_msg.end_mint_box_time.unwrap();
        attrs.push(attr("end_mint_box_time", config.end_mint_box_time.to_string().as_str()));
    }
    if update_msg.start_claim_token_time.is_some() {
        config.start_claim_token_time = update_msg.start_claim_token_time.unwrap();
        attrs.push(attr("start_claim_token_time", config.start_claim_token_time.to_string().as_str()));
    }
    if update_msg.end_claim_token_time.is_some() {
        config.end_claim_token_time = update_msg.end_claim_token_time.unwrap();
        attrs.push(attr("end_claim_token_time", config.end_claim_token_time.to_string().as_str()));
    }

    store_inviter_reward_config(deps.storage, &config)?;

    Ok(Response::new()
        .add_attributes(attrs))
}
