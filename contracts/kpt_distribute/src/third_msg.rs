use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Binary, Uint128};

#[cw_serde]
pub enum KptExecuteMsg {
    Mint { recipient: String, amount: Uint128, contract: Option<String>, msg: Option<Binary> },
}