use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Deps, to_binary, Binary};
use cw2::set_contract_version;
use cw721_base::ContractError;
use cw_utils::nonpayable;
use crate::handler::{do_mint, update_blind_box_config, update_config_leve};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_all_nft_info, query_blind_box_config, query_blind_box_config_level, query_blind_box_info, query_nft_info};
use crate::state::{BlindBoxConfig, BlindBoxLevel, store_blind_box_config};


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
    };


    let level_infos = if let Some(_level_infos) = msg.level_infos {
        _level_infos
    } else {
        vec![]
    };
    for level_info in level_infos {
        blind_box_config.level_infos.push(BlindBoxLevel {
            price: level_info.price,
            mint_total_count: level_info.mint_total_count,
            minted_count: 0u128,
        });
    }
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
        ExecuteMsg::Mint { level_index, recipient } => {
            do_mint(deps, env, info, level_index, recipient)
        }
        ExecuteMsg::TransferNft { recipient, token_id } => {
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
        } => {
            update_blind_box_config(deps, info, nft_base_url, nft_uri_suffix, gov, price_token, token_id_prefix,start_mint_time)
        }
        ExecuteMsg::UpdateConfigLevel {
            index,
            price,
            mint_total_count,
        } => {
            update_config_leve(deps, info, index, price, mint_total_count)
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
    use cosmwasm_std::{coin, StdError};
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
            start_mint_time: None
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
            start_mint_time: None
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