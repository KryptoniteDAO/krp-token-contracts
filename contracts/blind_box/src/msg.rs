use std::collections::HashMap;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Uint128};
use cw_utils::Expiration;
use crate::state::{ReferralLevelConfig, ReferralRewardTokenConfig};

#[cw_serde]
pub struct BlindBoxLevelMsg {
    pub price: u128,
    pub mint_total_count: u128,
}

#[cw_serde]
pub struct ReferralLevelRewardBoxConfigMsg {
    pub referral_level: u8,
    pub recommended_quantity: Option<u128>,
    // reward_level => reward_count
    pub reward_box: Option<HashMap<u8, u32>>,
}

#[cw_serde]
pub struct ReferralLevelConfigMsg {
    pub referral_level: u8,
    pub min_referral_total_amount: Option<u128>,
    pub max_referral_total_amount: Option<u128>,
    pub inviter_reward_rate: Option<u128>,
    pub invitee_discount_rate: Option<u128>,
}

#[cw_serde]
pub struct ReferralRewardTokenConfigMsg {
    pub reward_token_type: String,
    pub reward_token: Option<String>,
    pub conversion_ratio: Option<u128>,
}

#[cw_serde]
pub struct ReferralRewardConfigMsg {
    //reward_token_type => ReferralRewardTokenConfig
    pub reward_token_config: Option<HashMap<String, ReferralRewardTokenConfig>>,
    //referral_level => ReferralLevelConfig
    pub referral_level_config: HashMap<u8, ReferralLevelConfig>,
}

#[cw_serde]
pub struct BlindBoxConfigResponse {
    pub nft_base_url: String,
    pub nft_uri_suffix: String,
    pub gov: String,
    pub price_token: String,
    pub token_id_prefix: String,
    pub token_id_index: u128,
    pub start_mint_time: u64,
    pub level_infos: Vec<BlindBoxConfigLevelResponse>,
    pub receiver_price_addr: Addr,
    pub can_transfer_time: u64,
}

#[cw_serde]
pub struct BlindBoxConfigLevelResponse {
    pub level_index: u8,
    pub price: u128,
    pub mint_total_count: u128,
    pub minted_count: u128,
    pub received_total_amount: u128,
}

#[cw_serde]
pub struct BlindBoxInfoResponse {
    pub level_index: u8,
    pub price: u128,
    pub block_number: u64,
}


#[cw_serde]
pub struct ReferralLevelRewardBoxConfigResponse {
    pub recommended_quantity: u128,
    // reward_level => reward_count
    pub reward_box: HashMap<u8, u32>,
}

#[cw_serde]
pub struct ReferralLevelConfigResponse {
    pub min_referral_total_amount: u128,
    pub max_referral_total_amount: u128,
    pub inviter_reward_rate: u128,
    pub invitee_discount_rate: u128,
    pub reward_box_config: ReferralLevelRewardBoxConfigResponse,
}

#[cw_serde]
pub struct ReferralRewardTokenConfigResponse {
    pub reward_token: String,
    pub conversion_ratio: u128,
}

#[cw_serde]
pub struct ReferralRewardConfigResponse {
    //reward_token_type => ReferralRewardTokenConfig
    pub reward_token_config: HashMap<String, ReferralRewardTokenConfigResponse>,
    //referral_level => ReferralLevelConfig
    pub referral_level_config: HashMap<u8, ReferralLevelConfigResponse>,
    pub referral_reward_total_base_amount: u128,
    //referral_level => reward_count
    pub referral_reward_box_total: HashMap<u8, u32>,
}


#[cw_serde]
pub struct ReferralRewardTotalStateResponse {
    pub referral_reward_total_base_amount: u128,
    //referral_level => reward_count
    pub referral_reward_box_total: HashMap<u8, u32>,
}

#[cw_serde]
pub struct InviterReferralRecordResponse {
    pub invitee: Addr,
    pub token_ids: Vec<String>,
    pub mint_time: u64,
    pub reward_level: u8,
    pub invitee_index: u32,
    pub mint_box_level_index: u8,
    pub mint_price: u128,
    pub mint_pay_amount: u128,
    pub reward_to_inviter_base_amount: u128,
}

#[cw_serde]
pub struct CalMintInfoResponse {
    pub price: Uint128,
    pub paid_amount: Uint128,
    pub mint_discount_rate: Uint128,
    pub current_inviter_reward_level: Option<u8>,
    pub next_inviter_reward_level: Option<u8>,
    pub inviter: Option<Addr>,
}

#[cw_serde]
pub struct CheckReferralCodeResponse {
    pub exists: bool,
    pub user: Addr,
}
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
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,
    pub nft_base_url: String,
    pub nft_uri_suffix: String,
    pub gov: Option<Addr>,
    pub price_token: String,
    pub token_id_prefix: String,
    pub level_infos: Option<Vec<BlindBoxLevelMsg>>,
    pub start_mint_time: Option<u64>,
    pub receiver_price_addr: Addr,
    pub end_mint_time: Option<u64>,
    pub can_transfer_time: Option<u64>,
    pub referral_reward_config: Option<ReferralRewardConfigMsg>,
}


/// This is like Cw721ExecuteMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        nft_base_url: Option<String>,
        nft_uri_suffix: Option<String>,
        gov: Option<String>,
        price_token: Option<String>,
        token_id_prefix: Option<String>,
        start_mint_time: Option<u64>,
        receiver_price_addr: Option<Addr>,
        end_mint_time: Option<u64>,
        can_transfer_time: Option<u64>,
    },
    UpdateConfigLevel {
        index: u8,
        price: Option<u128>,
        mint_total_count: Option<u128>,
    },
    UpdateRewardTokenConfig {
        reward_token_type: String,
        reward_token: String,
        conversion_ratio: u128,
    },
    UpdateReferralLevelConfig {
        referral_level_config_msg: ReferralLevelConfigMsg,
    },
    UpdateReferralLevelBoxConfig {
        level_reward_box_config_msg: ReferralLevelRewardBoxConfigMsg,
    },
    CreateReferralInfo {
        referral_code: String,
        reward_token_type: String,
    },
    ModifyRewardTokenType {
        reward_token_type: String,
    },
    Mint { level_index: u8,mint_num :u128 , recipient: Option<String>, referral_code: Option<String> },
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft { recipient: String, token_id: String },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke { spender: String, token_id: String },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },

    // /// Mint a new NFT, can only be called by the contract minter
    // Mint(MintMsg<T>),

    /// Burn an NFT the sender has access to
    Burn { token_id: String },

}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Return the owner of the given token, error if token does not exist
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(cw721::ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(cw721::ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(cw721::OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(cw721::OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw721::NftInfoResponse < cw721_base::Extension >)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::AllNftInfoResponse < cw721_base::Extension >)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Return the minter
    #[returns(cw721_base::msg::MinterResponse)]
    Minter {},

    #[returns(BlindBoxConfigResponse)]
    QueryBlindBoxConfig {},

    #[returns(BlindBoxConfigLevelResponse)]
    QueryBlindBoxConfigLevel { index: u8 },

    #[returns(BlindBoxInfoResponse)]
    QueryBlindBoxInfo { token_id: String },

    #[returns(ReferralRewardConfigResponse)]
    QueryAllReferralRewardConfig {},

    #[returns(InviterReferralRecordResponse)]
    QueryInviterRecords {
        inviter: Addr,
        start_after: Option<Addr>,
        limit: Option<u32>,
    },
    #[returns(CalMintInfoResponse)]
    CalMintInfo {
        level_index: u8,
        mint_num: Uint128,
        referral_code: Option<String>,
    },
    #[returns(CheckReferralCodeResponse)]
    CheckReferralCode {
        referral_code: String,
    },
    #[returns(UserInfoResponse)]
    GetUserInfo {
        user: Addr,
    },
}


#[cw_serde]
pub struct MigrateMsg {}

