use crate::error::ContractError;
use crate::handler::{
    accept_gov, pre_mint_nft, receive_cw20, set_gov, update_config, user_unlock, user_withdraw,
};
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
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let sender = info.clone().sender;
    let gov = msg.gov.unwrap_or(sender.clone());
    // verify the following parameters:
    //
    //     start_lock_time >= current block time
    // end_lock_time > start_lock_time
    // nft_start_pre_mint_time >= current block time
    // nft_start_pre_mint_time > end_lock_time
    // nft_end_pre_mint_time > nft_start_pre_mint_time
    let current_block_time = env.block.time.seconds();
    if msg.start_lock_time < current_block_time {
        return Err(ContractError::InvalidStartLockTime {});
    }
    if msg.end_lock_time <= msg.start_lock_time {
        return Err(ContractError::InvalidEndLockTime {});
    }
    if msg.nft_start_pre_mint_time < current_block_time {
        return Err(ContractError::InvalidNftStartPreMintTime {});
    }
    if msg.nft_start_pre_mint_time <= msg.end_lock_time {
        return Err(ContractError::InvalidNftStartPreMintTimeAndEndLockTime {});
    }
    if msg.nft_end_pre_mint_time <= msg.nft_start_pre_mint_time {
        return Err(ContractError::InvalidNftEndPreMintTime {});
    }

    let config = TreasureConfig {
        gov: gov.clone(),
        lock_token: msg.lock_token.clone(),
        start_lock_time: msg.start_lock_time,
        end_lock_time: msg.end_lock_time,
        dust_reward_per_second: msg.dust_reward_per_second,
        withdraw_delay_duration: msg.withdraw_delay_duration,
        winning_num: msg.winning_num,
        mod_num: msg.mod_num,
        punish_receiver: msg.punish_receiver,
        nft_start_pre_mint_time: msg.nft_start_pre_mint_time,
        nft_end_pre_mint_time: msg.nft_end_pre_mint_time,
        no_delay_punish_coefficient: msg.no_delay_punish_coefficient,
        mint_nft_cost_dust: msg.mint_nft_cost_dust,
        new_gov: None,
    };

    let state = TreasureState {
        current_unlock_amount: Uint128::zero(),
        current_locked_amount: Uint128::zero(),
        total_locked_amount: Uint128::zero(),
        total_unlock_amount: Uint128::zero(),
        total_withdraw_amount: Uint128::zero(),
        total_punish_amount: Uint128::zero(),
        total_cost_dust_amount: Uint128::zero(),
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
        ExecuteMsg::UpdateConfig(msg) => update_config(deps, env, info, msg),
        ExecuteMsg::UserWithdraw { amount } => user_withdraw(deps, env, info, amount),
        ExecuteMsg::UserUnlock { amount } => user_unlock(deps, env, info, amount),
        ExecuteMsg::PreMintNft { mint_num } => pre_mint_nft(deps, env, info, mint_num),
        ExecuteMsg::SetGov { gov } => set_gov(deps, info, gov),
        ExecuteMsg::AcceptGov {} => accept_gov(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfigInfos { .. } => to_binary(&query_config_infos(deps)?),
        QueryMsg::QueryUserInfos { user } => to_binary(&query_user_infos(deps, env, user)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
