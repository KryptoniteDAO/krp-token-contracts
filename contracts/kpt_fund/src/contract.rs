use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Deps, to_binary, Uint128, Binary};
use cw2::set_contract_version;
use crate::handler::{get_reward, notify_reward_amount, re_stake, refresh_reward, stake, unstake, update_kpt_fund_config, withdraw};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{earned, get_claim_able_kpt, get_claim_able_kusd, get_reserved_kpt_for_vesting, get_user_last_withdraw_time, get_user_reward_per_token_paid, get_user_rewards, get_user_time2full_redemption, get_user_unstake_rate, kpt_fund_config};
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
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let gov = msg.gov.unwrap_or_else(|| info.sender.clone());


    let config = KptFundConfig {
        gov,
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
            update_config_msg
        } => {
            update_kpt_fund_config(deps, info, update_config_msg)
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
        QueryMsg::GetUserRewardPerTokenPaid { account } => {
            to_binary(&get_user_reward_per_token_paid(deps, account)?)
        }
        QueryMsg::GetUserRewards { account } => {
            to_binary(&get_user_rewards(deps, account)?)
        }
        QueryMsg::GetUserTime2fullRedemption { account } => {
            to_binary(&get_user_time2full_redemption(deps, account)?)
        }
        QueryMsg::GetUserUnstakeRate { account } => {
            to_binary(&get_user_unstake_rate(deps, account)?)
        }
        QueryMsg::GetUserLastWithdrawTime { account } => {
            to_binary(&get_user_last_withdraw_time(deps, account)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
