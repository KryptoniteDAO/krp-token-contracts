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
}