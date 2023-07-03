use std::collections::HashMap;
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Deps, to_binary, Binary};
use cw2::set_contract_version;
use cw_utils::nonpayable;
use crate::error::ContractError;
use crate::handler::{open_blind_box, receive_cw20, update_box_reward_config, update_reward_config, user_claim_nft_reward};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_all_config_and_state, query_box_claimable_infos, query_box_open_info, test_random};
use crate::state::{BoxRewardConfigState, OrdinaryBoxRewardLevelConfigState, RandomBoxRewardRuleConfigState, RewardConfig, set_box_reward_config, set_box_reward_config_state, set_reward_config};

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
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let reward_config = RewardConfig {
        gov: gov.clone(),
        nft_contract: msg.nft_contract,
    };

    // init state
    let mut ordinary_box_reward_level_config_state_map: HashMap<u8, OrdinaryBoxRewardLevelConfigState> = HashMap::new();

    msg.box_config.ordinary_box_reward_level_config.keys().for_each(|k| {
        let ordinary_box_reward_level_config_state = OrdinaryBoxRewardLevelConfigState {
            total_reward_amount: 0,
            total_open_box_count: 0,
        };
        ordinary_box_reward_level_config_state_map.insert(*k, ordinary_box_reward_level_config_state);
    });

    let mut random_box_reward_rule_config_state_vec: Vec<RandomBoxRewardRuleConfigState> = vec![];
    for _ in 0..msg.box_config.random_box_reward_rule_config.len() {
        let random_box_reward_rule_config_state = RandomBoxRewardRuleConfigState {
            total_reward_amount: 0,
            total_open_box_count: 0,
        };
        random_box_reward_rule_config_state_vec.push(random_box_reward_rule_config_state);
    }

    let reward_config_state = BoxRewardConfigState {
        ordinary_total_reward_amount: 0,
        ordinary_total_open_box_count: 0,
        ordinary_box_reward_level_config_state: ordinary_box_reward_level_config_state_map,
        random_total_reward_amount: 0,
        random_total_open_box_count: 0,
        random_box_reward_rule_config_state: random_box_reward_rule_config_state_vec,
        global_reward_claim_index: 0,
        global_reward_claim_total_amount: 0
    };

    let box_reward_config = msg.box_config.clone();

    set_reward_config(deps.storage, &reward_config)?;
    set_box_reward_config(deps.storage, &box_reward_config)?;
    set_box_reward_config_state(deps.storage, &reward_config_state)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateRewardConfig { gov, nft_contract } => {
            update_reward_config(deps, info, gov, nft_contract)
        }
        ExecuteMsg::UpdateBoxRewardConfig { box_reward_token, box_open_time } => {
            update_box_reward_config(deps, info, box_reward_token, box_open_time)
        }
        ExecuteMsg::OpenBlindBox { token_ids } => {
            open_blind_box(deps, env, info, token_ids)
        }
        ExecuteMsg::UserClaimNftReward { token_ids } => {
            user_claim_nft_reward(deps, info, token_ids)
        }
        ExecuteMsg::Receive(msg) => {
            receive_cw20(deps, env, info, msg)
        }
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryAllConfigAndState { .. } => {
            to_binary(&query_all_config_and_state(deps)?)
        }
        QueryMsg::QueryBoxOpenInfo { token_ids } => {
            to_binary(&query_box_open_info(deps, token_ids)?)
        }
        QueryMsg::TestRandom { token_ids } => {
            to_binary(&test_random(env, token_ids)?)
        }
        QueryMsg::QueryBoxClaimableInfos { token_ids } => {
            to_binary(&query_box_claimable_infos(deps, token_ids)?)
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
    use crate::state::BoxRewardConfig;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        // Set up the necessary data for testing
        let env = mock_env();
        let info = mock_info("creator", &vec![]);
        let msg = InstantiateMsg {
            gov: Some(Addr::unchecked("gov")),
            nft_contract: Addr::unchecked("nft_contract"),
            box_config: BoxRewardConfig {
                box_reward_token: Addr::unchecked("box_reward_token"),
                box_open_time: 0,
                ordinary_box_reward_level_config: HashMap::new(),
                random_in_box_level_index: 0,
                random_box_reward_rule_config: vec![],
                box_reward_distribute_addr: Addr::unchecked("box_reward_distribute_addr"),
                box_reward_distribute_rule_type: "".to_string(),
                global_reward_total_amount: 0,
            },
        };
        // Positive test case
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "instantiate");
    }
}