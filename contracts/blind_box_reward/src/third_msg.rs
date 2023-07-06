use cosmwasm_schema::{cw_serde,QueryResponses};


#[cw_serde]
#[derive(QueryResponses)]
pub enum BlindBoxQueryMsg{
    #[returns(BlindBoxInfoResponse)]
    QueryBlindBoxInfo { token_id: String },
}


#[cw_serde]
pub struct BlindBoxInfoResponse {
    pub level_index: u8,
    pub price: u128,
    pub block_number: u64,
    pub is_reward_box: bool,
}

#[cw_serde]
pub enum DistributeExecuteMsg {
    Claim {
        rule_type: String,
    },
}


#[cw_serde]
pub struct QueryClaimableInfoResponse {
    pub can_claim_amount: u128,
    pub release_amount: u128,
    pub linear_release_amount: u128,
}
#[cw_serde]
#[derive(QueryResponses)]
pub enum DistributeQueryMsg {
    #[returns(QueryClaimableInfoResponse)]
    QueryClaimableInfo {
        rule_type: String,
    },
}