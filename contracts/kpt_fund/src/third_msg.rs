use cosmwasm_schema::{cw_serde};
use cosmwasm_std::Uint128;


#[cw_serde]
pub enum KptExecuteMsg{
    Mint { recipient: String, amount: Uint128 },
    Burn { user: String, amount: Uint128 },
}

#[cw_serde]
pub enum VeKptExecuteMsg{
    Mint { recipient: String, amount: Uint128 },
    /// Implements CW20. Burn is a base message to destroy tokens forever
    Burn { user: String, amount: Uint128 },
}