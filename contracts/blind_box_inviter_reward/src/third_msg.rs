use std::collections::HashMap;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;


#[cw_serde]
pub struct UserInfoResponse {
    pub referral_code: String,
    pub inviter_referral_code: String,
    pub inviter: Addr,
    pub invitee_count: u32,
    pub last_mint_discount_rate: u128,
    pub current_reward_level: u8,
    pub user_reward_token_type: String,
    pub user_reward_total_base_amount: u128,
    pub user_referral_total_amount: u128,
    // referral_level => invitee count
    pub user_referral_level_count: HashMap<u8, u32>,
    // referral_level => reward_box_count
    pub user_reward_box: HashMap<u8, u32>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum BlindBoxQueryMsg {
    #[returns(UserInfoResponse)]
    GetUserInfo {
        user: Addr,
    },
}

#[cw_serde]
pub enum BlindBoxExecuteMsg {
    DoInviterRewardMint {
        inviter: Addr,
        level_index: u8,
        mint_num: u32,
    },
}
