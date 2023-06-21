use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Deps, to_binary, Binary};
use cw2::set_contract_version;
use cw_utils::nonpayable;
use crate::error::ContractError;
use crate::handler::{claim_reward, update_blind_box_reward_config, update_blind_box_reward_token_config, update_reward_token_reward_level};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_blind_box_config, query_user_claim_rewards};
use crate::state::{BlindBoxRewardConfig, RewardLevelConfig, RewardTokenConfig, store_blind_box_reward_config, store_blind_box_reward_token_config};


// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:blind-box-reward";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let r = nonpayable(&info.clone());
    if r.is_err() {
        return Err(StdError::generic_err(r.err().unwrap().to_string()));
    }

    let gov = if let Some(gov_addr) = msg.gov {
        gov_addr
    } else {
        info.clone().sender
    };


    let blid_box_reward_config = BlindBoxRewardConfig {
        gov: gov.clone(),
        nft_contract: msg.nft_contract,
    };

    store_blind_box_reward_config(deps.storage, &blid_box_reward_config)?;


    for reward_token_map_msg in msg.reward_token_map_msgs {
        let mut reward_level_configs = vec![];
        for reward_level_config_msg in reward_token_map_msg.reward_levels.unwrap_or(vec![]) {
            let reward_level_config = RewardLevelConfig {
                reward_amount: reward_level_config_msg.reward_amount.unwrap_or(0),
                level_total_claimed_amount: 0,
            };
            reward_level_configs.push(reward_level_config);
        }

        let reward_token_config = RewardTokenConfig {
            total_reward_amount: reward_token_map_msg.total_reward_amount.unwrap_or(0),
            total_claimed_amount: 0,
            total_claimed_count: 0,
            claimable_time: reward_token_map_msg.claimable_time.unwrap_or(0),
            reward_levels: reward_level_configs,
        };

        store_blind_box_reward_token_config(deps.storage, reward_token_map_msg.reward_token, &reward_token_config)?;

        // set contract version
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    }


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
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateBlindBoxConfig {
            gov,
            nft_contract
        } => {
            update_blind_box_reward_config(deps, info, gov, nft_contract)
        }
        ExecuteMsg::UpdateBlindBoxRewardTokenConfig {
            reward_token,
            total_reward_amount,
            claimable_time
        } => {
            update_blind_box_reward_token_config(deps, info, reward_token, total_reward_amount, claimable_time)
        }
        ExecuteMsg::UpdateRewardTokenRewardLevel {
            reward_token,
            reward_level,
            reward_amount
        } => {
            update_reward_token_reward_level(deps, info, reward_token, reward_level, reward_amount)
        }
        ExecuteMsg::ClaimReward { recipient } => {
            claim_reward(deps, env, info, recipient)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryUserClaimRewards { user_addr } => {
            to_binary(&query_user_claim_rewards(deps, env, user_addr.to_string())?)
        }
        QueryMsg::QueryBlindBoxConfig {} => {
            to_binary(&query_blind_box_config(deps)?)
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
    use cosmwasm_std::{ Addr};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::msg::{RewardLevelConfigMsg, RewardTokenConfigMsg};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            gov: Some(info.sender.clone()),
            nft_contract: Addr::unchecked("nft_contract"),
            reward_token_map_msgs: vec![RewardTokenConfigMsg {
                reward_token: "reward_token".to_string(),
                total_reward_amount: Some(100),
                claimable_time: Some(100),
                reward_levels: Some(vec![RewardLevelConfigMsg {
                    reward_amount: Some(100),
                }]),
            }
            ],
        };
        // positive test case
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.attributes[0], ("action", "instantiate"));
        assert_eq!(res.attributes[1], ("owner", "creator"));

    }
}