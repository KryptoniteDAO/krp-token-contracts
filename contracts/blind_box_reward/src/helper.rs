use cosmwasm_std::StdResult;

pub const BASE_RATE_12: u128 = 1000000000000u128;

pub fn is_empty_str(str: &str) -> bool {
    str.trim().is_empty()
}


pub fn calc_claim_index(claim_amount: u128, total_amount: u128, claim_index: u128) -> StdResult<u128> {
    if total_amount == 0 || claim_amount == 0 {
        return Ok(claim_index);
    }
    let res = claim_index + claim_amount * BASE_RATE_12 / total_amount;
    Ok(res)
}

pub fn calc_claim_amount(global_claim_index: u128, user_amount: u128, user_claim_index: u128) -> StdResult<u128> {
    if user_amount == 0 || global_claim_index == 0 || user_claim_index > global_claim_index {
        return Ok(0);
    }
    let res = user_amount * (global_claim_index - user_claim_index) / BASE_RATE_12;
    Ok(res)
}