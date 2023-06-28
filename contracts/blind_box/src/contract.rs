use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Deps, to_binary, Binary};
use cw2::set_contract_version;
use cw721_base::ContractError;
use cw_utils::nonpayable;
use crate::handler::{create_referral_info, do_mint, modify_reward_token_type, update_blind_box_config, update_config_level, update_referral_level_box_config, update_referral_level_config, update_reward_token_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{cal_mint_info, check_referral_code, get_user_info, query_all_nft_info, query_all_referral_reward_config, query_blind_box_config, query_blind_box_config_level, query_blind_box_info, query_inviter_records, query_nft_info};
use crate::state::{BlindBoxConfig, BlindBoxLevel, ReferralRewardConfig, store_blind_box_config, store_referral_reward_config};


// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:blind-box";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let r = nonpayable(&info.clone());
    if r.is_err() {
        return Err(StdError::generic_err(r.err().unwrap().to_string()));
    }

    let gov = if let Some(gov_addr) = msg.gov {
        gov_addr
    } else {
        info.clone().sender
    };

    let mut blind_box_config = BlindBoxConfig {
        nft_base_url: msg.nft_base_url,
        nft_uri_suffix: msg.nft_uri_suffix,
        gov,
        price_token: msg.price_token,
        token_id_prefix: msg.token_id_prefix,
        token_id_index: 0,
        start_mint_time: msg.start_mint_time.unwrap_or(0u64),
        level_infos: vec![],
        receiver_price_addr: msg.receiver_price_addr,
        end_mint_time: msg.end_mint_time.unwrap_or(0u64),
        can_transfer_time: msg.can_transfer_time.unwrap_or(0u64),
    };


    let level_infos = if let Some(_level_infos) = msg.level_infos {
        _level_infos
    } else {
        vec![]
    };
    for (index, level_info) in level_infos.iter().enumerate() {
        blind_box_config.level_infos.push(BlindBoxLevel {
            level_index: index as u8,
            price: level_info.price,
            mint_total_count: level_info.mint_total_count,
            minted_count: 0u128,
            received_total_amount: 0u128,
        });
    }

    let referral_reward_config;
    if msg.referral_reward_config.is_some() {
        let referral_reward_config_msg = msg.referral_reward_config.unwrap();

        referral_reward_config = ReferralRewardConfig {
            reward_token_config: referral_reward_config_msg.reward_token_config.unwrap_or(Default::default()),
            referral_level_config: referral_reward_config_msg.referral_level_config,
        };
    } else {
        referral_reward_config = ReferralRewardConfig {
            reward_token_config: Default::default(),
            referral_level_config: Default::default(),
        };
    }
    store_referral_reward_config(deps.storage, &referral_reward_config)?;

    let cw721_msg = cw721_base::InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: env.contract.address.to_string(),
    };

    cw721_base::entry::instantiate(deps.branch(), env, info.clone(), cw721_msg)?;

    store_blind_box_config(deps.storage, &blind_box_config)?;

// set contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "instantiate"),
        ("owner", info.sender.as_str()),
    ]))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { level_index,mint_num, recipient, referral_code } => {
            do_mint(deps, env, info, level_index,mint_num, recipient, referral_code)
        }
        ExecuteMsg::TransferNft { recipient, token_id } => {
            let blind_box_config = query_blind_box_config(deps.as_ref())?;
            let current_time = env.block.time.seconds();
            if blind_box_config.can_transfer_time > 0u64 && blind_box_config.can_transfer_time > current_time {
                return Err(ContractError::Std(StdError::generic_err(
                    "can not transfer now",
                )));
            }
            let cw721_msg = cw721_base::ExecuteMsg::TransferNft {
                recipient,
                token_id,
            };
            cw721_base::entry::execute(deps, env, info, cw721_msg)
        }
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => {
            let blind_box_config = query_blind_box_config(deps.as_ref())?;
            let current_time = env.block.time.seconds();
            if blind_box_config.can_transfer_time > 0u64 && blind_box_config.can_transfer_time > current_time {
                return Err(ContractError::Std(StdError::generic_err(
                    "can not transfer now",
                )));
            }
            let cw721_msg = cw721_base::ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            };
            cw721_base::entry::execute(deps, env, info, cw721_msg)
        }
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires
        } => {
            let cw721_msg = cw721_base::ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            };
            cw721_base::entry::execute(deps, env, info, cw721_msg)
        }
        ExecuteMsg::Revoke {
            spender,
            token_id,
        } => {
            let cw721_msg = cw721_base::ExecuteMsg::Revoke {
                spender,
                token_id,
            };
            cw721_base::entry::execute(deps, env, info, cw721_msg)
        }
        ExecuteMsg::ApproveAll {
            operator,
            expires,
        } => {
            let cw721_msg = cw721_base::ExecuteMsg::ApproveAll {
                operator,
                expires,
            };
            cw721_base::entry::execute(deps, env, info, cw721_msg)
        }
        ExecuteMsg::RevokeAll {
            operator,
        } => {
            let cw721_msg = cw721_base::ExecuteMsg::RevokeAll {
                operator,
            };
            cw721_base::entry::execute(deps, env, info, cw721_msg)
        }
        ExecuteMsg::Burn { token_id } => {
            let cw721_msg = cw721_base::ExecuteMsg::Burn { token_id };
            cw721_base::entry::execute(deps, env, info, cw721_msg)
        }
        ExecuteMsg::UpdateConfig {
            nft_base_url,
            nft_uri_suffix,
            gov,
            price_token,
            token_id_prefix,
            start_mint_time,
            receiver_price_addr,
            end_mint_time,
            can_transfer_time
        } => {
            update_blind_box_config(deps, info, nft_base_url, nft_uri_suffix, gov, price_token, token_id_prefix,
                                    start_mint_time, receiver_price_addr, end_mint_time,can_transfer_time)
        }
        ExecuteMsg::UpdateConfigLevel {
            index,
            price,
            mint_total_count,
        } => {
            update_config_level(deps, info, index, price, mint_total_count)
        }
        ExecuteMsg::UpdateRewardTokenConfig {
            reward_token_type, reward_token, conversion_ratio
        } => {
            update_reward_token_config(deps, info, reward_token_type, reward_token, conversion_ratio)
        }
        ExecuteMsg::UpdateReferralLevelConfig { referral_level_config_msg } => {
            update_referral_level_config(deps, info, referral_level_config_msg)
        }
        ExecuteMsg::UpdateReferralLevelBoxConfig { level_reward_box_config_msg } => {
            update_referral_level_box_config(deps, info, level_reward_box_config_msg)
        }
        ExecuteMsg::CreateReferralInfo { referral_code, reward_token_type } => {
            create_referral_info(deps, env, info, referral_code, reward_token_type)
        }
        ExecuteMsg::ModifyRewardTokenType { reward_token_type } => {
            modify_reward_token_type(deps, env, info, reward_token_type)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::OwnerOf {
            token_id,
            include_expired
        } => {
            let cw721_msg = cw721_base::QueryMsg::OwnerOf { token_id, include_expired };
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::Approval {
            token_id,
            spender,
            include_expired
        } => {
            let cw721_msg = cw721_base::QueryMsg::Approval { token_id, spender, include_expired };
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::Approvals {
            token_id,
            include_expired
        } => {
            let cw721_msg = cw721_base::QueryMsg::Approvals { token_id, include_expired };
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::Operator {
            owner,
            operator,
            include_expired
        } => {
            let cw721_msg = cw721_base::QueryMsg::Operator { owner, operator, include_expired };
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::AllOperators {
            owner,
            include_expired,
            start_after,
            limit
        } => {
            let cw721_msg = cw721_base::QueryMsg::AllOperators { owner, include_expired, start_after, limit };
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::NumTokens {} => {
            let cw721_msg = cw721_base::QueryMsg::NumTokens {};
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::ContractInfo {} => {
            let cw721_msg = cw721_base::QueryMsg::ContractInfo {};
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::NftInfo {
            token_id
        } => {
            // let cw721_msg = cw721_base::QueryMsg::NftInfo { token_id };
            // cw721_base::entry::query(deps, env, cw721_msg)
            to_binary(&query_nft_info(deps, env, token_id)?)
        }
        QueryMsg::AllNftInfo {
            token_id,
            include_expired
        } => {
            // let cw721_msg = cw721_base::QueryMsg::AllNftInfo { token_id, include_expired };
            // cw721_base::entry::query(deps, env, cw721_msg)
            to_binary(&query_all_nft_info(deps, env, token_id, include_expired)?)
        }
        QueryMsg::Tokens {
            owner,
            start_after,
            limit
        } => {
            let cw721_msg = cw721_base::QueryMsg::Tokens { owner, start_after, limit };
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::AllTokens {
            start_after,
            limit
        } => {
            let cw721_msg = cw721_base::QueryMsg::AllTokens { start_after, limit };
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::Minter {} => {
            let cw721_msg = cw721_base::QueryMsg::Minter {};
            cw721_base::entry::query(deps, env, cw721_msg)
        }
        QueryMsg::QueryBlindBoxConfig {} => {
            to_binary(&query_blind_box_config(deps)?)
        }
        QueryMsg::QueryBlindBoxConfigLevel { index } => {
            to_binary(&query_blind_box_config_level(deps, index)?)
        }
        QueryMsg::QueryBlindBoxInfo { token_id } => {
            to_binary(&query_blind_box_info(deps, token_id)?)
        }
        QueryMsg::QueryAllReferralRewardConfig {} => {
            to_binary(&query_all_referral_reward_config(deps)?)
        }
        QueryMsg::QueryInviterRecords {
            inviter,
            start_after,
            limit, } => {
            to_binary(&query_inviter_records(deps, &inviter, start_after, limit)?)
        }
        QueryMsg::CalMintInfo { level_index,mint_num, referral_code } => {
            to_binary(&cal_mint_info(deps, level_index, mint_num,referral_code)?)
        }
        QueryMsg::CheckReferralCode { referral_code } => {
            to_binary(&check_referral_code(deps, referral_code)?)
        }
        QueryMsg::GetUserInfo { user } => {
            to_binary(&get_user_info(deps, user)?)
        }
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, coin, StdError};
    use crate::msg::BlindBoxLevelMsg;

    const TOKEN_ID_PREFIX: &str = "prefix";
    const NFT_BASE_URL: &str = "https://nft.com/";
    const NFT_URI_SUFFIX: &str = ".json";
    const PRICE_TOKEN: &str = "token";
    const LEVEL_PRICE: u128 = 100;
    const LEVEL_MINT_TOTAL_COUNT: u128 = 10;


    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            name: "Test NFT".to_string(),
            symbol: "Test".to_string(),
            nft_base_url: NFT_BASE_URL.to_string(),
            nft_uri_suffix: NFT_URI_SUFFIX.to_string(),
            price_token: PRICE_TOKEN.to_string(),
            gov: None,
            token_id_prefix: TOKEN_ID_PREFIX.to_string(),
            level_infos: Some(vec![BlindBoxLevelMsg {
                price: LEVEL_PRICE,
                mint_total_count: LEVEL_MINT_TOTAL_COUNT,
            }]),
            start_mint_time: None,
            receiver_price_addr: Addr::unchecked("receiver"),
            end_mint_time: None,
            can_transfer_time: None,
            referral_reward_config: None,
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // test error case when message is payable
        let env = mock_env();
        let info = mock_info("creator", &[coin(100, "token")]);
        let msg = InstantiateMsg {
            name: "Test NFT".to_string(),
            symbol: "Test".to_string(),
            nft_base_url: NFT_BASE_URL.to_string(),
            nft_uri_suffix: NFT_URI_SUFFIX.to_string(),
            price_token: PRICE_TOKEN.to_string(),
            gov: None,
            token_id_prefix: TOKEN_ID_PREFIX.to_string(),
            level_infos: Some(vec![BlindBoxLevelMsg {
                price: LEVEL_PRICE,
                mint_total_count: LEVEL_MINT_TOTAL_COUNT,
            }]),
            start_mint_time: None,
            receiver_price_addr: Addr::unchecked("receiver"),
            end_mint_time: None,
            can_transfer_time: None,
            referral_reward_config: None,
        };
        let res = instantiate(deps.as_mut(), env, info, msg);
        match res {
            Err(StdError::GenericErr { msg, .. }) => {
                assert_eq!(msg, "This message does no accept funds")
            }
            _ => panic!("Unexpected error"),
        }
    }
}