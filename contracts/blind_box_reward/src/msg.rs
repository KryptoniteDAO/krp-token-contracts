use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;


#[cw_serde]
pub struct UserClaimableRewardDetailResponse {
    pub level_index: u8,
    pub claimable_reward: u128,
}

#[cw_serde]
pub struct UserClaimableRewardsResponse {
    pub reward_token: String,
    pub claimable_reward: u128,
    pub claimable_reward_details: Vec<UserClaimableRewardDetailResponse>,
}

#[cw_serde]
pub struct RewardLevelConfigMsg {
    pub reward_amount: Option<u128>,
}

#[cw_serde]
pub struct RewardTokenConfigMsg {
    pub reward_token: String,
    pub total_reward_amount: Option<u128>,
    pub claimable_time: Option<u64>,
    pub reward_levels: Option<Vec<RewardLevelConfigMsg>>,
}


#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub nft_contract: Addr,
    pub reward_token_map_msgs: Vec<RewardTokenConfigMsg>,
}

#[cw_serde]
pub struct BlindBoxConfigResponse {
    pub gov: Addr,
    pub nft_contract: Addr,
    pub reward_token_map_msgs: Vec<RewardTokenConfigResponse>,
}

#[cw_serde]
pub struct RewardLevelConfigResponse {
    pub reward_amount: u128,
    pub level_total_claimed_amount: u128,
}

#[cw_serde]
pub struct RewardTokenConfigResponse {
    pub reward_token: String,
    pub total_reward_amount: u128,
    pub total_claimed_amount: u128,
    pub total_claimed_count: u128,
    pub claimable_time: u64,
    pub reward_levels: Vec<RewardLevelConfigResponse>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateBlindBoxConfig {
        gov: Option<Addr>,
        nft_contract: Option<Addr>,
    },
    UpdateBlindBoxRewardTokenConfig {
        reward_token: Addr,
        total_reward_amount: u128,
        claimable_time: u64,
    },
    UpdateRewardTokenRewardLevel {
        reward_token: Addr,
        reward_level: u8,
        reward_amount: u128,
    },
    ClaimReward {
        recipient: Option<Addr>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec < UserClaimableRewardsResponse >)]
    QueryUserClaimRewards {
        user_addr: Addr,
    },
    #[returns(BlindBoxConfigResponse)]
    QueryBlindBoxConfig {},
}


#[cw_serde]
pub struct MigrateMsg {}