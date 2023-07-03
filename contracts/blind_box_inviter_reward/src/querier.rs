use std::ops::Sub;
use cosmwasm_std::{Addr, Deps, QueryRequest, StdResult, to_binary, WasmQuery};
use crate::msg::{CalCanClaimRewardTokenResponse, CalCanMintRewardBoxResponse, ConfigAndStateResponse};
use crate::state::{InviterOptDetail, read_inviter_opt_detail, read_inviter_reward_config, read_inviter_reward_config_state};
use crate::third_msg::{BlindBoxQueryMsg, UserInfoResponse};

pub fn cal_can_mint_reward_box(deps: Deps, user: Addr, level_index: &u8) -> StdResult<CalCanMintRewardBoxResponse> {
    let user_info = _get_nft_user_info(&deps, user.clone())?;

    let inviter_opt_detail = read_inviter_opt_detail(deps.storage, &user)?;

    let minted_reward_box_quantity = inviter_opt_detail.mint_box_level_detail.get(level_index).unwrap_or(&0u32).clone();

    let total_reward_box_quantity = user_info.user_reward_box.get(level_index).unwrap_or(&0u32).clone();

    let can_mint_box_quantity = total_reward_box_quantity.sub(minted_reward_box_quantity);

    Ok(CalCanMintRewardBoxResponse {
        can_mint_box_quantity,
        total_reward_box_quantity,
        minted_reward_box_quantity,
    })
}

pub fn cal_can_claim_reward_token(deps: Deps, user: Addr) -> StdResult<CalCanClaimRewardTokenResponse> {
    let user_info = _get_nft_user_info(&deps, user.clone())?;

    let inviter_opt_detail = read_inviter_opt_detail(deps.storage, &user)?;

    let can_claim_token_quantity = user_info.user_reward_total_base_amount.sub(inviter_opt_detail.claim_token_quantity);

    Ok(CalCanClaimRewardTokenResponse {
        can_claim_token_quantity,
        total_reward_token_quantity: user_info.user_reward_total_base_amount,
        claimed_reward_token_quantity: inviter_opt_detail.claim_token_quantity,
    })
}

pub fn query_all_config_and_state(deps: Deps) -> StdResult<ConfigAndStateResponse> {
    let config = read_inviter_reward_config(deps.storage)?;
    let state = read_inviter_reward_config_state(deps.storage)?;
    Ok(ConfigAndStateResponse {
        config,
        state,
    })
}

pub fn query_inviter_detail(deps: Deps, user: Addr) -> StdResult<InviterOptDetail> {
    read_inviter_opt_detail(deps.storage, &user)
}

fn _get_nft_user_info(deps: &Deps, user: Addr) -> StdResult<UserInfoResponse> {
    let config = read_inviter_reward_config(deps.storage)?;

    let user_info: UserInfoResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.nft_contract.to_string(),
        msg: to_binary(&BlindBoxQueryMsg::GetUserInfo { user: user.clone() })?,
    }))?;

    Ok(user_info)
}