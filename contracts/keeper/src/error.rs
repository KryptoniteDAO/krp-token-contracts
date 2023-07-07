use cosmwasm_std::{OverflowError, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Distribute contract unauthorized calling function:{0}, params:{1}")]
    Unauthorized(String, String),

    
    #[error("Distribute rewards less than threshold: {0}")]
    DistributeRewardsLessThanThreshold(Uint128),

    #[error("Invalid input")]
    InvalidInput {},

}
