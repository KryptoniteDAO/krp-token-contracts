use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid input")]
    InvalidInput {},

    #[error("Unable initial balances")]
    UnableInitialBalances {},

    #[error("Missing marketing info")]
    MissingMarketingInfo {},
    #[error("No new gov")]
    NoNewGov {},
}
