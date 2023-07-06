use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Deps, to_binary, Binary};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::handler::{claim_reward_token, mint_reward_box, update_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{cal_can_claim_reward_token, cal_can_mint_reward_box, query_all_config_and_state, query_inviter_detail};
use crate::state::{InviterRewardConfig, InviterRewardConfigState, store_inviter_reward_config, store_inviter_reward_config_state};


// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:blind-box-inviter-reward";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let gov = msg.gov.unwrap_or(info.sender);

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let inviter_reward_config = InviterRewardConfig {
        gov: gov.clone(),
        nft_contract: msg.nft_contract,
        reward_native_token: msg.reward_native_token,
        start_mint_box_time: msg.start_mint_box_time,
        end_mint_box_time: msg.end_mint_box_time,
        start_claim_token_time: msg.start_claim_token_time,
        end_claim_token_time: msg.end_claim_token_time,
    };

    let inviter_reward_config_state = InviterRewardConfigState {
        total_mint_box_count: 0,
        total_claim_token_quantity: 0,
        mint_box_level_detail: Default::default(),
    };

    store_inviter_reward_config(deps.storage, &inviter_reward_config)?;
    store_inviter_reward_config_state(deps.storage, &inviter_reward_config_state)?;

    Ok(Response::default()
        .add_attributes(vec![
            ("action", "instantiate"),
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
        ExecuteMsg::MintRewardBox { level_index, mint_num } => {
            mint_reward_box(deps, env, info, level_index, mint_num)
        }
        ExecuteMsg::ClaimRewardToken { amount } => {
            claim_reward_token(deps, env, info, amount)
        }
        ExecuteMsg::UpdateConfig { update_msg } => {
            update_config(deps, info, update_msg)
        }
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryAllConfigAndState { .. } => {
            to_binary(&query_all_config_and_state(deps)?)
        }
        QueryMsg::CalCanMintRewardBox { user, level_index } => {
            to_binary(&cal_can_mint_reward_box(deps, user, &level_index)?)
        }
        QueryMsg::CalCanClaimRewardToken { user } => {
            to_binary(&cal_can_claim_reward_token(deps, user)?)
        }
        QueryMsg::QueryInviterDetail { user } => {
            to_binary(&query_inviter_detail(deps, user)?)
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
    use cosmwasm_std::{Addr};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        // Set up the necessary variables for testing
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            gov: Some(Addr::unchecked("gov_address")),
            nft_contract: Addr::unchecked("nft_contract_address"),
            reward_native_token: "token".to_string(),
            start_mint_box_time: env.block.time.seconds(),
            end_mint_box_time: env.block.time.seconds(),
            start_claim_token_time: env.block.time.seconds(),
            end_claim_token_time: env.block.time.seconds(),
        };
        // Positive test case
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        assert_eq!(0, res.messages.len()); // Ensure no additional messages are sent
        assert_eq!(1, res.attributes.len()); // Ensure no additional attributes are added
    }
}