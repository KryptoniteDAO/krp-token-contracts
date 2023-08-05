use crate::error::ContractError;
use crate::handler::{pre_mint_nft, receive_cw20, update_config, user_withdraw};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_config_infos, query_user_infos};
use crate::state::{store_treasure_config, store_treasure_state, TreasureConfig, TreasureState};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:treasure";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let sender = info.clone().sender;
    let gov = msg.gov.unwrap_or(sender.clone());

    let config = TreasureConfig {
        gov: gov.clone(),
        lock_token: msg.lock_token.clone(),
        start_time: msg.start_time,
        end_time: msg.end_time,
        integral_reward_coefficient: msg.integral_reward_coefficient,
        lock_duration: msg.lock_duration,
        punish_coefficient: msg.punish_coefficient,
        mint_nft_cost_integral: msg.mint_nft_cost_integral,
        winning_num: msg.winning_num,
        mod_num: msg.mod_num,
        punish_receiver: msg.punish_receiver,
    };

    let state = TreasureState {
        current_locked_amount: Uint128::zero(),
        current_integral_amount: Uint128::zero(),
        total_locked_amount: Uint128::zero(),
        total_withdraw_amount: Uint128::zero(),
        total_punish_amount: Uint128::zero(),
        total_win_nft_num: 0,
        total_lose_nft_num: 0,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    store_treasure_config(deps.storage, &config)?;
    store_treasure_state(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![("action", "instantiate")]))
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
        ExecuteMsg::UpdateConfig(msg) => update_config(deps, info, msg),
        ExecuteMsg::UserWithdraw { amount } => user_withdraw(deps, env, info, amount),
        ExecuteMsg::PreMintNft { mint_num } => pre_mint_nft(deps, env, info, mint_num),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfigInfos { .. } => to_binary(&query_config_infos(deps)?),
        QueryMsg::QueryUserInfos { msg } => to_binary(&query_user_infos(deps, msg)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
