use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw20::Cw20ReceiveMsg;
use crate::state::{BoxRewardConfig, BoxRewardConfigState, RewardConfig};


#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub nft_contract: Addr,
    pub box_config: BoxRewardConfig,
}

#[cw_serde]
pub struct AllConfigAndStateResponse {
    pub config: RewardConfig,
    pub box_config: BoxRewardConfig,
    pub box_state: BoxRewardConfigState,
}

#[cw_serde]
pub struct QueryBoxClaimableInfoResponse {
    pub next_reward_claim_index: u128,
    pub total_claimable_distribute_amount: u128,
    pub total_claimable_amount: u128,
    pub box_claimable_infos: Vec<BoxClaimableAmountInfoResponse>,
}
#[cw_serde]
pub struct BoxClaimableAmountInfoResponse {
    pub token_id: String,
    pub claimable_amount: u128,
}

#[cw_serde]
pub struct BoxOpenInfoResponse {
    pub token_id: String,
    pub open_user: Addr,
    pub open_reward_amount: u128,
    pub open_box_time: u64,
    pub is_random_box: bool,
    pub is_reward_box: bool,
    pub box_level_index: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Receives a message of type [`Cw20ReceiveMsg`]
    Receive(Cw20ReceiveMsg),
    UpdateRewardConfig {
        gov: Option<Addr>,
        nft_contract: Option<Addr>,
    },
    UpdateBoxRewardConfig {
        box_reward_token: Option<Addr>,
        box_open_time: Option<u64>,
    },
    OpenBlindBox {
        token_ids: Vec<String>,
    },
    UserClaimNftReward {
        token_ids: Vec<String>,
    },
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AllConfigAndStateResponse)]
    QueryAllConfigAndState {},
    #[returns(Vec < BoxOpenInfoResponse >)]
    QueryBoxOpenInfo {
        token_ids: Vec<String>,
    },
    #[returns(std::collections::HashMap < u64, u64 >)]
    TestRandom {
        token_ids: Vec<String>,
    },
    #[returns(QueryBoxClaimableInfoResponse)]
    QueryBoxClaimableInfos {
        token_ids: Vec<String>,
    },
}


/// This structure describes a CW20 hook message.
#[cw_serde]
pub enum Cw20HookMsg {
    ClaimNftReward {
        token_ids: Vec<String>
    },
}

#[cw_serde]
pub struct MigrateMsg {}