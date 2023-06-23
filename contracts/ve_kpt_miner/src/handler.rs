use std::ops::Add;
use cosmwasm_std::{Addr, attr, DepsMut, Env, MessageInfo, QueryRequest, Response, StdError, StdResult, SubMsg, to_binary, Uint128, Uint256, WasmMsg, WasmQuery};
use crate::msg::{LastTimeRewardApplicableResponse, RewardPerTokenResponse, UpdateMinerConfigStruct, UpdateMinerStateStruct};
use crate::querier::{earned, is_empty_address, last_time_reward_applicable, reward_per_token};
use crate::state::{read_miner_config, read_miner_state, read_rewards, store_is_redemption_provider, store_miner_config, store_miner_state, store_rewards, store_user_reward_per_token_paid, store_user_updated_at};
use crate::third_msg::{GetUnlockTimeResponse, KptFundExecuteMsg, VeKptBoostQueryMsg, VeKptExecuteMsg};

pub fn update_miner_config(deps: DepsMut, info: MessageInfo,
                           update_struct: UpdateMinerConfigStruct) -> StdResult<Response> {
    let mut miner_config = read_miner_config(deps.storage)?;
    if info.sender.ne(&miner_config.gov) {
        return Err(StdError::generic_err("unauthorized"));
    }

    let mut attrs = vec![];
    attrs.push(attr("action", "update_miner_config"));
    if let Some(gov) = update_struct.gov {
        miner_config.gov = gov.clone();
        attrs.push(attr("gov", gov.to_string()));
    }
    if let Some(kusd_denom) = update_struct.kusd_denom {
        miner_config.kusd_denom = kusd_denom.clone();
        attrs.push(attr("kusd_denom", kusd_denom.to_string()));
    }
    if let Some(kusd_controller_addr) = update_struct.kusd_controller_addr {
        miner_config.kusd_controller_addr = kusd_controller_addr.clone();
        attrs.push(attr("kusd_controller_addr", kusd_controller_addr.to_string()));
    }
    if let Some(ve_kpt_boost_addr) = update_struct.ve_kpt_boost_addr {
        miner_config.ve_kpt_boost_addr = ve_kpt_boost_addr.clone();
        attrs.push(attr("ve_kpt_boost_addr", ve_kpt_boost_addr.to_string()));
    }
    if let Some(kpt_fund_addr) = update_struct.kpt_fund_addr {
        miner_config.kpt_fund_addr = kpt_fund_addr.clone();
        attrs.push(attr("kpt_fund_addr", kpt_fund_addr.to_string()));
    }
    if let Some(ve_kpt_addr) = update_struct.ve_kpt_addr {
        miner_config.ve_kpt_addr = ve_kpt_addr.clone();
        attrs.push(attr("ve_kpt_addr", ve_kpt_addr.to_string()));
    }
    if let Some(reward_controller_addr) = update_struct.reward_controller_addr {
        miner_config.reward_controller_addr = reward_controller_addr.clone();
        attrs.push(attr("reward_controller_addr", reward_controller_addr.to_string()));
    }

    store_miner_config(deps.storage, &miner_config)?; // update config

    Ok(Response::new().add_attributes(attrs))
}

pub fn update_miner_state(deps: DepsMut, env: Env, info: MessageInfo, update_struct: UpdateMinerStateStruct) -> StdResult<Response> {
    let miner_config = read_miner_config(deps.storage)?;
    let mut miner_state = read_miner_state(deps.storage)?;
    if info.sender.ne(&miner_config.gov) {
        return Err(StdError::generic_err("unauthorized"));
    }

    let mut attrs = vec![];
    attrs.push(attr("action", "update_miner_state"));
    if let Some(duration) = update_struct.duration {
        let current_time = Uint128::from(env.block.time.seconds());
        if miner_state.finish_at > current_time {
            return Err(StdError::generic_err("duration can only be updated after the end of the current period"));
        }
        miner_state.duration = duration.clone();
        attrs.push(attr("duration", duration.to_string()));
    }


    if let Some(extra_rate) = update_struct.extra_rate {
        miner_state.extra_rate = extra_rate.clone();
        attrs.push(attr("extra_rate", extra_rate.to_string()));
    }

    if let Some(lockdown_period) = update_struct.lockdown_period {
        miner_state.lockdown_period = lockdown_period.clone();
        attrs.push(attr("lockdown_period", lockdown_period.to_string()));
    }

    store_miner_state(deps.storage, &miner_state)?; // update state

    Ok(Response::new().add_attributes(attrs))
}

pub fn set_is_redemption_provider(deps: DepsMut, info: MessageInfo, user: Addr, is_redemption_provider: bool) -> StdResult<Response> {
    let miner_config = read_miner_config(deps.storage)?;
    if info.sender.ne(&miner_config.gov) {
        return Err(StdError::generic_err("unauthorized"));
    }
    store_is_redemption_provider(deps.storage, user.clone(), &is_redemption_provider)?;
    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "set_is_redemption_provider"),
            attr("user", user.to_string()),
            attr("is_redemption_provider", is_redemption_provider.to_string()),
        ]))
}

fn _update_reward(deps: DepsMut, env: Env, account: Addr) -> StdResult<()> {
    let reward_per_token_response: RewardPerTokenResponse = reward_per_token(deps.as_ref(), env.clone()).unwrap();
    let reward_per_token_stored = reward_per_token_response.reward_per_token;
    let last_time_reward_applicable_response: LastTimeRewardApplicableResponse = last_time_reward_applicable(deps.as_ref(), env.clone()).unwrap();
    let updated_at = last_time_reward_applicable_response.last_time_reward_applicable;

    let mut miner_state = read_miner_state(deps.storage)?;
    miner_state.reward_per_token_stored = reward_per_token_stored.clone();
    miner_state.updated_at = updated_at.clone();
    store_miner_state(deps.storage, &miner_state)?; // update state

    if !is_empty_address(account.as_str()) {
        let earned = earned(deps.as_ref(), env.clone(), account.clone()).unwrap().earned;
        store_rewards(deps.storage, account.clone(), &earned)?;
        store_user_reward_per_token_paid(deps.storage, account.clone(), &reward_per_token_stored)?;
        store_user_updated_at(deps.storage, account.clone(), &Uint128::from(env.block.time.seconds()))?;
    }
    Ok(())
}

/**
 * @notice Update user's claimable reward data and record the timestamp.
 */
pub fn refresh_reward(deps: DepsMut, env: Env, account: Addr) -> StdResult<Response> {
    _update_reward(deps, env, account)?;
    Ok(Response::new().add_attributes(vec![attr("action", "refresh_reward")]))
}

pub fn get_reward(mut deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    _update_reward(deps.branch(), env.clone(), info.sender.clone())?;

    let sender = info.sender.clone();
    let miner_config = read_miner_config(deps.storage)?;

    let unlock_time_msg = VeKptBoostQueryMsg::GetUnlockTime {
        user: sender.clone(),
    };
    let unlock_time_response: GetUnlockTimeResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: miner_config.ve_kpt_boost_addr.clone().to_string(),
        msg: to_binary(&unlock_time_msg)?,
    }))?;
    let unlock_time = unlock_time_response.unlock_time;
    let current_time = Uint128::from(env.block.time.seconds());
    if current_time < unlock_time {
        return Err(StdError::generic_err("Your lock-in period has not ended. You can't claim your veKPT now."));
    }
    let reward = read_rewards(deps.storage, info.sender.clone());

    let mut sub_msgs = vec![];
    if reward > Uint128::zero() {
        store_rewards(deps.storage, sender.clone(), &Uint128::zero())?;
        let refresh_reward_msg = KptFundExecuteMsg::RefreshReward {
            account: sender.clone(),
        };
        let refresh_reward_sub_msg = SubMsg::new(WasmMsg::Execute {
            contract_addr: miner_config.kpt_fund_addr.clone().to_string(),
            msg: to_binary(&refresh_reward_msg)?,
            funds: vec![],
        });
        sub_msgs.push(refresh_reward_sub_msg);

        let mint_msg = VeKptExecuteMsg::Mint {
            recipient: sender.clone().to_string(),
            amount: reward.clone(),
        };
        let mint_sub_msg = SubMsg::new(WasmMsg::Execute {
            contract_addr: miner_config.ve_kpt_addr.clone().to_string(),
            msg: to_binary(&mint_msg)?,
            funds: vec![],
        });
        sub_msgs.push(mint_sub_msg);
    }

    Ok(Response::new()
        .add_submessages(sub_msgs)
        .add_attributes(vec![
            attr("action", "get_reward"),
            attr("sender", sender),
            attr("reward", reward),
        ]))
}

pub fn notify_reward_amount(mut deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> StdResult<Response> {
    _update_reward(deps.branch(), env.clone(), Addr::unchecked(""))?;
    if !amount.is_zero() {
        let miner_config = read_miner_config(deps.storage)?;
        if info.sender != miner_config.reward_controller_addr {
            return Err(StdError::generic_err("unauthorized"));
        }

        let current_time = Uint128::from(env.block.time.seconds());
        let mut miner_state = read_miner_state(deps.storage)?;
        if current_time >= miner_state.finish_at {
            // miner_state.reward_rate = amount / miner_state.duration;
            miner_state.reward_rate = Uint256::from(amount).multiply_ratio(Uint256::from(1000000000000u128), Uint256::from(miner_state.duration));
        } else {
            // let remaining_rewards = (miner_state.finish_at - current_time) * miner_state.reward_rate;
            let remaining_rewards = Uint256::from(miner_state.finish_at - current_time)
                .multiply_ratio(Uint256::from(miner_state.reward_rate), Uint256::from(1000000000000u128));
            // miner_state.reward_rate = (amount + remaining_rewards) / miner_state.duration;
            miner_state.reward_rate = (Uint256::from(amount).add(remaining_rewards))
                .multiply_ratio(Uint256::from(1000000000000u128), Uint256::from(miner_state.duration));
        }
        if miner_state.reward_rate.is_zero() {
            return Err(StdError::generic_err("reward rate = 0"));
        }

        miner_state.finish_at = current_time + miner_state.duration;
        miner_state.updated_at = current_time;

        store_miner_state(deps.storage, &miner_state)?;
    }
    Ok(Response::new().add_attributes(vec![
        attr("action", "notify_reward_amount"),
        attr("sender", info.sender),
    ]))
}
