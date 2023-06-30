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

    #[error("NotClaimable")]
    NotClaimable {},

    #[error("ClaimableTimeNotSet")]
    ClaimableTimeNotSet {},

    #[error("InvalidInput")]
    InvalidInput {},

    #[error("ArithmeticOverflow")]
    ArithmeticOverflow {},

    #[error("ArithmeticUnderflow")]
    ArithmeticUnderflow {},

    #[error("BoxAlreadyOpen")]
    BoxAlreadyOpen {},

}
