use cosmwasm_std::{Addr, attr, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, to_binary, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
use crate::error::ContractError;
use crate::querier::query_user_claim_rewards;
use crate::state::{read_blind_box_reward_config, read_blind_box_reward_token_config, RewardLevelConfig, store_blind_box_reward_config, store_blind_box_reward_token_config};

pub fn update_blind_box_reward_config(
    deps: DepsMut,
    info: MessageInfo,
    gov: Option<Addr>,
    nft_contract: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut config = read_blind_box_reward_config(deps.storage)?;
    if config.gov.ne(&info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![
        attr("action", "update_blind_box_reward_config"),
    ];

    if let Some(gov) = gov {
        config.gov = gov.clone();
        attrs.push(attr("gov", gov.as_str()));
    }

    if let Some(nft_contract) = nft_contract {
        config.nft_contract = nft_contract.clone();
        attrs.push(attr("nft_contract", nft_contract.as_str()));
    }
    store_blind_box_reward_config(deps.storage, &config)?; // store config
    Ok(Response::new().add_attributes(attrs))
}

pub fn update_blind_box_reward_token_config(deps: DepsMut, info: MessageInfo,
                                            reward_token: Addr,
                                            total_reward_amount: u128,
                                            claimable_time: u64,
) -> Result<Response, ContractError> {
    let config = read_blind_box_reward_config(deps.storage)?;
    if config.gov.ne(&info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }
    let mut reward_token_config = read_blind_box_reward_token_config(deps.storage, reward_token.to_string().clone());
    reward_token_config.total_reward_amount = total_reward_amount.clone();
    reward_token_config.claimable_time = claimable_time.clone();

    store_blind_box_reward_token_config(deps.storage, reward_token.clone().to_string(), &reward_token_config)?; // store config

    Ok(Response::new().add_attributes(vec![
        attr("action", "update_blind_box_reward_token_config"),
        attr("reward_token", reward_token.to_string()),
        attr("total_reward_amount", total_reward_amount.to_string()),
        attr("claimable_time", claimable_time.to_string()),
    ]))
}


pub fn update_reward_token_reward_level(deps: DepsMut, info: MessageInfo,
                                        reward_token: Addr,
                                        reward_level: u8,
                                        reward_amount: u128,
) -> Result<Response, ContractError> {
    let config = read_blind_box_reward_config(deps.storage)?;
    if config.gov.ne(&info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }
    let level_index = reward_level as usize;

    let mut reward_token_config = read_blind_box_reward_token_config(deps.storage, reward_token.to_string().clone());

    if reward_token_config.claimable_time == 0 {
        return Err(ContractError::ClaimableTimeNotSet {});
    }

    let level_count = reward_token_config.reward_levels.len();
    // add
    if level_count == 0 || level_index > (level_count - 1) {
        reward_token_config.reward_levels.push(RewardLevelConfig {
            reward_amount: reward_amount.clone(),
            level_total_claimed_amount: 0,
        });
    } else {
        reward_token_config.reward_levels[level_index].reward_amount = reward_amount.clone();
    }

    store_blind_box_reward_token_config(deps.storage, reward_token.clone().to_string(), &reward_token_config)?; // store config

    Ok(Response::new().add_attributes(vec![
        attr("action", "update_reward_token_reward_level"),
        attr("reward_token", reward_token.to_string()),
        attr("reward_level", reward_level.to_string()),
        attr("reward_amount", reward_amount.to_string()),
    ]))
}

pub fn claim_reward(deps: DepsMut, env: Env, info: MessageInfo, recipient: Option<Addr>) -> Result<Response, ContractError> {
    let sender = info.sender.clone();
    let claim_rewards = query_user_claim_rewards(deps.as_ref(), env.clone(), sender.clone().to_string())?;
    let mut sub_msgs = vec![];
    let _recipient = recipient.unwrap_or(sender.clone());

    for user_claimable_reward in claim_rewards {
        let reward_token = user_claimable_reward.reward_token;
        if user_claimable_reward.claimable_reward > 0 {
            sub_msgs.push(SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: reward_token.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: _recipient.clone().to_string(),
                    amount: Uint128::from(user_claimable_reward.claimable_reward.clone()),
                })?,
                funds: vec![],
            })));
            let mut reward_token_config = read_blind_box_reward_token_config(deps.storage, reward_token.clone().to_string());
            reward_token_config.total_claimed_amount += user_claimable_reward.claimable_reward;
            reward_token_config.total_claimed_count += user_claimable_reward.claimable_reward_details.len() as u128;
            for reward_detail in user_claimable_reward.claimable_reward_details {
                let level_index = reward_detail.level_index as usize;
                reward_token_config.reward_levels[level_index].level_total_claimed_amount += reward_detail.claimable_reward;
            }

            store_blind_box_reward_token_config(deps.storage, reward_token.clone().to_string(), &reward_token_config)?; // store config
        }
    }

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "claim_reward"),
            attr("recipient", _recipient.to_string()),
        ])
        .add_submessages(sub_msgs)
    )
}

