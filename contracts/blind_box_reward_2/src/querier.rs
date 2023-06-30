use cosmwasm_std::{Deps, Env, QueryRequest, StdResult, to_binary, WasmQuery};
use cw721::{Cw721QueryMsg, TokensResponse};
use crate::msg::{BlindBoxConfigResponse, RewardLevelConfigResponse, RewardTokenConfigResponse, UserClaimableRewardDetailResponse, UserClaimableRewardsResponse};
use crate::state::{get_blind_box_reward_token_config_keys, read_blind_box_reward_config, read_blind_box_reward_token_config};
use crate::third_msg::{BlindBoxInfoResponse, BlindBoxQueryMsg};

pub fn query_user_claim_rewards(deps: Deps, env: Env, user_addr: String) -> StdResult<Vec<UserClaimableRewardsResponse>> {
    let mut result = vec![];
    let reward_tokens = get_blind_box_reward_token_config_keys(deps.storage);
    let config = read_blind_box_reward_config(deps.storage)?;
    let current_time = env.block.time.seconds();
    for reward_token_std in reward_tokens {
        let reward_token = reward_token_std?;
        let tokens_response: TokensResponse = deps.querier.clone().query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: reward_token.to_string(),
            msg: to_binary(&Cw721QueryMsg::Tokens { owner: user_addr.clone(), start_after: None, limit: None })?,
        }))?;

        if tokens_response.tokens.is_empty() {
            continue;
        } else {
            let reward_token_config = read_blind_box_reward_token_config(deps.storage, reward_token.to_string());
            let mut claimable_reward_details: Vec<UserClaimableRewardDetailResponse> = vec![];
            if current_time.clone() < reward_token_config.claimable_time {
                continue;
            } else {
                let mut total_reward_amount = 0u128;
                for token_id in tokens_response.tokens {
                    let nft_level_info: BlindBoxInfoResponse = deps.querier.clone().query(&QueryRequest::Wasm(WasmQuery::Smart {
                        contract_addr: config.nft_contract.to_string(),
                        msg: to_binary(&BlindBoxQueryMsg::QueryBlindBoxInfo { token_id })?,
                    }))?;
                    if nft_level_info.block_number > 0u64 {
                        let level_index = nft_level_info.level_index;
                        let claimable_reward = reward_token_config.reward_levels[level_index.clone() as usize].reward_amount;
                        total_reward_amount += claimable_reward.clone();
                        claimable_reward_details.push(UserClaimableRewardDetailResponse {
                            level_index,
                            claimable_reward,
                        });
                    }
                }
                result.push(UserClaimableRewardsResponse {
                    reward_token: reward_token.to_string(),
                    claimable_reward: total_reward_amount,
                    claimable_reward_details,
                });
            }
        }
    }

    Ok(result)
}


pub fn query_blind_box_config(deps: Deps) -> StdResult<BlindBoxConfigResponse> {
    let config = read_blind_box_reward_config(deps.storage)?;

    let reward_tokens = get_blind_box_reward_token_config_keys(deps.storage);
    let mut reward_token_map_msgs = vec![];
    for reward_token_std in reward_tokens {
        let reward_token = reward_token_std?;
        let reward_token_config = read_blind_box_reward_token_config(deps.storage, reward_token.clone());
        let mut reward_levels = vec![];
        for reward_level in reward_token_config.reward_levels {
            reward_levels.push(RewardLevelConfigResponse {
                reward_amount: reward_level.reward_amount,
                level_total_claimed_amount: reward_level.level_total_claimed_amount,
            });
        }
        reward_token_map_msgs.push(RewardTokenConfigResponse {
            reward_token: reward_token.to_string(),
            total_reward_amount: reward_token_config.total_reward_amount,
            total_claimed_amount: reward_token_config.total_claimed_amount,
            total_claimed_count: reward_token_config.total_claimed_count,
            claimable_time: reward_token_config.claimable_time,
            reward_levels,
        });
    }
    Ok(BlindBoxConfigResponse {
        gov: config.gov,
        nft_contract: config.nft_contract,
        reward_token_map_msgs,
    })
}
