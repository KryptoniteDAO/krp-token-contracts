use std::ops::{Add, Div, Sub};
use cosmwasm_std::{Addr, Deps, Env, Isqrt, StdError, StdResult};
use crate::msg::{DelegatesResponse, GetPastTotalSupplyResponse, GetPastVotesResponse, GetVotesResponse, NumCheckpointsResponse};
use crate::state::{Checkpoint, read_checkpoints_default, read_delegates_default, read_vote_info_default};


/**
 * @dev Get the `pos`-th checkpoint for `account`.
 */
pub fn checkpoints(deps: Deps, account: Addr, pos: u32) -> StdResult<Checkpoint> {
    let checkpoints = read_checkpoints_default(deps.storage, account)?;
    let _pos = pos as usize;
    if _pos >= checkpoints.len() {
        return Err(StdError::generic_err("Position out of range"));
    }
    Ok(checkpoints[_pos].clone())
}

/**
 * @dev Get number of checkpoints for `account`.
 */
pub fn num_checkpoints(deps: Deps, account: Addr) -> StdResult<NumCheckpointsResponse> {
    let check_points = read_checkpoints_default(deps.storage, account)?;
    Ok(NumCheckpointsResponse { num: check_points.len() })
}


/**
 * @dev Get the address `account` is currently delegating to.
 */
pub fn delegates(deps: Deps, account: Addr) -> StdResult<DelegatesResponse> {
    let delegate = read_delegates_default(deps.storage, account)?;
    Ok(DelegatesResponse { delegate })
}

/**
 * @dev Gets the current votes balance for `account`
 */
pub fn get_votes(deps: Deps, account: Addr) -> StdResult<GetVotesResponse> {
    let check_points = read_checkpoints_default(deps.storage, account)?;
    if check_points.is_empty() {
        return Ok(GetVotesResponse { votes: 0u128 });
    }
    let votes = check_points.last().unwrap().votes;

    Ok(GetVotesResponse { votes })
}


/**
 * @dev Retrieve the number of votes for `account` at the end of `blockNumber`.
 *
 * Requirements:
 *
 * - `blockNumber` must have been already mined
 */
pub fn get_past_votes(deps: Deps, env: Env, account: Addr, block_number: u64) -> StdResult<GetPastVotesResponse> {
    if block_number >= env.block.height {
        return Err(StdError::generic_err("Block not yet mined"));
    }
    let check_points = read_checkpoints_default(deps.storage, account)?;
    let votes = _check_points_lookup(check_points, block_number);
    Ok(GetPastVotesResponse { votes })
}


/**
 * @dev Retrieve the `totalSupply` at the end of `blockNumber`. Note, this value is the sum of all balances.
 * It is but NOT the sum of all the delegated votes!
 *
 * Requirements:
 *
 * - `blockNumber` must have been already mined
 */
pub fn get_past_total_supply(deps: Deps, env: Env, block_number: u64) -> StdResult<GetPastTotalSupplyResponse> {
    if block_number >= env.block.height {
        return Err(StdError::generic_err("Block not yet mined"));
    }
    let vote_info = read_vote_info_default(deps.storage)?;
    let total_supply = _check_points_lookup(vote_info.total_supply_checkpoints, block_number);
    Ok(GetPastTotalSupplyResponse { total_supply })
}

/**
 * @dev Lookup a value in a list of (sorted) checkpoints.
 */
fn _check_points_lookup(check_points: Vec<Checkpoint>, block_number: u64) -> u128 {
    // We run a binary search to look for the earliest checkpoint taken after `blockNumber`.
    //
    // Initially we check if the block is recent to narrow the search range.
    // During the loop, the index of the wanted checkpoint remains in the range [low-1, high).
    // With each iteration, either `low` or `high` is moved towards the middle of the range to maintain the invariant.
    // - If the middle checkpoint is after `blockNumber`, we look in [low, mid)
    // - If the middle checkpoint is before or equal to `blockNumber`, we look in [mid+1, high)
    // Once we reach a single value (when low == high), we've found the right checkpoint at the index high-1, if not
    // out of bounds (in which case we're looking too far in the past and the result is 0).
    // Note that if the latest checkpoint available is exactly for `blockNumber`, we end up with an index that is
    // past the end of the array, so we technically don't find a checkpoint after `blockNumber`, but it works out
    // the same.

    let length = check_points.len();
    let mut low = 0usize;
    let mut high = length;
    if length > 5 {
        let mid = length - length.isqrt();
        if check_points[mid].from_block > block_number {
            high = mid;
        } else {
            low = mid.add(1usize);
        }
    }
    while low < high {
        let mid = (low + high).div(2usize);
        if check_points[mid].from_block > block_number {
            high = mid;
        } else {
            low = mid.add(1usize);
        }
    }
    if high == 0 {
        return 0;
    }
    check_points[high.sub(1usize)].votes
}


#[cfg(test)]
mod tests {
    use std::ops::Sub;
    use super::*;
    #[test]
    fn test_check_points_lookup() {
        // Positive test case
        let check_points = vec![
            Checkpoint { from_block: 0, votes: 100 },
            Checkpoint { from_block: 100, votes: 200 },
            Checkpoint { from_block: 200, votes: 300 },
            Checkpoint { from_block: 300, votes: 400 },
        ];
        let pos = check_points.len();
        let s :usize = 1usize;
        println!("check_points: {:?}", check_points[pos.sub(s)]);
        assert_eq!(_check_points_lookup(check_points.clone(), 50), 100);
        assert_eq!(_check_points_lookup(check_points.clone(), 150), 200);
        assert_eq!(_check_points_lookup(check_points.clone(), 250), 300);
        assert_eq!(_check_points_lookup(check_points.clone(), 350), 400);

    }
}
