use cosmwasm_std::{Addr, Uint128};
use cosmwasm_schema::{cw_serde,QueryResponses};
use cw20::Logo;

use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub cw20_init_msg: Cw20InstantiateMsg,

    pub max_supply: u128,
    // default msg.sender
    pub gov: Option<Addr>,
    pub max_minted: u128,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig{
        max_minted: Option<Uint128>,
        fund: Option<Addr>,
        gov: Option<Addr>,
    },
    SetMinters {
        contracts: Vec<Addr>,
        is_minter: Vec<bool>
    },
    Mint { recipient: String, amount: Uint128 },
    /// Implements CW20. Burn is a base message to destroy tokens forever
    Burn { user: String, amount: Uint128 },

    /// Only with the "marketing" extension. If authorized, updates marketing metadata.
    /// Setting None/null for any of these will leave it unchanged.
    /// Setting Some("") will clear this field on the contract storage
    UpdateMarketing {
        /// A URL pointing to the project behind this token.
        project: Option<String>,
        /// A longer description of the token and it's utility. Designed for tooltips or such
        description: Option<String>,
        /// The address (if any) who can update this data structure
        marketing: Option<String>,
    },
    /// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
    UploadLogo(Logo),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(VoteConfigResponse)]
    VoteConfig{},
    #[returns(IsMinterResponse)]
    IsMinter { address: String },
    #[returns(CheckpointResponse)]
    Checkpoints {account: Addr, pos: u32},
    #[returns(NumCheckpointsResponse)]
    NumCheckpoints{account: Addr},
    // #[returns(DelegatesResponse)]
    // Delegates { account: Addr },
    #[returns(GetVotesResponse)]
    GetVotes{account:Addr},
    #[returns(GetPastVotesResponse)]
    GetPastVotes{account:Addr, block_number:u64},
    #[returns(GetPastTotalSupplyResponse)]
    GetPastTotalSupply{block_number:u64},

    /// Implements CW20. Returns the current balance of the given address, 0 if unset.
    #[returns(cw20::BalanceResponse)]
    Balance { address: String },
    /// Implements CW20. Returns metadata on the contract - name, decimals, supply, etc.
    #[returns(cw20::TokenInfoResponse)]
    TokenInfo {},
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    #[returns(cw20::MinterResponse)]
    Minter {},
    /// Implements CW20 "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    // #[returns(cw20::AllowanceResponse)]
    // Allowance { owner: String, spender: String },
    // /// Only with "enumerable" extension (and "allowances")
    // /// Returns all allowances this owner has approved. Supports pagination.
    // #[returns(cw20::AllAllowancesResponse)]
    // AllAllowances {
    //     owner: String,
    //     start_after: Option<String>,
    //     limit: Option<u32>,
    // },
    // /// Only with "enumerable" extension (and "allowances")
    // /// Returns all allowances this spender has been granted. Supports pagination.
    // #[returns(cw20::AllSpenderAllowancesResponse)]
    // AllSpenderAllowances {
    //     spender: String,
    //     start_after: Option<String>,
    //     limit: Option<u32>,
    // },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    #[returns(cw20::AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    #[returns(cw20::MarketingInfoResponse)]
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data is stored for this
    /// contract.
    #[returns(cw20::DownloadLogoResponse)]
    DownloadLogo {},
}

#[cw_serde]
pub struct GetPastTotalSupplyResponse {
    pub total_supply: u128,
}
#[cw_serde]
pub struct GetPastVotesResponse {
    pub votes: u128,
}
#[cw_serde]
pub struct GetVotesResponse {
    pub votes: u128,
}

#[cw_serde]
pub struct DelegatesResponse {
    pub delegate: Addr,
}

#[cw_serde]
pub struct NumCheckpointsResponse {
    pub num: usize,
}

#[cw_serde]
pub enum FundMsg {
    RefreshReward {
        account: Addr,
    },
}

#[cw_serde]
pub struct VoteConfigResponse {
    pub max_supply: u128,
    pub fund: Addr,
    pub gov: Addr,
    pub max_minted:Uint128,
    pub total_minted:Uint128,
}


#[cw_serde]
pub struct CheckpointResponse{
    pub from_block: u64,
    pub votes: u128,
}

#[cw_serde]
pub struct IsMinterResponse {
    pub is_minter: bool,
}



#[cw_serde]
pub struct MigrateMsg {}


