use cosmwasm_std::StdError;
use cw721_base::ContractError;

pub const BASE_RATE: u128 = 1000000u128;

pub fn is_empty_address(address: &str) -> bool {
    address.trim().is_empty()
}



pub fn check_referral_code_role(referral_code: String) -> Result<(), ContractError> {
    if referral_code.is_empty() {
        return Err(ContractError::Std(StdError::generic_err("referral_code is empty")));
    }

    if referral_code.chars().any(|c| !c.is_ascii_alphanumeric()) {
        return Err(ContractError::Std(StdError::generic_err("referral_code contains non-alphanumeric characters")));
    }

    if referral_code.len() > 12 {
        return Err(ContractError::Std(StdError::generic_err("referral_code is too long (max 12 characters)")));
    }

    Ok(())
}