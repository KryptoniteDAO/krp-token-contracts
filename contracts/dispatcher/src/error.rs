use cosmwasm_std::{Addr, ConversionOverflowError, OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},
    #[error("No New Gov")]
    NoNewGov {},

    #[error("ConversionOverflow")]
    ConversionOverflow(#[from] ConversionOverflowError),

    #[error("InvalidTotalLockAmount")]
    InvalidTotalLockAmount {},
    #[error("InvalidTotalUnlockAmount")]
    InvalidTotalUnlockAmount {},

    #[error("RegretTimeIsOver")]
    RegretTimeIsOver {},

    #[error("RegretTokenReceiverNotSet")]
    RegretTokenReceiverNotSet {},

    #[error("ClaimTimeIsNotArrived")]
    ClaimTimeIsNotArrived {},
    #[error("InvalidStartLockPeriodTime")]
    InvalidStartLockPeriodTime {},

    #[error("RegretTimeNotStart")]
    RegretTimeNotStart {},

    #[error("UserAlreadyExists:{0}")]
    UserAlreadyExists(Addr),

    #[error("UserAmountIsZero:{0}")]
    UserAmountIsZero(Addr),

    #[error("UserUnlockAmountTooLarge:{0}")]
    UserUnlockAmountTooLarge(Addr),

    #[error("UserLockAmountTooLarge:{0}")]
    UserLockAmountTooLarge(Addr),

    #[error("UserAlreadyRegret:{0}")]
    UserAlreadyRegret(Addr),

    #[error("UserAlreadyClaimed:{0}")]
    UserAlreadyClaimed(Addr),

    #[error("UserNotExists:{0}")]
    UserNotExists(Addr),

    #[error("UserClaimLockAmountTooLarge:{0}")]
    UserClaimLockAmountTooLarge(Addr),

    #[error("UserClaimUnlockAmountTooLarge:{0}")]
    UserClaimUnlockAmountTooLarge(Addr),
    #[error("UserClaimAmountIsZero:{0}")]
    UserClaimAmountIsZero(Addr),

    #[error("Global Claim Lock Amount Too Large")]
    GlobalClaimLockAmountTooLarge {},
}
