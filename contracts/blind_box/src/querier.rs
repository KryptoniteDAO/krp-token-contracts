use std::collections::HashMap;
use std::ops::Mul;
use cosmwasm_std::{Addr, Deps, Env, from_binary, StdError, StdResult, Uint128};
use crate::helper::{BASE_RATE, is_empty_address};
use crate::msg::{BlindBoxConfigLevelResponse, BlindBoxConfigResponse, BlindBoxInfoResponse, CalMintInfoResponse, CheckReferralCodeResponse, InviterReferralRecordResponse, ReferralLevelConfigResponse, ReferralLevelRewardBoxConfigResponse, ReferralRewardConfigResponse, ReferralRewardTokenConfigResponse, UserInfoResponse};
use crate::state::{read_blind_box_config, read_blind_box_info, read_inviter_records, read_referral_reward_config, read_referral_reward_total_state, read_user_info, read_user_referral_code, ReferralLevelConfig};

pub fn query_blind_box_config(deps: Deps) -> StdResult<BlindBoxConfigResponse> {
    let blind_box_config = read_blind_box_config(deps.storage)?;
    let mut level_infos = vec![];
    for level_info_db in blind_box_config.level_infos {
        let level_info = level_info_db.clone();
        level_infos.push(BlindBoxConfigLevelResponse {
            level_index: level_info.level_index,
            price: level_info.price,
            mint_total_count: level_info.mint_total_count,
            minted_count: level_info.minted_count,
            received_total_amount: level_info.received_total_amount,
        });
    }
    Ok(BlindBoxConfigResponse {
        nft_base_url: blind_box_config.nft_base_url,
        nft_uri_suffix: blind_box_config.nft_uri_suffix,
        gov: blind_box_config.gov.to_string(),
        price_token: blind_box_config.price_token,
        token_id_prefix: blind_box_config.token_id_prefix,
        token_id_index: blind_box_config.token_id_index,
        start_mint_time: blind_box_config.start_mint_time,
        level_infos,
        receiver_price_addr: blind_box_config.receiver_price_addr,
        can_transfer_time: blind_box_config.can_transfer_time,
        inviter_reward_box_contract: blind_box_config.inviter_reward_box_contract,
    })
}

pub fn query_blind_box_config_level(deps: Deps, index: u8) -> StdResult<BlindBoxConfigLevelResponse> {
    let blind_box_config = read_blind_box_config(deps.storage)?;
    let _index = index as usize;
    let level_infos = blind_box_config.level_infos[_index].clone();
    Ok(BlindBoxConfigLevelResponse {
        level_index: level_infos.level_index,
        price: level_infos.price,
        mint_total_count: level_infos.mint_total_count,
        minted_count: level_infos.minted_count,
        received_total_amount: level_infos.received_total_amount,
    })
}

pub fn query_blind_box_info(deps: Deps, token_id: String) -> StdResult<BlindBoxInfoResponse> {
    let blind_box_info = read_blind_box_info(deps.storage, token_id);
    Ok(BlindBoxInfoResponse {
        level_index: blind_box_info.level_index,
        price: blind_box_info.price,
        block_number: blind_box_info.block_number,
        is_random_box: blind_box_info.is_random_box,
        is_reward_box: blind_box_info.is_reward_box,
    })
}

pub fn query_tokens(deps: Deps, env: Env,
                    owner: String,
                    start_after: Option<String>,
                    limit: Option<u32>) -> StdResult<cw721::TokensResponse> {
    let cw721_msg = cw721_base::QueryMsg::Tokens { owner, start_after, limit };
    let tokens_bin = cw721_base::entry::query(deps, env, cw721_msg)?;
    let tokens: cw721::TokensResponse = from_binary(&tokens_bin)?;
    Ok(tokens)
}

pub fn query_nft_info(deps: Deps, env: Env, token_id: String) -> StdResult<cw721::NftInfoResponse<cw721_base::Extension>> {
    let cw721_msg = cw721_base::QueryMsg::NftInfo { token_id: token_id.clone() };
    let nft_info_bin = cw721_base::entry::query(deps, env, cw721_msg)?;
    let nft_info: cw721::NftInfoResponse<cw721_base::Extension> = from_binary(&nft_info_bin)?;
    let blind_box_config = read_blind_box_config(deps.storage)?;
    let box_info = read_blind_box_info(deps.storage, token_id.clone());
    let token_uri = format!("{}{}{}", blind_box_config.nft_base_url, box_info.level_index, blind_box_config.nft_uri_suffix);
    Ok(cw721::NftInfoResponse {
        token_uri: Option::from(token_uri),
        extension: nft_info.extension,
    })
}

pub fn query_all_nft_info(deps: Deps, env: Env, token_id: String, include_expired: Option<bool>) -> StdResult<cw721::AllNftInfoResponse<cw721_base::Extension>> {
    let cw721_msg = cw721_base::QueryMsg::AllNftInfo { token_id: token_id.clone(), include_expired };
    let all_nft_info_bin = cw721_base::entry::query(deps, env, cw721_msg)?;
    let all_nft_info: cw721::AllNftInfoResponse<cw721_base::Extension> = from_binary(&all_nft_info_bin)?;
    let blind_box_config = read_blind_box_config(deps.storage)?;
    let box_info = read_blind_box_info(deps.storage, token_id.clone());
    let token_uri = format!("{}{}{}", blind_box_config.nft_base_url, box_info.level_index, blind_box_config.nft_uri_suffix);
    Ok(cw721::AllNftInfoResponse {
        access: cw721::OwnerOfResponse { owner: all_nft_info.access.owner, approvals: all_nft_info.access.approvals },

        info: cw721::NftInfoResponse { token_uri: Option::from(token_uri), extension: all_nft_info.info.extension },
    })
}

pub fn query_all_referral_reward_config(deps: Deps) -> StdResult<ReferralRewardConfigResponse> {
    let referral_reward_config = read_referral_reward_config(deps.storage)?;
    let reward_token_config = referral_reward_config.reward_token_config;
    let mut reward_token_config_res = HashMap::new();
    for (key, value_ref) in reward_token_config.iter() {
        let value = value_ref.clone();
        reward_token_config_res.insert(key.clone(), ReferralRewardTokenConfigResponse {
            reward_token: value.reward_token.clone(),
            conversion_ratio: value.conversion_ratio.clone(),
        });
    }
    let referral_level_config = referral_reward_config.referral_level_config;
    let mut referral_level_config_res = HashMap::new();
    for (key, value_ref) in referral_level_config.iter() {
        let value = value_ref.clone();
        referral_level_config_res.insert(key.clone(), ReferralLevelConfigResponse {
            min_referral_total_amount: value.min_referral_total_amount,
            max_referral_total_amount: value.max_referral_total_amount,
            inviter_reward_rate: value.inviter_reward_rate,
            invitee_discount_rate: value.invitee_discount_rate,
            reward_box_config: ReferralLevelRewardBoxConfigResponse {
                recommended_quantity: value.reward_box_config.recommended_quantity,
                reward_box: value.reward_box_config.reward_box.clone(),
            },
        });
    }

    let referral_reward_total_state = read_referral_reward_total_state(deps.storage);
    Ok(ReferralRewardConfigResponse {
        reward_token_config: reward_token_config_res,
        referral_level_config: referral_level_config_res,
        referral_reward_total_base_amount: referral_reward_total_state.referral_reward_total_base_amount,
        referral_reward_box_total: referral_reward_total_state.referral_reward_box_total,
    })
}

pub fn query_inviter_records(deps: Deps, inviter: &Addr,
                             start_after: Option<Addr>,
                             limit: Option<u32>) -> StdResult<Vec<InviterReferralRecordResponse>> {
    let inviter_records = read_inviter_records(deps.storage, &inviter.clone(), start_after, limit)?;

    let mut res = vec![];
    for record_ref in inviter_records.iter() {
        let record = record_ref.clone();
        res.push(InviterReferralRecordResponse {
            invitee: record.invitee,
            token_ids: record.token_ids,
            mint_time: record.mint_time,
            reward_level: record.reward_level,
            invitee_index: record.invitee_index,
            mint_box_level_index: record.mint_box_level_index,
            mint_price: record.mint_price,
            mint_pay_amount: record.mint_pay_amount,
            reward_to_inviter_base_amount: record.reward_to_inviter_base_amount,
        })
    }
    // let res_len = res.len();
    // let show_msg = format!("res_len:{},inviter:{},start_after_msg：{:?},limit_msg：{:?}", res_len,inviter.to_string(),start_after_msg,limit_msg);
    // println!("{}",show_msg);
    // if 1<2 {
    //     return Err(StdError::generic_err(show_msg));
    // }
    Ok(res)
}

pub fn cal_mint_info(deps: Deps, level_index: u8, mint_num: Uint128, referral_code: Option<String>) -> StdResult<CalMintInfoResponse> {
    let price = Uint128::from(read_blind_box_config(deps.storage)?.level_infos[level_index as usize].clone().price);
    let mut paid_amount = price.mul(mint_num);
    let mut mint_discount_rate = Uint128::zero();
    let mut inviter: Option<Addr> = None;
    let mut current_inviter_reward_level = None;
    let mut next_inviter_reward_level = None;
    if referral_code.is_some() {
        let referral_code = referral_code.unwrap();
        // check referral_code exists
        let inviter_addr = read_user_referral_code(deps.storage, referral_code);
        if !is_empty_address(inviter_addr.as_str()) {
            let inviter_info = read_user_info(deps.storage, &inviter_addr);
            if !inviter_info.referral_code.is_empty() {
                let referral_reward_config = read_referral_reward_config(deps.storage)?;
                let referral_level_config = referral_reward_config.referral_level_config;

                let find_current_referral_level_configs = referral_level_config.clone()
                    .iter()
                    .filter(|(_, config)|
                        config.min_referral_total_amount <= inviter_info.user_referral_total_amount
                            && config.max_referral_total_amount >= inviter_info.user_referral_total_amount
                    )
                    .map(|(k, v)| (*k, v.clone()))
                    .collect::<HashMap<u8, ReferralLevelConfig>>();
                let find_level_config_len = find_current_referral_level_configs.len() as u8;


                if find_level_config_len == 1u8 {
                    let (referral_level_index, level_config) = find_current_referral_level_configs.iter().next().unwrap();

                    current_inviter_reward_level = Option::from(referral_level_index.clone());
                    next_inviter_reward_level = current_inviter_reward_level.clone();

                    // calculate next inviter reward level
                    let minted_referral_total_amount = inviter_info.user_referral_total_amount + price.mul(mint_num).u128();

                    if minted_referral_total_amount > level_config.max_referral_total_amount {
                        let find_next_referral_level_configs = referral_level_config.clone()
                            .iter()
                            .filter(|(_, config)|
                                config.min_referral_total_amount <= minted_referral_total_amount
                                    && config.max_referral_total_amount >= minted_referral_total_amount
                            )
                            .map(|(k, v)| (*k, v.clone()))
                            .collect::<HashMap<u8, ReferralLevelConfig>>();
                        if !find_next_referral_level_configs.is_empty() && find_next_referral_level_configs.len() == 1 {
                            let (referral_level_index, _) = find_next_referral_level_configs.iter().next().unwrap();
                            next_inviter_reward_level = Option::from(referral_level_index.clone());
                        }
                    }

                    mint_discount_rate = Uint128::from(level_config.invitee_discount_rate.clone());
                    paid_amount = paid_amount.multiply_ratio(BASE_RATE - mint_discount_rate.u128(), BASE_RATE);
                } else {
                    return Err(StdError::generic_err(format!("referral level config not found,config length:{}", find_level_config_len.to_string())));
                }
            }
            inviter = Option::from(inviter_addr);
        } else {
            return Err(StdError::generic_err("referral code not found"));
        }
    }

    Ok(CalMintInfoResponse {
        price,
        paid_amount,
        mint_discount_rate,
        current_inviter_reward_level,
        next_inviter_reward_level,
        inviter,
    })
}

pub fn check_referral_code(deps: Deps, referral_code: String) -> StdResult<CheckReferralCodeResponse> {
    let inviter = read_user_referral_code(deps.storage, referral_code);
    let exists = !is_empty_address(inviter.clone().as_str());
    Ok(CheckReferralCodeResponse {
        exists,
        user: inviter,
    })
}

pub fn get_user_info(deps: Deps, user: Addr) -> StdResult<UserInfoResponse> {
    let user_info = read_user_info(deps.storage, &user);
    Ok(UserInfoResponse {
        referral_code: user_info.referral_code,
        inviter_referral_code: user_info.inviter_referral_code,
        inviter: user_info.inviter,
        invitee_count: user_info.invitee_count,
        last_mint_discount_rate: user_info.last_mint_discount_rate,
        current_reward_level: user_info.current_reward_level,
        user_reward_token_type: user_info.user_reward_token_type,
        user_reward_total_base_amount: user_info.user_reward_total_base_amount,
        user_referral_total_amount: user_info.user_referral_total_amount,
        user_referral_level_count: user_info.user_referral_level_count,
        user_reward_box: user_info.user_reward_box,
    })
}

#[cfg(test)]
mod tests {
    use crate::querier::{cal_mint_info, query_inviter_records};
    use crate::msg::{CalMintInfoResponse};
    use crate::state::{BlindBoxConfig, BlindBoxLevel, check_invitee_existence, InviterReferralRecord, read_inviter_records, ReferralLevelConfig, ReferralLevelRewardBoxConfig, ReferralRewardConfig, store_blind_box_config, store_inviter_record_elem, store_referral_reward_config, store_user_info, store_user_referral_code, UserInfo};
    use cosmwasm_std::{Addr, Uint128};
    use std::collections::HashMap;
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn test_cal_mint_info_positive() {
        // Setup test environment
        let mut deps = mock_dependencies();
        let level_index = 0u8;
        let mint_num = Uint128::from(10u128);
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
            level_infos: vec![BlindBoxLevel { level_index, price: 100, mint_total_count: 0, minted_count: 0, received_total_amount: 0, is_random_box: false }],
            receiver_price_addr: Addr::unchecked(""),
            can_transfer_time: 0,
            inviter_reward_box_contract: Addr::unchecked(""),
        };
        store_blind_box_config(&mut deps.storage, &config).unwrap();
        let referral_reward_config = ReferralRewardConfig {
            reward_token_config: Default::default(),
            referral_level_config: {
                let mut config = HashMap::new();
                config.insert(
                    0u8,
                    ReferralLevelConfig {
                        min_referral_total_amount: 0,
                        max_referral_total_amount: 1000,
                        invitee_discount_rate: 100000,
                        inviter_reward_rate: 50000,
                        reward_box_config: ReferralLevelRewardBoxConfig { recommended_quantity: 0, reward_box: Default::default() },
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
            inviter: Addr::unchecked(""),
            invitee_count: 0,
            last_mint_discount_rate: 0,
            current_reward_level: 0,
            user_reward_token_type: "".to_string(),
            user_reward_total_base_amount: 0,
            user_referral_total_amount: 500,
            user_referral_level_count: Default::default(),
            user_reward_box: Default::default(),
        };
        store_user_info(&mut deps.storage, &inviter_addr, &inviter_info).unwrap();
        store_user_referral_code(&mut deps.storage, "test_referral_code".to_string(), &inviter_addr).unwrap();
        // Call the function and check the result
        let result = cal_mint_info(deps.as_ref(), level_index, mint_num, referral_code).unwrap();

        assert_eq!(
            result,
            CalMintInfoResponse {
                price: Uint128::from(100u128),
                paid_amount: Uint128::from(900u128),
                mint_discount_rate: Uint128::from(100000u128),
                current_inviter_reward_level: Some(0u8),
                next_inviter_reward_level: Some(0u8),
                inviter: Some(inviter_addr),
            }
        );
    }


    #[test]
    fn store_and_read_inviter_records() {
        let mut deps = mock_dependencies();
        let inviter = Addr::unchecked("inviter0000".to_string());
        let invitee = Addr::unchecked("invitee0000".to_string());
        let record = InviterReferralRecord {
            invitee: invitee.clone(),
            token_ids: vec!["token0001".to_string(), "token0002".to_string()],
            mint_time: 1626372000,
            reward_level: 1,
            invitee_index: 1,
            mint_box_level_index: 2,
            mint_price: 1000u128,
            mint_pay_amount: 1000u128,
            reward_to_inviter_base_amount: 1000u128,
        };
        let record2 = InviterReferralRecord {
            invitee: invitee.clone(),
            token_ids: vec!["token0003".to_string()],
            mint_time: 1626372000,
            reward_level: 1,
            invitee_index: 1,
            mint_box_level_index: 2,
            mint_price: 1000u128,
            mint_pay_amount: 1000u128,
            reward_to_inviter_base_amount: 1000u128,
        };
        // store the record
        store_inviter_record_elem(&mut deps.storage, &inviter, &record).unwrap();

        // read the record
        let records = read_inviter_records(&deps.storage, &inviter, None, None).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].invitee, invitee);
        assert_eq!(records[0].token_ids, vec!["token0001".to_string(), "token0002".to_string()]);
        assert_eq!(records[0].mint_time, 1626372000);
        assert_eq!(records[0].reward_level, 1);
        assert_eq!(records[0].invitee_index, 1);
        assert_eq!(records[0].mint_box_level_index, 2);
        assert_eq!(records[0].mint_price, 1000u128);
        assert_eq!(records[0].mint_pay_amount, 1000u128);
        assert_eq!(records[0].reward_to_inviter_base_amount, 1000u128);
        store_inviter_record_elem(&mut deps.storage, &inviter, &record2).unwrap();
        println!("records: {:?}", records);

        let records = read_inviter_records(&deps.storage, &inviter, None, None).unwrap();
        println!("records: {:?}", records);
        let records = query_inviter_records(deps.as_ref(), &inviter, None, None).unwrap();
        for record in records {
            println!("record.token_ids: {:?}", record.token_ids);
        }

        let check_invitee = check_invitee_existence(&deps.storage, &inviter, &invitee);
        println!("check_invitee:{:?}", check_invitee);
    }
}


