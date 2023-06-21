use cosmwasm_std::{Addr, attr, DepsMut, Env, MessageInfo, Response, StdError, Uint128};
use cw721_base::ContractError;
use crate::state::{BlindBoxInfo, BlindBoxLevel, read_blind_box_config, store_blind_box_config, store_blind_box_info};

pub fn do_mint(deps: DepsMut, env: Env, info: MessageInfo, level_index: u8, recipient: Option<String>) -> Result<Response, ContractError> {
    let _level_index = level_index as usize;
    let mut blind_box_config = read_blind_box_config(deps.storage)?;
    let mut level_infos = blind_box_config.clone().level_infos;
    let price = level_infos[_level_index].price;
    level_infos[_level_index].minted_count = level_infos[_level_index].minted_count + 1;

    if level_infos[_level_index].minted_count > level_infos[_level_index].mint_total_count {
        return Err(ContractError::Std(StdError::generic_err("minted_count > mint_total_count")));
    }

    let payment = info.funds.iter()
        .find(|x| x.denom.eq(&blind_box_config.clone().price_token))
        .ok_or_else(|| StdError::generic_err("denom not found"))?;

    let amount = payment.amount;

    if amount.ne(&Uint128::from(price)) {
        return Err(ContractError::Std(StdError::generic_err("amount not eq price")));
    }


    let token_id_index = blind_box_config.clone().token_id_index + 1;
    let token_id_prefix = blind_box_config.clone().token_id_prefix;
    let current_token_id = format!("{}{}", token_id_prefix, token_id_index.clone());

    store_blind_box_info(deps.storage, current_token_id.clone(), &BlindBoxInfo {
        level_index,
        price,
        block_number: env.block.height,
    })?;

    blind_box_config.token_id_index = token_id_index;

    store_blind_box_config(deps.storage, &blind_box_config)?;


    let owner = recipient.unwrap_or(info.sender.to_string());

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

pub fn update_blind_box_config(deps: DepsMut, info: MessageInfo,
                               nft_base_url: Option<String>,
                               nft_uri_suffix: Option<String>,
                               gov: Option<String>,
                               price_token: Option<String>,
                               token_id_prefix: Option<String>,
                               start_mint_time: Option<u64>,
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

    store_blind_box_config(deps.storage, &blind_box_config)?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn update_config_leve(deps: DepsMut, info: MessageInfo, index: u32,
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
                price: price.unwrap(),
                mint_total_count: mint_total_count.unwrap(),
                minted_count: 0,
            });
        }
    } else {
        let level_info = blind_box_config.level_infos[level_index].clone();
        blind_box_config.level_infos[level_index] = BlindBoxLevel {
            price: price.unwrap_or(level_info.price),
            mint_total_count: mint_total_count.unwrap_or(level_info.mint_total_count),
            minted_count: level_info.minted_count,
        };
    }

    store_blind_box_config(deps.storage, &blind_box_config)?;
    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "update_config_leve"),
            attr("index", index.to_string()),
            attr("price", price.unwrap_or(0).to_string()),
            attr("mint_total_count", mint_total_count.unwrap_or(0).to_string()),
        ]))
}

