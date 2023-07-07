use crate::msg::{ConfigResponse, StateResponse};
use crate::state::{read_config, read_state};

use cosmwasm_std::{Addr, BalanceResponse, BankQuery, Deps, QueryRequest, StdResult, Uint128};

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = read_config(deps.storage)?;

    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?.to_string(),
        threshold: config.threshold,
        rewards_contract: deps
            .api
            .addr_humanize(&config.rewards_contract)?
            .to_string(),
        rewards_denom: config.rewards_denom,
    })
}

pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = read_state(deps.storage)?;

    Ok(StateResponse {
        distributed_amount: state.distributed_amount,
        update_time: state.update_time,
        distributed_total: state.distributed_total,
    })
}

pub fn query_balance(deps: Deps, account_addr: Addr, denom: String) -> StdResult<Uint128> {
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;
    Ok(balance.amount.amount)
}
