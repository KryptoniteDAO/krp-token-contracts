use crate::error::ContractError;
use crate::querier::query_balance;
use crate::state::{read_config, read_state, store_config, store_state};
use crate::third_msg::StakingRewardsExecuteMsg;
use cosmwasm_std::{
    attr, to_binary, Addr, Api, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128, WasmMsg,
};

pub fn optional_addr_validate(api: &dyn Api, addr: Option<String>) -> StdResult<Option<Addr>> {
    let addr = if let Some(addr) = addr {
        Some(api.addr_validate(&addr)?)
    } else {
        None
    };
    Ok(addr)
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner_addr: Option<Addr>,
    threshold: Option<Uint128>,
    rewards_contract: Option<Addr>,
    rewards_denom: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = read_config(deps.as_ref().storage)?;
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;

    if sender_raw != config.owner {
        return Err(ContractError::Unauthorized(
            "update_config".to_string(),
            info.sender.to_string(),
        ));
    }

    if let Some(owner_addr) = owner_addr {
        config.owner = deps.api.addr_canonicalize(owner_addr.as_str())?
    }

    if let Some(threshold) = threshold {
        config.threshold = threshold;
    }

    if let Some(rewards_contract) = rewards_contract {
        config.rewards_contract = deps.api.addr_canonicalize(rewards_contract.as_str())?;
    }

    if let Some(rewards_denom) = rewards_denom {
        config.rewards_denom = rewards_denom;
    }

    store_config(deps.storage, &config)?;

    Ok(Response::default())
}

pub fn distribute_rewards(
    deps: DepsMut,
    env: Env,
) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    let mut state = read_state(deps.storage)?;


    let rewards_balance = query_balance(deps.as_ref(), env.contract.address, config.rewards_denom.clone())?;
    if rewards_balance < config.threshold {
        return Err(ContractError::DistributeRewardsLessThanThreshold(
            config.threshold,
        ));
    }

    state.distributed_amount = rewards_balance;
    state.distributed_total += rewards_balance;
    state.update_time = env.block.time.seconds().into();

    let total = state.distributed_total.clone();
    store_state(deps.storage, &state)?;

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: deps
                .api
                .addr_humanize(&config.rewards_contract)?
                .to_string(),
            msg: to_binary(&StakingRewardsExecuteMsg::NotifyRewardAmount { })?,
            funds: vec![Coin {
                denom: config.rewards_denom,
                amount: rewards_balance,
            }],
        }))
        .add_attributes(vec![
            attr("action", "distribute_rewards"),
            attr("distributed_amount", rewards_balance.to_string()),
            attr("distributed_total", total.to_string()),
        ]))
}
