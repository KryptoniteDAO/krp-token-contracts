use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Deps, to_binary, Uint128, Binary, Addr};
use cw2::set_contract_version;
use cw_utils::nonpayable;
use crate::handler::{get_reward, notify_reward_amount, re_stake, refresh_reward, stake, unstake, update_kpt_fund_config, withdraw};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, UpdateConfigMsg};
use crate::querier::{earned, get_claim_able_kpt, get_claim_able_kusd, get_reserved_kpt_for_vesting, kpt_fund_config};
use crate::state::{KptFundConfig, store_kpt_fund_config};


// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-kpt-fund";
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

    let config = KptFundConfig {
        gov: gov,
        ve_kpt_addr: msg.ve_kpt_addr,
        kpt_addr: msg.kpt_addr,
        kusd_denom: msg.kusd_denom,
        kusd_reward_addr: msg.kusd_reward_addr,
        kusd_reward_total_amount: Uint128::zero(),
        kusd_reward_total_paid_amount: Uint128::zero(),
        reward_per_token_stored: Uint128::zero(),
        exit_cycle: msg.exit_cycle,
        claim_able_time: msg.claim_able_time,
    };

    store_kpt_fund_config(deps.storage, &config)?;

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
        ExecuteMsg::UpdateKptFundConfig
        {
            gov, ve_kpt_addr, kpt_addr, kusd_denom, kusd_reward_addr, exit_cycle, claim_able_time
        } => {
            let date_msg = UpdateConfigMsg {
                gov,
                ve_kpt_addr,
                kpt_addr,
                kusd_denom,
                kusd_reward_addr,
                exit_cycle,
                claim_able_time,
            };
            update_kpt_fund_config(deps, info, date_msg)
        }
        ExecuteMsg::RefreshReward { account } => {
            refresh_reward(deps, account)
        }
        ExecuteMsg::Stake { amount } => {
            stake(deps, info, amount)
        }
        ExecuteMsg::Unstake { amount } => {
            unstake(deps, env, info, amount)
        }
        ExecuteMsg::Withdraw { user } => {
            withdraw(deps, env, user)
        }
        ExecuteMsg::ReStake { .. } => {
            re_stake(deps, env, info)
        }
        ExecuteMsg::GetReward { .. } => {
            get_reward(deps, info)
        }
        ExecuteMsg::NotifyRewardAmount { .. } => {
            notify_reward_amount(deps, info)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::KptFundConfig {} => to_binary(&kpt_fund_config(deps)?),
        QueryMsg::GetClaimAbleKpt { user } => to_binary(&get_claim_able_kpt(deps, env, user)?),
        QueryMsg::GetReservedKptForVesting { user } => to_binary(&get_reserved_kpt_for_vesting(deps, env, user)?),
        QueryMsg::Earned { account } => to_binary(&earned(deps, account)?),
        QueryMsg::GetClaimAbleKusd { account } => to_binary(&get_claim_able_kusd(deps, account)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}


#[cfg(test)]
mod tests {
    // Import necessary dependencies for testing
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, from_binary, Uint128, Uint64};
    use crate::contract::query;
    use crate::msg::{InstantiateMsg, KptFundConfigResponse, QueryMsg, UpdateConfigMsg};
    use crate::state::{KptFundConfig, read_kpt_fund_config};

    // Test the instantiate function
    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &[]);
        let msg = InstantiateMsg {
            gov: None,
            ve_kpt_addr: Addr::unchecked("ve_kpt"),
            kpt_addr: Addr::unchecked("kpt"),
            kusd_denom: "kusd".to_string(),
            kusd_reward_addr: Addr::unchecked("kusd_reward"),
            exit_cycle: Uint64::from(10u64),
            claim_able_time: Uint64::from(10u64),
        };
        let res = crate::contract::instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res.attributes, vec![
            ("action", "instantiate"),
            ("owner", "owner"),
        ]);
        let config: KptFundConfig = read_kpt_fund_config(deps.as_ref().storage).unwrap();
        assert_eq!(config, KptFundConfig {
            gov: info.sender,
            ve_kpt_addr: msg.ve_kpt_addr,
            kpt_addr: msg.kpt_addr,
            kusd_denom: msg.kusd_denom,
            kusd_reward_addr: msg.kusd_reward_addr,
            kusd_reward_total_amount: Uint128::zero(),
            kusd_reward_total_paid_amount: Uint128::zero(),
            reward_per_token_stored: Uint128::zero(),
            exit_cycle: msg.exit_cycle,
            claim_able_time: msg.claim_able_time,
        });
    }

    // Test the update_kpt_fund_config function
    #[test]
    fn test_update_kpt_fund_config() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &[]);

        // Instantiate the contract
        let msg = InstantiateMsg {
            gov: None,
            ve_kpt_addr: Addr::unchecked("ve_kpt"),
            kpt_addr: Addr::unchecked("kpt"),
            kusd_denom: "kusd".to_string(),
            kusd_reward_addr: Addr::unchecked("kusd_reward"),
            exit_cycle: Uint64::from(10u64),
            claim_able_time: Uint64::from(10u64),
        };
        crate::contract::instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        // Update the config
        let update_msg = UpdateConfigMsg {
            gov: Option::from(Addr::unchecked("new_gov")),
            ve_kpt_addr: Option::from(Addr::unchecked("new_ve_kpt")),
            kpt_addr: Option::from(Addr::unchecked("new_kpt")),
            kusd_denom: Option::from("new_kusd".to_string()),
            kusd_reward_addr: Option::from(Addr::unchecked("new_kusd_reward")),
            exit_cycle: Option::from(Uint64::from(20u64)),
            claim_able_time: Option::from(Uint64::from(20u64)),
        };
        let info = mock_info("owner2", &[]);
        let res = crate::handler::update_kpt_fund_config(deps.as_mut(), info.clone(), update_msg.clone());
        assert!(res.is_err());
        let info = mock_info("owner", &[]);
        let res = crate::handler::update_kpt_fund_config(deps.as_mut(), info.clone(), update_msg.clone()).unwrap();
        println!("{:?}", res);
        let config: KptFundConfig = read_kpt_fund_config(deps.as_ref().storage).unwrap();
        println!("{:?}", config);
        assert_eq!(config, KptFundConfig {
            gov: Option::from(update_msg.gov.unwrap()).unwrap(),
            ve_kpt_addr: Option::from(update_msg.ve_kpt_addr.unwrap()).unwrap(),
            kpt_addr: Option::from(update_msg.kpt_addr.unwrap()).unwrap(),
            kusd_denom: Option::from(update_msg.kusd_denom.unwrap()).unwrap(),
            kusd_reward_addr: Option::from(update_msg.kusd_reward_addr.unwrap()).unwrap(),
            kusd_reward_total_amount: Uint128::zero(),
            kusd_reward_total_paid_amount: Uint128::zero(),
            reward_per_token_stored: Uint128::zero(),
            exit_cycle: Option::from(update_msg.exit_cycle.unwrap()).unwrap(),
            claim_able_time: Option::from(update_msg.claim_able_time.unwrap()).unwrap(),
        });
    }

    #[test]
    fn test_query_kpt_fund_config() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &[]);

        // Instantiate the contract
        let msg = InstantiateMsg {
            gov: None,
            ve_kpt_addr: Addr::unchecked("ve_kpt"),
            kpt_addr: Addr::unchecked("kpt"),
            kusd_denom: "kusd".to_string(),
            kusd_reward_addr: Addr::unchecked("kusd_reward"),
            exit_cycle: Uint64::from(10u64),
            claim_able_time: Uint64::from(10u64),
        };
        crate::contract::instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

        let msg = QueryMsg::KptFundConfig {};
        let res = query(deps.as_ref(), env, msg).unwrap();
        let config: KptFundConfigResponse = from_binary(&res).unwrap();

        assert_eq!(config, KptFundConfigResponse {
            gov: Addr::unchecked("owner"),
            ve_kpt_addr: Addr::unchecked("ve_kpt"),
            kpt_addr: Addr::unchecked("kpt"),
            kusd_denom: "kusd".to_string(),
            kusd_reward_addr: Addr::unchecked("kusd_reward"),
            kusd_reward_total_amount: Uint128::zero(),
            kusd_reward_total_paid_amount: Uint128::zero(),
            reward_per_token_stored: Uint128::zero(),
            exit_cycle: Uint64::from(10u64),
            claim_able_time: Uint64::from(10u64),
        });
    }
}