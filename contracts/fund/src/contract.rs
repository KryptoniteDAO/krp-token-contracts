use crate::handler::{
    accept_gov, get_reward, notify_reward_amount, re_stake, receive_cw20, refresh_reward, set_gov,
    set_ve_fund_minter, unstake, update_fund_config, ve_fund_mint, withdraw,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{
    earned, fund_config, get_claim_able_kusd, get_claim_able_seilor,
    get_reserved_seilor_for_vesting, get_user_last_withdraw_time, get_user_reward_per_token_paid,
    get_user_rewards, get_user_time2full_redemption, get_user_unstake_rate, is_ve_fund_minter,
};
use crate::state::{store_fund_config, FundConfig};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-seilor-fund";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let gov = msg.gov.unwrap_or_else(|| info.sender.clone());

    // validate that the claim_able_time is greater than current block time.
    if msg.claim_able_time.u64() <= env.block.time.seconds() {
        return Err(StdError::generic_err(
            "claim_able_time must be greater than current time",
        ));
    }
    let config = FundConfig {
        gov,
        ve_seilor_addr: msg.ve_seilor_addr,
        seilor_addr: msg.seilor_addr,
        kusd_denom: msg.kusd_denom,
        kusd_reward_addr: msg.kusd_reward_addr,
        kusd_reward_total_amount: Uint128::zero(),
        kusd_reward_total_paid_amount: Uint128::zero(),
        reward_per_token_stored: Uint128::zero(),
        exit_cycle: msg.exit_cycle,
        claim_able_time: msg.claim_able_time,
        new_gov: None,
    };

    store_fund_config(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "instantiate"),
        ("owner", info.sender.as_str()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::UpdateFundConfig { update_config_msg } => {
            update_fund_config(deps, env, info, update_config_msg)
        }
        ExecuteMsg::RefreshReward { account } => refresh_reward(deps, account),
        ExecuteMsg::Unstake { amount } => unstake(deps, env, info, amount),
        ExecuteMsg::Withdraw { user } => withdraw(deps, env, user),
        ExecuteMsg::ReStake { .. } => re_stake(deps, env, info),
        ExecuteMsg::GetReward { .. } => get_reward(deps, info),
        ExecuteMsg::NotifyRewardAmount { .. } => notify_reward_amount(deps, info),
        ExecuteMsg::SetGov { gov } => set_gov(deps, info, gov),
        ExecuteMsg::AcceptGov {} => accept_gov(deps, info),
        ExecuteMsg::SetVeFundMinter {
            minter,
            is_ve_minter,
        } => set_ve_fund_minter(deps, info, minter, is_ve_minter),
        ExecuteMsg::VeFundMint { user, amount } => ve_fund_mint(deps, info, user, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FundConfig {} => to_binary(&fund_config(deps)?),
        QueryMsg::GetClaimAbleSeilor { user } => {
            to_binary(&get_claim_able_seilor(deps, env, user)?)
        }
        QueryMsg::GetReservedSeilorForVesting { user } => {
            to_binary(&get_reserved_seilor_for_vesting(deps, env, user)?)
        }
        QueryMsg::Earned { account } => to_binary(&earned(deps, account)?),
        QueryMsg::GetClaimAbleKusd { account } => to_binary(&get_claim_able_kusd(deps, account)?),
        QueryMsg::GetUserRewardPerTokenPaid { account } => {
            to_binary(&get_user_reward_per_token_paid(deps, account)?)
        }
        QueryMsg::GetUserRewards { account } => to_binary(&get_user_rewards(deps, account)?),
        QueryMsg::GetUserTime2fullRedemption { account } => {
            to_binary(&get_user_time2full_redemption(deps, account)?)
        }
        QueryMsg::GetUserUnstakeRate { account } => {
            to_binary(&get_user_unstake_rate(deps, account)?)
        }
        QueryMsg::GetUserLastWithdrawTime { account } => {
            to_binary(&get_user_last_withdraw_time(deps, account)?)
        }
        QueryMsg::IsVeFundMinter { minter } => to_binary(&is_ve_fund_minter(deps, minter)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
