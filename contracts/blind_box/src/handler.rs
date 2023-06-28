use std::ops::Mul;
use cosmwasm_std::{Addr, attr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, Uint128};
use cw721_base::ContractError;
use crate::helper::{BASE_RATE, check_referral_code_role, is_empty_address};
use crate::msg::{ReferralLevelConfigMsg, ReferralLevelRewardBoxConfigMsg};
use crate::querier::{cal_mint_info, query_tokens};
use crate::state::{BlindBoxConfig, BlindBoxInfo, BlindBoxLevel, check_invitee_existence, InviterReferralRecord, read_blind_box_config, read_referral_reward_config, read_referral_reward_total_state, read_user_info, read_user_referral_code, store_blind_box_config, store_blind_box_info, store_inviter_record_elem, store_referral_reward_config, store_referral_reward_total_state, store_user_info, store_user_referral_code};

pub fn create_referral_info(deps: DepsMut, env: Env, info: MessageInfo, referral_code: String, reward_token_type: String) -> Result<Response, ContractError> {
    check_referral_code_role(referral_code.clone())?;

    let user_addr = info.sender.clone();

    // must address has nft
    let user_tokens = query_tokens(deps.as_ref(), env, user_addr.clone().to_string(), None, None)?;
    if user_tokens.tokens.is_empty() {
        return Err(ContractError::Std(StdError::generic_err("user has no nft")));
    }

    let mut user_info = read_user_info(deps.storage, &user_addr);
    if !user_info.referral_code.is_empty() {
        return Err(ContractError::Std(StdError::generic_err("referral_code already exists")));
    }

    let reward_token_config = read_referral_reward_config(deps.storage)?.reward_token_config;
    if !reward_token_config.contains_key(reward_token_type.as_str()) {
        return Err(ContractError::Std(StdError::generic_err("selected reward_token_type exists")));
    }

    let inviter = read_user_referral_code(deps.storage, referral_code.clone());
    if !is_empty_address(&inviter.to_string()) {
        return Err(ContractError::Std(StdError::generic_err("referral_code exists")));
    }

    user_info.referral_code = referral_code.clone();
    user_info.user_reward_token_type = reward_token_type.clone();
    store_user_info(deps.storage, &user_addr, &user_info)?;

    store_user_referral_code(deps.storage, referral_code.clone(), &user_addr)?;


    Ok(Response::new().add_attributes(vec![
        ("action", "set_referral_code"),
        ("referral_code", referral_code.as_str()),
        ("reward_token_type", reward_token_type.as_str()),
    ]))
}


pub fn modify_reward_token_type(deps: DepsMut, env: Env, info: MessageInfo, reward_token_type: String) -> Result<Response, ContractError> {
    let user_addr = info.sender.clone();
    let mut user_info = read_user_info(deps.storage, &user_addr);
    if !user_info.user_reward_token_type.is_empty() {
        return Err(ContractError::Std(StdError::generic_err("user_reward_token_type not exists")));
    }

    let reward_token_config = read_referral_reward_config(deps.storage)?.reward_token_config;
    if !reward_token_config.contains_key(reward_token_type.as_str()) {
        return Err(ContractError::Std(StdError::generic_err("selected reward_token_type exists")));
    }

    let end_mint_time = read_blind_box_config(deps.storage)?.end_mint_time;

    if end_mint_time < env.block.time.seconds() {
        return Err(ContractError::Std(StdError::generic_err("end_mint_time < block time, modifying the reward type is not allowed.")));
    }

    user_info.user_reward_token_type = reward_token_type.clone();
    store_user_info(deps.storage, &user_addr, &user_info)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "modify_reward_token_type"),
        ("reward_token_type", reward_token_type.as_str()),
    ]))
}

pub fn do_mint(mut deps: DepsMut, env: Env, info: MessageInfo,
               level_index: u8, mint_num: u128,
               recipient: Option<String>, referral_code: Option<String>) -> Result<Response, ContractError> {
    if mint_num < 1 {
        return Err(ContractError::Std(StdError::generic_err("mint_num must > 0")));
    }

    let _level_index = level_index as usize;
    let user = info.sender.clone();
    let mut blind_box_config = read_blind_box_config(deps.storage)?;
    let mut level_info = blind_box_config.level_infos[_level_index].clone();
    let price = level_info.price;
    level_info.minted_count = level_info.minted_count + mint_num.clone();

    if level_info.minted_count > level_info.mint_total_count {
        return Err(ContractError::Std(StdError::generic_err("minted_count > mint_total_count")));
    }

    let payment = info.funds.iter()
        .find(|x| x.denom.eq(&blind_box_config.clone().price_token))
        .ok_or_else(|| StdError::generic_err("payment token not found"))?;

    let amount = payment.amount;


    level_info.received_total_amount += amount.u128();

    let start_token_id_index = blind_box_config.clone().token_id_index + 1;
    let token_id_prefix = blind_box_config.clone().token_id_prefix;

    let current_token_ids = (start_token_id_index..start_token_id_index + mint_num)
        .map(|i| format!("{}{}", token_id_prefix, i))
        .collect::<Vec<_>>();

    for token_id in &current_token_ids {
        store_blind_box_info(
            deps.storage,
            token_id.clone(),
            &BlindBoxInfo {
                level_index,
                price: price.clone(),
                block_number: env.block.height,
            },
        )?;
    }

    blind_box_config.token_id_index += mint_num;
    blind_box_config.level_infos[_level_index] = level_info;


    store_blind_box_config(deps.storage, &blind_box_config)?;


    let owner = recipient.unwrap_or(info.sender.to_string());

    let (paid_amount, mint_discount_rate, inviter) = _do_inviter(deps.branch(), env.clone(), user.clone(),
                                                                 current_token_ids.clone(), level_index,
                                                                 referral_code.clone())?;


    if amount.ne(&paid_amount.clone()) {
        return Err(ContractError::Std(StdError::generic_err("amount not eq price")));
    }

    _do_invitee(deps.branch(), user, referral_code.clone(), inviter.clone(), mint_discount_rate.clone())?;

    // transfer payment to receiver
    let real_payment = Coin {
        denom: payment.denom.clone(),
        amount: paid_amount.clone(),
    };
    let sender_amount_msg = _transfer_payment_to_recevier_msg(blind_box_config, &real_payment);

    // mint nft


    let mint_results: Result<Vec<_>, _> = current_token_ids.clone()
        .iter()
        .map(|token_id| _mint_nft(deps.branch(), env.clone(), info.clone(), token_id.clone(), owner.clone()))
        .collect();

    for mint_res in mint_results? {
        if mint_res.attributes.is_empty() {
            return Err(ContractError::Std(StdError::generic_err("mint nft error")));
        }
    }

    let mut attributes = vec![
        attr("action", "do_mint"),
        attr("token_ids", current_token_ids.clone().join(",").as_str()),
        attr("level_index", level_index.to_string().as_str()),
        attr("price", price.to_string().as_str()),
        attr("paid_amount", paid_amount.to_string().as_str()),
        attr("mint_discount_rate", mint_discount_rate.to_string().as_str()),
    ];
    if let Some(referral_code) = referral_code {
        attributes.push(attr("referral_code", referral_code.as_str()));
    }
    if let Some(inviter) = inviter {
        attributes.push(attr("inviter", inviter.to_string().as_str()));
    }

    Ok(Response::new()
        .add_message(sender_amount_msg)
        .add_attributes(attributes))
}

fn _do_invitee(deps: DepsMut, user: Addr, referral_code: Option<String>, inviter: Option<Addr>, mint_discount_rate: Uint128) -> Result<(), ContractError> {
    if inviter.is_some() {
        let mut user_info = read_user_info(deps.storage, &user);
        user_info.inviter = inviter.unwrap_or(Addr::unchecked(""));
        user_info.inviter_referral_code = referral_code.unwrap_or(String::from(""));
        user_info.last_mint_discount_rate = mint_discount_rate.u128();
        store_user_info(deps.storage, &user, &user_info)?;
    }
    Ok(())
}

fn _do_inviter(deps: DepsMut, env: Env, user: Addr,
               token_ids: Vec<String>, level_index: u8, referral_code_opt: Option<String>)
               -> Result<(Uint128, Uint128, Option<Addr>), ContractError> {
    let mint_num = Uint128::from(token_ids.len() as u128);
    let mint_info = cal_mint_info(deps.as_ref(), level_index.clone(), mint_num, referral_code_opt)?;
    let paid_amount = mint_info.paid_amount;
    let mint_discount_rate = mint_info.mint_discount_rate;
    let mut inviter: Option<Addr> = mint_info.inviter;
    let current_inviter_reward_level = mint_info.next_inviter_reward_level;
    // check referral_code and current_inviter_reward_level not none
    if inviter.is_some() && current_inviter_reward_level.is_some() {
        let inviter_addr = inviter.unwrap();
        let current_inviter_reward_level_index = current_inviter_reward_level.unwrap();
        // let next_inviter_reward_level_index = mint_info.next_inviter_reward_level.unwrap();

        let mut inviter_info = read_user_info(deps.storage, &inviter_addr);

        // check reward config exists
        let referral_reward_config = read_referral_reward_config(deps.storage)?;
        let referral_level_config = referral_reward_config.referral_level_config;


        let inviter_current_referral_level_config = referral_level_config.get(&current_inviter_reward_level_index).unwrap();

        // inviter_info.current_reward_level = next_inviter_reward_level_index.clone();
        inviter_info.current_reward_level = current_inviter_reward_level_index.clone();

        // calculate inviter reward amount
        let inviter_reward_amount = paid_amount.multiply_ratio(inviter_current_referral_level_config.inviter_reward_rate, BASE_RATE);
        inviter_info.user_reward_total_base_amount += inviter_reward_amount.clone().u128();

        //save total
        let mut referral_reward_total_state = read_referral_reward_total_state(deps.storage);

        referral_reward_total_state.referral_reward_total_base_amount += inviter_reward_amount.clone().u128();

        // update inviter referral level map

        // add sell amount
        let amount = mint_info.price.mul(mint_num);
        inviter_info.user_referral_total_amount += amount.clone().u128();
        // current level reward box
        let box_mul = inviter_info.user_referral_total_amount.clone() / inviter_current_referral_level_config.clone().reward_box_config.recommended_quantity;

        println!("inviter_info.user_referral_total_amount: {}", inviter_info.user_referral_total_amount);
        println!("inviter_current_referral_level_config.reward_box_config.recommended_quantity: {}", inviter_current_referral_level_config.clone().reward_box_config.recommended_quantity);

        println!("box_mul: {}", box_mul);
        if box_mul > 0 {
            // inviter reward box recalculation
            // sub total state referral_reward_box_total

            for (referral_level, quantity) in &inviter_info.user_reward_box {
                if *quantity > 0u32 {
                    let referral_reward_box_total = referral_reward_total_state.referral_reward_box_total.get(referral_level).unwrap_or(&0u32);
                    if referral_reward_box_total > quantity {
                        referral_reward_total_state.referral_reward_box_total.insert(referral_level.clone(),
                                                                                     referral_reward_box_total - quantity.clone());
                    } else {
                        referral_reward_total_state.referral_reward_box_total.insert(referral_level.clone(), 0u32);
                    }
                }
            }


            //Replacement quantity
            inviter_info.user_reward_box.clear();

            for (referral_level, quantity) in &inviter_current_referral_level_config.reward_box_config.reward_box {
                let add_quantity = quantity.clone() * (box_mul as u32);
                inviter_info.user_reward_box.insert(referral_level.clone(), add_quantity);
                let referral_reward_box_total = referral_reward_total_state.referral_reward_box_total.get(referral_level).unwrap_or(&0u32);
                referral_reward_total_state.referral_reward_box_total.insert(referral_level.clone(), referral_reward_box_total + add_quantity);
            }
        }

        let invitee_exists = check_invitee_existence(deps.as_ref().storage, &inviter_addr, &user)?;
        //Inviter +1
        if !invitee_exists {
            inviter_info.invitee_count += 1;
            let user_referral_level_count = inviter_info.user_referral_level_count.get(&current_inviter_reward_level_index).unwrap_or(&0u32).clone();
            inviter_info.user_referral_level_count.insert(current_inviter_reward_level_index, user_referral_level_count + 1u32);
        }

        // save inviter info
        store_user_info(deps.storage, &inviter_addr, &inviter_info)?;

        // save inviter details
        let inviter_referral_record = InviterReferralRecord {
            invitee: user.clone(),
            token_ids: token_ids.clone(),
            mint_time: env.block.time.seconds(),
            reward_level: current_inviter_reward_level_index.clone(),
            invitee_index: inviter_info.invitee_count,
            mint_box_level_index: level_index,
            mint_price: amount.u128(),
            mint_pay_amount: paid_amount.u128(),
            reward_to_inviter_base_amount: mint_discount_rate.u128(),
        };


        store_inviter_record_elem(deps.storage, &inviter_addr, &user, &inviter_referral_record)?;


        store_referral_reward_total_state(deps.storage, &referral_reward_total_state)?;

        inviter = Some(inviter_addr);
    }

    Ok((paid_amount, mint_discount_rate, inviter))
}

fn _mint_nft(deps: DepsMut, env: Env, info: MessageInfo, current_token_id: String, owner: String) -> Result<Response, ContractError> {
    let cw721_msg = cw721_base::ExecuteMsg::Mint {
        token_id: current_token_id,
        owner,
        token_uri: None,
        extension: None,
    };

    let mint_info = MessageInfo {
        sender: env.clone().contract.address,
        funds: info.funds,
    };

    cw721_base::entry::execute(deps, env, mint_info, cw721_msg)
}

fn _transfer_payment_to_recevier_msg(blind_box_config: BlindBoxConfig, payment: &Coin) -> CosmosMsg {
    let sender_amount_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: blind_box_config.clone().receiver_price_addr.to_string(),
        amount: vec![
            Coin {
                denom: blind_box_config.clone().price_token,
                amount: payment.amount,
            }],
    });
    sender_amount_msg
}

pub fn update_blind_box_config(deps: DepsMut, info: MessageInfo,
                               nft_base_url: Option<String>,
                               nft_uri_suffix: Option<String>,
                               gov: Option<String>,
                               price_token: Option<String>,
                               token_id_prefix: Option<String>,
                               start_mint_time: Option<u64>,
                               receiver_price_addr: Option<Addr>,
                               end_mint_time: Option<u64>,
                               can_transfer_time: Option<u64>,
) -> Result<Response, ContractError> {
    let mut blind_box_config = read_blind_box_config(deps.storage)?;


    if info.sender.ne(&blind_box_config.gov) {
        return Err(ContractError::Std(StdError::generic_err("not gov")));
    }
    let mut attrs = vec![];
    attrs.push(attr("action", "update_blind_box_config"));

    if let Some(_nft_base_url) = nft_base_url {
        blind_box_config.nft_base_url = _nft_base_url.clone();
        attrs.push(attr("nft_base_url", _nft_base_url));
    }

    if let Some(_nft_uri_suffix) = nft_uri_suffix {
        blind_box_config.nft_uri_suffix = _nft_uri_suffix.clone();
        attrs.push(attr("nft_uri_suffix", _nft_uri_suffix));
    }


    if let Some(gov) = gov {
        blind_box_config.gov = Addr::unchecked(gov.clone());
        attrs.push(attr("gov", gov));
    }

    if let Some(price_token) = price_token {
        blind_box_config.price_token = price_token.clone();
        attrs.push(attr("price_token", price_token));
    }

    if let Some(token_id_prefix) = token_id_prefix {
        blind_box_config.token_id_prefix = token_id_prefix.clone();
        attrs.push(attr("token_id_prefix", token_id_prefix));
    }

    if let Some(start_mint_time) = start_mint_time {
        blind_box_config.start_mint_time = start_mint_time.clone();
        attrs.push(attr("start_mint_time", start_mint_time.to_string()));
    }
    if let Some(end_mint_time) = end_mint_time {
        blind_box_config.end_mint_time = end_mint_time.clone();
        attrs.push(attr("end_mint_time", end_mint_time.to_string()));
    }

    if let Some(receiver_price_addr) = receiver_price_addr {
        blind_box_config.receiver_price_addr = receiver_price_addr.clone();
        attrs.push(attr("receiver_price_addr", receiver_price_addr.to_string()));
    }

    if let Some(can_transfer_time) = can_transfer_time {
        blind_box_config.can_transfer_time = can_transfer_time.clone();
        attrs.push(attr("can_transfer_time", can_transfer_time.to_string()));
    }

    store_blind_box_config(deps.storage, &blind_box_config)?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn update_config_level(deps: DepsMut, info: MessageInfo, index: u8,
                           price: Option<u128>,
                           mint_total_count: Option<u128>) -> Result<Response, ContractError> {
    let mut blind_box_config = read_blind_box_config(deps.storage)?;

    if info.sender.ne(&blind_box_config.gov) {
        return Err(ContractError::Std(StdError::generic_err("not gov")));
    }

    let level_index = index as usize;
    let level_count = blind_box_config.level_infos.len();
    // add
    if level_count == 0 || level_index > (level_count - 1) {
        if price.is_none() || mint_total_count.is_none() {
            return Err(ContractError::Std(StdError::generic_err("price or mint_total_count is none")));
        } else {
            blind_box_config.level_infos.push(BlindBoxLevel {
                level_index: index.clone(),
                price: price.unwrap(),
                mint_total_count: mint_total_count.unwrap(),
                minted_count: 0,
                received_total_amount: 0,
            });
        }
    } else {
        let level_info = blind_box_config.level_infos[level_index].clone();
        blind_box_config.level_infos[level_index] = BlindBoxLevel {
            level_index: level_info.level_index,
            price: price.unwrap_or(level_info.price),
            mint_total_count: mint_total_count.unwrap_or(level_info.mint_total_count),
            minted_count: level_info.minted_count,
            received_total_amount: level_info.received_total_amount,
        };
    }

    store_blind_box_config(deps.storage, &blind_box_config)?;
    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "update_config_leve"),
            attr("index", index.to_string()),
            attr("price", price.unwrap_or(0u128).to_string()),
            attr("mint_total_count", mint_total_count.unwrap_or(0u128).to_string()),
        ]))
}


pub fn update_reward_token_config(deps: DepsMut, info: MessageInfo,
                                  reward_token_type: String, reward_token: String, conversion_ratio: u128)
                                  -> Result<Response, ContractError> {
    let blind_box_config = read_blind_box_config(deps.storage)?;

    if info.sender.ne(&blind_box_config.gov) {
        return Err(ContractError::Std(StdError::generic_err("not gov")));
    }

    let mut referral_reward_config = read_referral_reward_config(deps.storage)?;
    let reward_token_config = referral_reward_config.reward_token_config.get_mut(&reward_token_type).unwrap();
    let mut attrs = vec![];
    attrs.push(attr("action", "update_reward_token_config"));
    attrs.push(attr("reward_token_type", reward_token_type.clone()));

    reward_token_config.reward_token = reward_token.clone();
    attrs.push(attr("reward_token", reward_token));

    reward_token_config.conversion_ratio = conversion_ratio.clone();
    attrs.push(attr("conversion_ratio", conversion_ratio.to_string()));

    // referral_reward_config.reward_token_config.insert(reward_token_type.clone(), reward_token_config.clone());
    store_referral_reward_config(deps.storage, &referral_reward_config)?;

    Ok(Response::new()
        .add_attributes(attrs))
}

pub fn update_referral_level_config(deps: DepsMut, info: MessageInfo, referral_level_config_msg: ReferralLevelConfigMsg) -> Result<Response, ContractError> {
    let blind_box_config = read_blind_box_config(deps.storage)?;

    if info.sender.ne(&blind_box_config.gov) {
        return Err(ContractError::Std(StdError::generic_err("not gov")));
    }
    let referral_level_index = referral_level_config_msg.referral_level.clone();

    let mut attrs = vec![];
    attrs.push(attr("action", "update_referral_level_config"));
    attrs.push(attr("referral_level", referral_level_index.clone().to_string()));

    let mut referral_reward_config = read_referral_reward_config(deps.storage)?;

    let mut referral_level_config = referral_reward_config.referral_level_config
        .get_mut(&referral_level_index).unwrap();

    if let Some(min_referral_total_amount) = referral_level_config_msg.min_referral_total_amount {
        referral_level_config.min_referral_total_amount = min_referral_total_amount.clone();
        attrs.push(attr("min_referral_total_amount", min_referral_total_amount.to_string()));
    }

    if let Some(max_referral_total_amount) = referral_level_config_msg.max_referral_total_amount {
        referral_level_config.max_referral_total_amount = max_referral_total_amount.clone();
        attrs.push(attr("max_referral_total_amount", max_referral_total_amount.to_string()));
    }

    if let Some(inviter_reward_rate) = referral_level_config_msg.inviter_reward_rate {
        referral_level_config.inviter_reward_rate = inviter_reward_rate.clone();
        attrs.push(attr("inviter_reward_rate", inviter_reward_rate.to_string()));
    }

    if let Some(invitee_discount_rate) = referral_level_config_msg.invitee_discount_rate {
        referral_level_config.invitee_discount_rate = invitee_discount_rate.clone();
        attrs.push(attr("invitee_discount_rate", invitee_discount_rate.to_string()));
    }

    // referral_reward_config.referral_level_config.insert(referral_level_index.clone(), referral_level_config.clone());
    store_referral_reward_config(deps.storage, &referral_reward_config)?;


    Ok(Response::new()
        .add_attributes(attrs))
}


pub fn update_referral_level_box_config(deps: DepsMut, info: MessageInfo, level_reward_box_config_msg: ReferralLevelRewardBoxConfigMsg) -> Result<Response, ContractError> {
    let blind_box_config = read_blind_box_config(deps.storage)?;

    if info.sender.ne(&blind_box_config.gov) {
        return Err(ContractError::Std(StdError::generic_err("not gov")));
    }
    let referral_level_index = level_reward_box_config_msg.referral_level.clone();

    let mut attrs = vec![];
    attrs.push(attr("action", "update_referral_level_box_config"));
    attrs.push(attr("referral_level", referral_level_index.clone().to_string()));

    let mut referral_reward_config = read_referral_reward_config(deps.storage)?;

    let mut referral_level_config = referral_reward_config.referral_level_config
        .get_mut(&referral_level_index).unwrap();

    if let Some(recommended_quantity) = level_reward_box_config_msg.recommended_quantity {
        referral_level_config.reward_box_config.recommended_quantity = recommended_quantity.clone();
        attrs.push(attr("recommended_quantity", recommended_quantity.to_string()));
    }

    if let Some(reward_box) = level_reward_box_config_msg.reward_box {
        for (key, value) in reward_box.iter() {
            referral_level_config.reward_box_config.reward_box.insert(key.clone(), value.clone());
            attrs.push(attr(format!("reward_box_{}", key.clone()), value.clone().to_string()));
        }
    }

    // referral_reward_config.referral_level_config.insert(referral_level_index.clone(), referral_level_config.clone());
    store_referral_reward_config(deps.storage, &referral_reward_config)?;

    Ok(Response::new()
        .add_attributes(attrs))
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use cosmwasm_std::{Addr, Uint128};
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use crate::handler::_do_inviter;
    use crate::state::{BlindBoxConfig, BlindBoxLevel, read_referral_reward_total_state, read_user_info, ReferralLevelConfig, ReferralLevelRewardBoxConfig, ReferralRewardConfig, store_blind_box_config, store_referral_reward_config, store_user_info, store_user_referral_code, UserInfo};


    #[test]
    fn test_do_inviter_success() {

        // Setup test environment
        let mut deps = mock_dependencies();
        let level_index = 0u8;
        // let mint_num = Uint128::from(1u128);
        let referral_code = Some("test_referral_code".to_string());
        // Populate storage with necessary data
        let config = BlindBoxConfig {
            nft_base_url: "".to_string(),
            nft_uri_suffix: "".to_string(),
            gov: Addr::unchecked(""),
            price_token: "".to_string(),
            token_id_prefix: "".to_string(),
            token_id_index: 0,
            start_mint_time: 0,
            end_mint_time: 0,
            level_infos: vec![BlindBoxLevel { level_index, price: 20000000, mint_total_count: 0, minted_count: 0, received_total_amount: 0 }],
            receiver_price_addr: Addr::unchecked(""),
            can_transfer_time: 0,
        };
        store_blind_box_config(&mut deps.storage, &config).unwrap();
        let referral_reward_config = ReferralRewardConfig {
            reward_token_config: Default::default(),
            referral_level_config: {
                let mut config = HashMap::new();
                let mut reward_box_map_0 = HashMap::new();

                reward_box_map_0.insert(0u8, 1u32);
                config.insert(
                    0u8,
                    ReferralLevelConfig {
                        min_referral_total_amount: 0,
                        max_referral_total_amount: 10000000000,
                        invitee_discount_rate: 100000,
                        inviter_reward_rate: 50000,
                        reward_box_config: ReferralLevelRewardBoxConfig { recommended_quantity: 500000000, reward_box: reward_box_map_0.clone() },
                    },
                );
                let mut reward_box_map_1 = HashMap::new();
                reward_box_map_1.insert(1u8, 1u32);
                config.insert(
                    1u8,
                    ReferralLevelConfig {
                        min_referral_total_amount: 10000000001,
                        max_referral_total_amount: 50000000000,
                        invitee_discount_rate: 70000,
                        inviter_reward_rate: 50000,
                        reward_box_config: ReferralLevelRewardBoxConfig { recommended_quantity: 4000000000, reward_box: reward_box_map_1.clone() },
                    },
                );
                config
            },
        };
        store_referral_reward_config(&mut deps.storage, &referral_reward_config).unwrap();
        let inviter_addr = Addr::unchecked("inviter");
        let inviter_info = UserInfo {
            referral_code: "test_referral_code".to_string(),
            inviter_referral_code: "".to_string(),
            inviter: Addr::unchecked("inviter"),
            invitee_count: 0,
            last_mint_discount_rate: 0,
            current_reward_level: 0,
            user_reward_token_type: "".to_string(),
            user_reward_total_base_amount: 0,
            user_referral_total_amount: 9990000000,
            user_referral_level_count: Default::default(),
            user_reward_box: Default::default(),
        };
        store_user_info(&mut deps.storage, &inviter_addr, &inviter_info).unwrap();
        store_user_referral_code(&mut deps.storage, "test_referral_code".to_string(), &inviter_addr).unwrap();

        let user = Addr::unchecked("user1");
        let token_ids = vec!["token1".to_string(), "token2".to_string()];
        let level_index = 0u8;
        let result = _do_inviter(
            deps.as_mut(),
            mock_env(),
            user.clone(),
            token_ids.clone(),
            level_index,
            referral_code.clone(),
        );
        println!("{:?}", result);
        assert!(result.is_ok());
        let (payment, discount, inviter) = result.unwrap();
        assert_eq!(payment, Uint128::from(36000000u128));
        assert_eq!(discount, Uint128::from(100000u128));
        assert!(inviter.is_some());
        let inviter = read_user_info(deps.as_ref().storage, &Addr::unchecked("inviter"));
        println!("{:?}", inviter);
        let referral_reward_total_state = read_referral_reward_total_state(deps.as_ref().storage);
        println!("{:?}", referral_reward_total_state);
    }
}