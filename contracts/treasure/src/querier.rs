use crate::msg::{ConfigInfosResponse, QueryUserInfosMsg, UserInfosResponse};
use crate::state::read_treasure_config;
use cosmwasm_std::{Deps, StdResult};

pub fn query_config_infos(deps: Deps) -> StdResult<ConfigInfosResponse> {
    let config = read_treasure_config(deps.storage)?;
    let state = crate::state::read_treasure_state(deps.storage)?;
    Ok(ConfigInfosResponse { config, state })
}

pub fn query_user_infos(deps: Deps, msg: QueryUserInfosMsg) -> StdResult<UserInfosResponse> {
    let mut user_state = None;
    let mut lock_records = None;
    let mut withdraw_records = None;
    let mut mint_nft_records = None;
    if msg.query_user_state {
        user_state = Some(crate::state::read_treasure_user_state(
            deps.storage,
            &msg.user_addr,
        )?);
    }

    if msg.query_lock_records {
        lock_records = Some(crate::state::read_user_lock_records(
            deps.storage,
            &msg.user_addr,
            msg.start_after.clone(),
            msg.limit.clone(),
        )?);
    }

    if msg.query_withdraw_records {
        withdraw_records = Some(crate::state::read_user_withdraw_records(
            deps.storage,
            &msg.user_addr,
            msg.start_after.clone(),
            msg.limit.clone(),
        )?);
    }

    if msg.query_mint_nft_records {
        mint_nft_records = Some(crate::state::read_user_mint_nft_records(
            deps.storage,
            &msg.user_addr,
            msg.start_after.clone(),
            msg.limit.clone(),
        )?);
    }

    Ok(UserInfosResponse {
        user_state,
        lock_records,
        withdraw_records,
        mint_nft_records,
    })
}
