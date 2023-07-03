use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use crate::state::{InviterRewardConfig, InviterRewardConfigState};


#[cw_serde]
pub struct CalCanMintRewardBoxResponse {
    pub can_mint_box_quantity: u32,
    pub total_reward_box_quantity: u32,
    pub minted_reward_box_quantity: u32,
}

#[cw_serde]
pub struct CalCanClaimRewardTokenResponse {
    pub can_claim_token_quantity: u128,
    pub total_reward_token_quantity: u128,
    pub claimed_reward_token_quantity: u128,
}

#[cw_serde]
pub struct ConfigAndStateResponse {
    pub config: InviterRewardConfig,
    pub state: InviterRewardConfigState,
}

#[cw_serde]
pub struct UpdateInviterRewardConfigMsg {
    pub gov: Option<Addr>,
    pub nft_contract: Option<Addr>,
    pub reward_native_token: Option<String>,
    pub start_mint_box_time: Option<u64>,
    pub end_mint_box_time: Option<u64>,
    pub start_claim_token_time: Option<u64>,
    pub end_claim_token_time: Option<u64>,
}


#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub nft_contract: Addr,
    pub reward_native_token: String,
    pub start_mint_box_time: u64,
    pub end_mint_box_time: u64,
    pub start_claim_token_time: u64,
    pub end_claim_token_time: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    MintRewardBox {
        level_index: u8,
        mint_num: u32,
    },
    ClaimRewardToken {
        amount: Option<u128>
    },
    UpdateConfig {
        update_msg: UpdateInviterRewardConfigMsg,
    },
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigAndStateResponse)]
    QueryAllConfigAndState {},
    #[returns(CalCanMintRewardBoxResponse)]
    CalCanMintRewardBox {
        user: Addr,
        level_index: u8,
    },
    #[returns(CalCanClaimRewardTokenResponse)]
    CalCanClaimRewardToken {
        user: Addr,
    },
    #[returns(crate::state::InviterOptDetail)]
    QueryInviterDetail {
        user: Addr,
    },
}

#[cw_serde]
pub struct MigrateMsg {}