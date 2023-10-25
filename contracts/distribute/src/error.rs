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

    #[error("NotStartClaimTimeError")]
    NotStartClaimTimeError {},

    #[error("NoMoreAmountClaim")]
    NoMoreAmountClaim {},

    #[error("AmountClaimOverTotal,claimed_amount:{0},rule_total_amount:{1}")]
    AmountClaimOverTotal(u128, u128),

    #[error("No New Gov")]
    NoNewGov {},
}
