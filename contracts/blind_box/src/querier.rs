use cosmwasm_std::{Deps, Env, from_binary, StdResult};
use crate::msg::{BlindBoxConfigLevelResponse, BlindBoxConfigResponse, BlindBoxInfoResponse};
use crate::state::{read_blind_box_config, read_blind_box_info};

pub fn query_blind_box_config(deps: Deps) -> StdResult<BlindBoxConfigResponse> {
    let blind_box_config = read_blind_box_config(deps.storage)?;
    Ok(BlindBoxConfigResponse {
        nft_base_url: blind_box_config.nft_base_url,
        nft_uri_suffix: blind_box_config.nft_uri_suffix,
        gov: blind_box_config.gov.to_string(),
        price_token: blind_box_config.price_token,
        token_id_prefix: blind_box_config.token_id_prefix,
        token_id_index: blind_box_config.token_id_index,
        start_mint_time: blind_box_config.start_mint_time,
        receiver_price_addr: blind_box_config.receiver_price_addr,
    })
}

pub fn query_blind_box_config_level(deps: Deps, index: u32) -> StdResult<BlindBoxConfigLevelResponse> {
    let blind_box_config = read_blind_box_config(deps.storage)?;
    let _index = index as usize;
    Ok(BlindBoxConfigLevelResponse {
        price: blind_box_config.level_infos[_index].price,
        mint_total_count: blind_box_config.level_infos[_index].mint_total_count,
        minted_count: blind_box_config.level_infos[_index].minted_count,
    })
}

pub fn query_blind_box_info(deps: Deps, token_id: String) -> StdResult<BlindBoxInfoResponse> {
    let blind_box_info = read_blind_box_info(deps.storage, token_id);
    Ok(BlindBoxInfoResponse {
        level_index: blind_box_info.level_index,
        price: blind_box_info.price,
        block_number: blind_box_info.block_number,
    })
}


pub fn query_nft_info(deps: Deps, env: Env, token_id: String) -> StdResult<cw721::NftInfoResponse<cw721_base::Extension>> {
    let cw721_msg = cw721_base::QueryMsg::NftInfo { token_id:token_id.clone() };
    let nft_info_bin = cw721_base::entry::query(deps, env, cw721_msg)?;
    let nft_info: cw721::NftInfoResponse<cw721_base::Extension> = from_binary(&nft_info_bin)?;
    let blind_box_config = read_blind_box_config(deps.storage)?;
    let token_uri = format!("{}{}{}", blind_box_config.nft_base_url, token_id, blind_box_config.nft_uri_suffix);
    Ok(cw721::NftInfoResponse {
        token_uri: Option::from(token_uri),
        extension: nft_info.extension,
    })
}

pub fn query_all_nft_info(deps: Deps, env: Env, token_id: String, include_expired: Option<bool>) -> StdResult<cw721::AllNftInfoResponse<cw721_base::Extension>> {
    let cw721_msg = cw721_base::QueryMsg::AllNftInfo { token_id:token_id.clone(), include_expired };
    let all_nft_info_bin = cw721_base::entry::query(deps, env, cw721_msg)?;
    let all_nft_info: cw721::AllNftInfoResponse<cw721_base::Extension> = from_binary(&all_nft_info_bin)?;
    let blind_box_config = read_blind_box_config(deps.storage)?;
    let token_uri = format!("{}{}{}", blind_box_config.nft_base_url, token_id, blind_box_config.nft_uri_suffix);
    Ok(cw721::AllNftInfoResponse {
        access: cw721::OwnerOfResponse { owner: all_nft_info.access.owner, approvals: all_nft_info.access.approvals },

        info: cw721::NftInfoResponse { token_uri: Option::from(token_uri), extension: all_nft_info.info.extension },
    })
}