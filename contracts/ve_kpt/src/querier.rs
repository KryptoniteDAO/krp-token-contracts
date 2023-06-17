use cosmwasm_std::{Addr, Deps, StdResult};
use crate::msg::{IsMinterResponse, VoteConfigResponse};
use crate::state::{is_minter, read_vote_config, VoteConfig};

pub fn query_vote_config(deps: Deps) -> StdResult<VoteConfigResponse> {
    let config: VoteConfig = read_vote_config(deps.storage)?;
    Ok(VoteConfigResponse {
        max_supply: config.max_supply,
        kpt_fund: config.kpt_fund,
        gov: config.gov,
        max_minted: config.max_minted,
        total_minted: config.total_minted,
    })
}

pub fn query_is_minter(deps: Deps, minter: Addr) -> StdResult<IsMinterResponse> {
    Ok(IsMinterResponse {
        is_minter: is_minter(deps.storage, minter).unwrap(),
    })
}