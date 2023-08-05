use cosmwasm_std::{OverflowError, StdError};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Failed to parse integer")]
    ParseInt(#[from] ParseIntError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("TreasureNotStart")]
    TreasureNotStart {},

    #[error("TreasureEnd")]
    TreasureEnd {},

    #[error("InvalidLockToken")]
    InvalidLockToken {},

    #[error("InvalidCw20HookMsg")]
    InvalidCw20HookMsg {},

    #[error("InsufficientLockFunds")]
    InsufficientLockFunds {},

    #[error("InsufficientIntegralFunds")]
    InsufficientIntegralFunds {},

    #[error("InvalidMintNum")]
    InvalidMintNum {},
}
