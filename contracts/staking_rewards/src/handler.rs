use cosmwasm_std::{Addr, attr, CosmosMsg, DepsMut, Env, from_binary, MessageInfo, QueryRequest, Response, StdError, StdResult, SubMsg, to_binary, Uint128, WasmMsg, WasmQuery};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use crate::msg::{Cw20HookMsg, UpdateStakingConfigStruct};
use crate::querier::{earned, is_empty_address, reward_per_token};
use crate::state::{read_balance_of, read_rewards, read_staking_config, read_staking_state, store_balance_of, store_rewards, store_staking_config, store_staking_state, store_user_reward_per_token_paid, store_user_updated_at};
use crate::third_msg::{GetUnlockTimeResponse, KptFundExecuteMsg, RewardTokenExecuteMsg, VeKptBoostQueryMsg};

pub fn update_staking_config(deps: DepsMut, info: MessageInfo,
                             update_struct: UpdateStakingConfigStruct) -> StdResult<Response> {
    let mut staking_config = read_staking_config(deps.storage)?;
    if info.sender.ne(&staking_config.gov) {
        return Err(StdError::generic_err("unauthorized"));
    }

    let mut attrs = vec![];
    attrs.push(attr("action", "update_staking_config"));

    if let Some(gov) = update_struct.gov {
        staking_config.gov = gov.clone();
        attrs.push(attr("gov", gov.to_string()));
    }

    if let Some(staking_token) = update_struct.staking_token {
        staking_config.staking_token = staking_token.clone();
        attrs.push(attr("staking_token", staking_token.to_string()));
    }

    if let Some(rewards_token) = update_struct.rewards_token {
        staking_config.rewards_token = rewards_token.clone();
        attrs.push(attr("rewards_token", rewards_token.to_string()));
    }

    if let Some(ve_kpt_boost) = update_struct.ve_kpt_boost {
        staking_config.ve_kpt_boost = ve_kpt_boost.clone();
        attrs.push(attr("ve_kpt_boost", ve_kpt_boost.to_string()));
    }

    if let Some(kpt_fund) = update_struct.kpt_fund {
        staking_config.kpt_fund = kpt_fund.clone();
        attrs.push(attr("kpt_fund", kpt_fund.to_string()));
    }
    if let Some(reward_controller_addr) = update_struct.reward_controller_addr {
        staking_config.reward_controller_addr = reward_controller_addr.clone();
        attrs.push(attr("reward_controller_addr", reward_controller_addr.to_string()));
    }

    store_staking_config(deps.storage, &staking_config)?;

    Ok(Response::new().add_attributes(attrs))
}


pub fn update_staking_duration(deps: DepsMut, env: Env, info: MessageInfo, duration: Uint128) -> StdResult<Response> {
    let staking_config = read_staking_config(deps.storage)?;
    let mut staking_state = read_staking_state(deps.storage)?;
    if info.sender.ne(&staking_config.gov) {
        return Err(StdError::generic_err("unauthorized"));
    }

    let current_time = Uint128::from(env.block.time.seconds());
    if staking_state.finish_at > current_time {
        return Err(StdError::generic_err("duration can only be updated after the end of the current period"));
    }
    staking_state.duration = duration.clone();

    store_staking_state(deps.storage, &staking_state)?; // update state

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "update_staking_state"),
            attr("duration", duration.to_string()),
        ])
    )
}


// Update user's claimable reward data and record the timestamp.
fn _update_reward(deps: DepsMut, env: Env, account: Addr) -> StdResult<()> {
    let reward_per_token_response = reward_per_token(deps.as_ref(), env.clone()).unwrap();
    let reward_per_token_stored = reward_per_token_response.reward_per_token;
    // let last_time_reward_applicable_response = last_time_reward_applicable(deps.as_ref(), env.clone()).unwrap();
    // let updated_at = last_time_reward_applicable_response.last_time_reward_applicable;

    if !is_empty_address(account.as_str()) {
        let earned = earned(deps.as_ref(), env.clone(), account.clone()).unwrap().earned;
        store_rewards(deps.storage, account.clone(), &earned)?;
        store_user_reward_per_token_paid(deps.storage, account.clone(), &reward_per_token_stored)?;
        store_user_updated_at(deps.storage, account.clone(), &Uint128::from(env.block.time.seconds()))?;
    }
    Ok(())
}

// Allows users to stake a specified amount of tokens
pub fn stake(deps: DepsMut, info: MessageInfo, user: Addr, amount: Uint128) -> StdResult<Response> {
    if amount.is_zero() {
        return Err(StdError::generic_err("amount = 0"));
    }

    let mut balance_of = read_balance_of(deps.storage, user.clone());
    let mut staking_state = read_staking_state(deps.storage)?;
    balance_of += amount;
    staking_state.total_supply += amount;

    store_balance_of(deps.storage, info.sender.clone(), &balance_of)?;
    store_staking_state(deps.storage, &staking_state)?;
    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "stake"),
            attr("user", user.to_string()),
            attr("amount", amount.to_string()),
        ]))
}

// Allows users to withdraw a specified amount of staked tokens
pub fn withdraw(deps: DepsMut, info: MessageInfo, token: Addr, user: Addr, amount: Uint128) -> StdResult<Response> {
    if amount.is_zero() {
        return Err(StdError::generic_err("amount = 0"));
    }

    let mut balance_of = read_balance_of(deps.storage, user.clone());
    let mut staking_state = read_staking_state(deps.storage)?;
    balance_of -= amount;
    staking_state.total_supply -= amount;

    store_balance_of(deps.storage, info.sender.clone(), &balance_of)?;
    store_staking_state(deps.storage, &staking_state)?;

    let messages: Vec<CosmosMsg> = vec![
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: user.clone().to_string(),
                amount,
            })?,
            funds: vec![],
        })
    ];

    Ok(Response::new()
        .add_messages(messages)
        .add_attributes(vec![
            attr("action", "withdraw"),
            attr("user", user.to_string()),
            attr("amount", amount.to_string()),
        ]))
}

/// ## Description
/// Receives a message of type [`Cw20ReceiveMsg`] and processes it depending on the received template.
/// If the template is not found in the received message, then an [`ContractError`] is returned,
/// otherwise it returns the [`Response`] with the specified attributes if the operation was successful.
/// ## Params
/// * **deps** is an object of type [`DepsMut`].
///
/// * **env** is an object of type [`Env`].
///
/// * **info** is an object of type [`MessageInfo`].
///
/// * **cw20_msg** is an object of type [`Cw20ReceiveMsg`]. This is the CW20 message that has to be processed.
pub fn receive_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let contract_addr = info.sender.clone();
    let msg_sender = deps.api.addr_validate(&cw20_msg.sender).unwrap();
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Stake {}) => {
            let staking_config = read_staking_config(deps.storage)?;
            if contract_addr.ne(&staking_config.staking_token) {
                return Err(StdError::generic_err("not staking token"));
            }
            stake(deps, info, msg_sender, cw20_msg.amount)
        }
        Ok(Cw20HookMsg::Withdraw {}) => {
            let staking_config = read_staking_config(deps.storage)?;
            if contract_addr.ne(&staking_config.staking_token) {
                return Err(StdError::generic_err("not staking token"));
            }
            withdraw(deps, info, contract_addr.clone(), msg_sender, cw20_msg.amount)
        }
        Err(_) => Err(StdError::generic_err("Not a valid cw20 message")),
    }
}

// Allows users to claim their earned rewards
pub fn get_reward(mut deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    _update_reward(deps.branch(), env.clone(), info.sender.clone())?;

    let sender = info.sender.clone();
    let staking_config = read_staking_config(deps.storage)?;

    let unlock_time_msg = VeKptBoostQueryMsg::GetUnlockTime {
        user: sender.clone(),
    };
    let unlock_time_response: GetUnlockTimeResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: staking_config.ve_kpt_boost.clone().to_string(),
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
            contract_addr: staking_config.kpt_fund.clone().to_string(),
            msg: to_binary(&refresh_reward_msg)?,
            funds: vec![],
        });
        sub_msgs.push(refresh_reward_sub_msg);

        let mint_msg = RewardTokenExecuteMsg::Mint {
            recipient: sender.clone().to_string(),
            amount: reward.clone(),
        };
        let mint_sub_msg = SubMsg::new(WasmMsg::Execute {
            contract_addr: staking_config.rewards_token.clone().to_string(),
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

// Allows the owner to set the mining rewards.
pub fn notify_reward_amount(mut deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> StdResult<Response> {
    _update_reward(deps.branch(), env.clone(), Addr::unchecked(""))?;
    if !amount.is_zero() {
        let staking_config = read_staking_config(deps.storage)?;
        if info.sender != staking_config.reward_controller_addr {
            return Err(StdError::generic_err("unauthorized"));
        }

        let current_time = Uint128::from(env.block.time.seconds());
        let mut staking_state = read_staking_state(deps.storage)?;
        if current_time >= staking_state.finish_at {
            staking_state.reward_rate = amount / staking_state.duration;
        } else {
            let remaining_rewards = (staking_state.finish_at - current_time) * staking_state.reward_rate;
            staking_state.reward_rate = (amount + remaining_rewards) / staking_state.duration;
        }
        if staking_state.reward_rate.is_zero() {
            return Err(StdError::generic_err("reward rate = 0"));
        }

        staking_state.finish_at = current_time + staking_state.duration;
        staking_state.updated_at = current_time;

        store_staking_state(deps.storage, &staking_state)?;
    }
    Ok(Response::new().add_attributes(vec![
        attr("action", "notify_reward_amount"),
        attr("sender", info.sender),
    ]))
}
