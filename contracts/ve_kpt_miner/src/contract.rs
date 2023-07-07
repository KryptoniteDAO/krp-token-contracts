use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Deps, to_binary, Binary, Uint128, Uint256};
use cw2::set_contract_version;
use crate::handler::{get_reward, notify_reward_amount, refresh_reward, set_is_redemption_provider, update_miner_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, UpdateMinerConfigStruct};
use crate::querier::{earned, get_boost, get_miner_config, get_miner_state, last_time_reward_applicable, reward_per_token, staked_of};
use crate::state::{MinerConfig, MinerState, store_miner_config, store_miner_state};


// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-ve-kpt-miner";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let gov = msg.gov.unwrap_or_else(|| info.sender.clone());

    let miner_config = MinerConfig {
        gov,
        kusd_denom: msg.kusd_denom,
        kusd_controller_addr: msg.kusd_controller_addr,
        ve_kpt_boost_addr: msg.ve_kpt_boost_addr,
        kpt_fund_addr: msg.kpt_fund_addr,
        ve_kpt_addr: msg.ve_kpt_addr,
        reward_controller_addr: msg.reward_controller_addr,
    };
    let extra_rate = if let Some(extra_rate) = msg.extra_rate {
        extra_rate
    } else {
        Uint128::zero()
    };

    let miner_state = MinerState {
        duration: msg.duration,
        finish_at: Uint128::zero(),
        updated_at: Uint128::zero(),
        reward_rate: Uint256::zero(),
        reward_per_token_stored: Uint128::zero(),
        extra_rate,
        lockdown_period: msg.lockdown_period,
    };

    store_miner_config(deps.storage, &miner_config)?;
    store_miner_state(deps.storage, &miner_state)?;

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
        ExecuteMsg::UpdateMinerConfig {
            gov, kusd_denom, kusd_controller_addr,
            ve_kpt_boost_addr, kpt_fund_addr, ve_kpt_addr,reward_controller_addr
        } => {
            update_miner_config(deps, info, UpdateMinerConfigStruct {
                gov,
                kusd_denom,
                kusd_controller_addr,
                ve_kpt_boost_addr,
                kpt_fund_addr,
                ve_kpt_addr,
                reward_controller_addr,
            })
        }
        ExecuteMsg::SetIsRedemptionProvider { user, is_redemption_provider } => {
            set_is_redemption_provider(deps, info, user, is_redemption_provider)
        }
        ExecuteMsg::RefreshReward { account } => {
            refresh_reward(deps, env, account)
        }
        ExecuteMsg::GetReward {} => {
            get_reward(deps, env, info)
        }
        ExecuteMsg::NotifyRewardAmount { amount } => {
            notify_reward_amount(deps, env, info, amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::StakedOf { user } => { to_binary(&staked_of(deps, user)?) }
        QueryMsg::RewardPerToken {} => { to_binary(&reward_per_token(deps, env)?) }
        QueryMsg::LastTimeRewardApplicable {} => { to_binary(&last_time_reward_applicable(deps, env)?) }
        QueryMsg::GetBoost { account } => { to_binary(&get_boost(deps, account)?) }
        QueryMsg::Earned { account } => { to_binary(&earned(deps, env, account)?) }
        QueryMsg::GetMinerConfig {} => { to_binary(&get_miner_config(deps)?) }
        QueryMsg::GetMinerState {} => { to_binary(&get_miner_state(deps)?) }
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}


#[cfg(test)]
mod tests {
    // Import necessary dependencies
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, Uint128, Uint256};
    // Import functions and structs from the contract
    use crate::msg::{InstantiateMsg};
    use crate::state::{MinerConfig, MinerState, read_miner_config, read_miner_state};

    // Test the instantiate function
    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();


        let msg = InstantiateMsg {
            gov: Some(Addr::unchecked("gov")),
            kusd_denom: "kusd".to_string(),
            kusd_controller_addr: Addr::unchecked("kusd_controller_addr"),
            ve_kpt_boost_addr: Addr::unchecked("ve_kpt_boost_addr"),
            kpt_fund_addr: Addr::unchecked("kpt_fund_addr"),
            ve_kpt_addr: Addr::unchecked("ve_kpt_addr"),
            reward_controller_addr: Addr::unchecked("reward_controller_addr"),
            duration: Uint128::new(1),
            extra_rate: Some(Uint128::new(1)),
            lockdown_period: Uint128::new(1),
        };

        let info = mock_info("creator", &[]);
        let res = crate::contract::instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let config = MinerConfig {
            gov: Addr::unchecked("gov"),
            kusd_denom: "kusd".to_string(),
            kusd_controller_addr: Addr::unchecked("kusd_controller_addr"),
            ve_kpt_boost_addr: Addr::unchecked("ve_kpt_boost_addr"),
            kpt_fund_addr: Addr::unchecked("kpt_fund_addr"),
            ve_kpt_addr: Addr::unchecked("ve_kpt_addr"),
            reward_controller_addr: Addr::unchecked("reward_controller_addr"),
        };
        let state = MinerState {
            duration: Uint128::new(1),
            finish_at: Uint128::zero(),
            updated_at: Uint128::zero(),
            reward_rate: Uint256::zero(),
            reward_per_token_stored: Uint128::zero(),
            extra_rate: Uint128::new(1),
            lockdown_period: Uint128::new(1),
        };
        let stored_config: MinerConfig = read_miner_config(deps.as_ref().storage).unwrap();
        println!("stored_config {:?}", stored_config);
        let stored_state: MinerState = read_miner_state(deps.as_ref().storage).unwrap();
        print!("stored_state {:?}", stored_state);
        assert_eq!(config, stored_config);
        assert_eq!(state, stored_state);
    }
}