use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Deps, to_binary, Binary, Addr, Uint128};
use cw2::set_contract_version;
use cw_utils::nonpayable;
use crate::handler::{get_reward, notify_reward_amount, receive_cw20, update_staking_config, update_staking_duration, withdraw};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, UpdateStakingConfigStruct};
use crate::querier::{earned, get_boost, get_user_reward_per_token_paid, get_user_updated_at, last_time_reward_applicable, query_staking_config, query_staking_state, reward_per_token};
use crate::state::{StakingConfig, StakingState, store_staking_config, store_staking_state};


// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-ve-kpt-staking-rewards";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let r = nonpayable(&info);
    if r.is_err() {
        return Err(StdError::generic_err("NonPayable"));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let gov = if let Some(gov_addr) = msg.gov {
        Addr::unchecked(gov_addr)
    } else {
        info.sender.clone()
    };

    let staking_config = StakingConfig {
        gov,
        staking_token: msg.staking_token,
        rewards_token: msg.rewards_token,
        ve_kpt_boost: msg.ve_kpt_boost,
        kpt_fund: msg.kpt_fund,
        reward_controller_addr: msg.reward_controller_addr,
    };

    store_staking_config(deps.storage, &staking_config)?;

    let staking_state = StakingState {
        duration: msg.duration,
        finish_at: Uint128::zero(),
        updated_at: Uint128::zero(),
        reward_rate: Uint128::zero(),
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
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(msg) => {
            receive_cw20(deps, env, info, msg)
        }
        ExecuteMsg::UpdateStakingConfig {
            gov,
            staking_token,
            rewards_token,
            ve_kpt_boost,
            kpt_fund,
            reward_controller_addr, } => {
            update_staking_config(deps, info, UpdateStakingConfigStruct {
                gov,
                staking_token,
                rewards_token,
                ve_kpt_boost,
                kpt_fund,
                reward_controller_addr,
            })
        }
        ExecuteMsg::UpdateStakingState { duration } => {
            update_staking_duration(deps, env, info, duration)
        }
        ExecuteMsg::GetReward {} => {
            get_reward(deps, env, info)
        }
        ExecuteMsg::NotifyRewardAmount { amount } => {
            notify_reward_amount(deps, env, info, amount)
        }
        ExecuteMsg::Withdraw { amount } => {
            withdraw(deps, env, info, amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RewardPerToken {} => {
            to_binary(&reward_per_token(deps, env)?)
        }
        QueryMsg::LastTimeRewardApplicable {} => {
            to_binary(&last_time_reward_applicable(deps, env)?)
        }
        QueryMsg::GetBoost { account } => {
            to_binary(&get_boost(deps, account)?)
        }
        QueryMsg::Earned { account } => {
            to_binary(&earned(deps, env, account)?)
        }
        QueryMsg::QueryStakingConfig {} => {
            to_binary(&query_staking_config(deps)?)
        }
        QueryMsg::QueryStakingState {} => {
            to_binary(&query_staking_state(deps)?)
        }
        QueryMsg::GetUserUpdatedAt { account } => {
            to_binary(&get_user_updated_at(deps, account)?)
        }
        QueryMsg::GetUserRewardPerTokenPaid { account } => {
            to_binary(&get_user_reward_per_token_paid(deps, account)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{StdError, Uint128, Coin};
    use cw2::get_contract_version;
    use crate::state::{read_staking_config, read_staking_state};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            gov: None,
            staking_token: Addr::unchecked("staking_token"),
            rewards_token: Addr::unchecked("rewards_token"),
            ve_kpt_boost: Addr::unchecked("ve_kpt_boost"),
            kpt_fund: Addr::unchecked("kpt_fund"),
            reward_controller_addr: Addr::unchecked("reward_controller_addr"),
            duration: Uint128::from(100u128),
        };
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());
        // Verify staking config was stored correctly
        let staking_config = read_staking_config(deps.as_ref().storage).unwrap();
        assert_eq!(staking_config.gov, info.sender);
        assert_eq!(staking_config.staking_token, Addr::unchecked("staking_token"));
        assert_eq!(staking_config.rewards_token, Addr::unchecked("rewards_token"));
        assert_eq!(staking_config.ve_kpt_boost, Addr::unchecked("ve_kpt_boost"));
        assert_eq!(staking_config.kpt_fund, Addr::unchecked("kpt_fund"));
        assert_eq!(staking_config.reward_controller_addr, Addr::unchecked("reward_controller_addr"));
        // Verify staking state was stored correctly
        let staking_state = read_staking_state(deps.as_ref().storage).unwrap();
        assert_eq!(staking_state.duration, Uint128::from(100u128));
        assert_eq!(staking_state.finish_at, Uint128::zero());
        assert_eq!(staking_state.updated_at, Uint128::zero());
        assert_eq!(staking_state.reward_rate, Uint128::zero());
        assert_eq!(staking_state.reward_per_token_stored, Uint128::zero());
        assert_eq!(staking_state.total_supply, Uint128::zero());
        // Verify contract version was stored correctly
        let contract_version = get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(contract_version.contract, CONTRACT_NAME.to_string());
        assert_eq!(contract_version.version, CONTRACT_VERSION.to_string());
    }

    #[test]
    fn test_instantiate_nonpayable() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            gov: None,
            staking_token: Addr::unchecked("staking_token"),
            rewards_token: Addr::unchecked("rewards_token"),
            ve_kpt_boost: Addr::unchecked("ve_kpt_boost"),
            kpt_fund: Addr::unchecked("kpt_fund"),
            reward_controller_addr: Addr::unchecked("reward_controller_addr"),
            duration: Uint128::from(100u128),
        };
        let info = mock_info("creator", &vec![Coin::new(1000, "token")]);
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert_eq!(res.unwrap_err(), StdError::generic_err("NonPayable"));
    }
}
