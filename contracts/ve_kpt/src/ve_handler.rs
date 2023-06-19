use std::ops::Sub;
use cosmwasm_std::{Addr, attr, Binary, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw20_base::allowances::{execute_burn_from, execute_send_from, execute_transfer_from};
use cw20_base::contract::{execute_burn, execute_mint, execute_send, execute_transfer, query_balance, query_token_info};
use crate::state::{Checkpoint, read_checkpoints_default, read_delegates_default, read_vote_config, store_checkpoints, store_delegates};


/**
 * @dev Snapshots the totalSupply after it has been increased.
 */

pub fn ve_mint(mut deps: DepsMut, env: Env, recipient: Addr, amount: u128) -> StdResult<Response> {
    // mint tokens by contracts
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };
    let res_cw20 = execute_mint(deps.branch(), env.clone(), sub_info, recipient.clone().to_string(), Uint128::from(amount));
    if res_cw20.is_err() {
        return Err(StdError::generic_err(res_cw20.err().unwrap().to_string()));
    }

    let total_supply = query_token_info(deps.branch().as_ref())?.total_supply.u128();
    let max_supply = read_vote_config(deps.branch().as_ref().storage)?.max_supply;

    if total_supply > max_supply {
        return Err(StdError::generic_err("total supply risks overflowing votes"));
    }

    _write_checkpoint(&mut deps, env.block.height, recipient.clone(), _add, amount);

    let res = Response::new()
        .add_attributes(res_cw20.unwrap().attributes)
        .add_attributes(vec![
        ("action", "ve_mint"),
    ]);
    Ok(res)
}

/**
 * @dev Snapshots the totalSupply after it has been decreased.
 */
pub fn ve_burn(mut deps: DepsMut, env: Env, owner: Addr, amount: u128) -> StdResult<Response> {
    let sub_info = MessageInfo {
        sender: owner.clone(),
        funds: vec![],
    };
    let res_cw20 = execute_burn(deps.branch(), env.clone(), sub_info, Uint128::from(amount));
    if res_cw20.is_err() {
        return Err(StdError::generic_err(res_cw20.err().unwrap().to_string()));
    }

    _write_checkpoint(&mut deps, env.block.height, owner.clone(), _subtract, amount);

    let res = Response::new()
        .add_attributes(res_cw20.unwrap().attributes)
        .add_attributes(vec![
        ("action", "ve_burn"),
    ]);
    Ok(res)

}

/**
 * @dev Snapshots the totalSupply after it has been decreased.
 */
#[allow(dead_code)]
pub fn ve_burn_from(mut deps: DepsMut, env: Env, info: MessageInfo, owner: Addr, amount: u128) -> StdResult<Response> {
    let sub_info = MessageInfo {
        sender: info.sender.clone(),
        funds: vec![],
    };
    let res_cw20 = execute_burn_from(deps.branch(), env.clone(), sub_info, owner.clone().to_string(), Uint128::from(amount));
    if res_cw20.is_err() {
        return Err(StdError::generic_err(res_cw20.err().unwrap().to_string()));
    }

    _write_checkpoint(&mut deps, env.block.height, owner.clone(), _subtract, amount);

    let res = Response::new()
        .add_attributes(res_cw20.unwrap().attributes)
        .add_attributes(vec![
        ("action", "ve_burn_from"),
    ]);
    Ok(res)
}

/**
 * @dev Moves voting power from one address to another.
 */
#[allow(dead_code)]
pub fn ve_transfer(mut deps: DepsMut, env: Env, info: MessageInfo, recipient: Addr, amount: u128) -> StdResult<Response> {
    let sender = info.clone().sender;
    let sub_info = MessageInfo {
        sender: sender.clone(),
        funds: vec![],
    };

    let res_cw20 = execute_transfer(deps.branch(), env.clone(), sub_info, recipient.clone().to_string(), Uint128::from(amount));
    if res_cw20.is_err() {
        return Err(StdError::generic_err(res_cw20.err().unwrap().to_string()));
    }

    delegate(deps.branch(), env.clone(), info.clone(), sender.clone())?;
    delegate(deps.branch(), env.clone(), info.clone(), recipient.clone())?;
    _move_voting_power(deps, env, sender.clone(), recipient.clone(), amount)?;

    let res = Response::new()
        .add_attributes(res_cw20.unwrap().attributes)
        .add_attributes(vec![
        ("action", "ve_transfer"),
    ]);
    Ok(res)
}

/**
 * @dev Moves voting power from one address to another.
 */
#[allow(dead_code)]
pub fn ve_transfer_from(mut deps: DepsMut, env: Env, info: MessageInfo, owner: Addr, recipient: Addr, amount: u128) -> StdResult<Response> {
    let sub_info = MessageInfo {
        sender: info.clone().sender,
        funds: vec![],
    };

    let res_cw20 = execute_transfer_from(deps.branch(), env.clone(), sub_info, owner.clone().to_string(), recipient.clone().to_string(), Uint128::from(amount));
    if res_cw20.is_err() {
        return Err(StdError::generic_err(res_cw20.err().unwrap().to_string()));
    }

    delegate(deps.branch(), env.clone(), info.clone(), owner.clone())?;
    delegate(deps.branch(), env.clone(), info.clone(), recipient.clone())?;
    _move_voting_power(deps, env, owner.clone(), recipient.clone(), amount)?;

    let res = Response::new()
        .add_attributes(res_cw20.unwrap().attributes)
        .add_attributes(vec![
        ("action", "ve_transfer_from"),
    ]);
    Ok(res)
}

/**
 * @dev Moves voting power from one address to another.
 */
#[allow(dead_code)]
pub fn ve_send(mut deps: DepsMut, env: Env, info: MessageInfo, contract: Addr, amount: u128, msg: Binary) -> StdResult<Response> {
    let sender = info.clone().sender;
    let sub_info = MessageInfo {
        sender: sender.clone(),
        funds: vec![],
    };

    let res_cw20 = execute_send(deps.branch(), env.clone(), sub_info, contract.clone().to_string(), Uint128::from(amount), msg);
    if res_cw20.is_err() {
        return Err(StdError::generic_err(res_cw20.err().unwrap().to_string()));
    }

    delegate(deps.branch(), env.clone(), info.clone(), sender.clone())?;
    delegate(deps.branch(), env.clone(), info.clone(), contract.clone())?;
    _move_voting_power(deps, env, sender.clone(), contract.clone(), amount)?;

    let res = Response::new()
        .add_attributes(res_cw20.unwrap().attributes)
        .add_attributes(vec![
        ("action", "ve_send"),
    ]);
    Ok(res)
}

/**
 * @dev Moves voting power from one address to another.
 */
#[allow(dead_code)]
pub fn ve_send_from(mut deps: DepsMut, env: Env, info: MessageInfo, owner: Addr, contract: Addr, amount: u128, msg: Binary) -> StdResult<Response> {
    let sub_info = MessageInfo {
        sender: info.clone().sender,
        funds: vec![],
    };

    let res_cw20 = execute_send_from(deps.branch(), env.clone(), sub_info, owner.clone().to_string(), contract.clone().to_string(), Uint128::from(amount), msg);
    if res_cw20.is_err() {
        return Err(StdError::generic_err(res_cw20.err().unwrap().to_string()));
    }

    delegate(deps.branch(), env.clone(), info.clone(), owner.clone())?;
    delegate(deps.branch(), env.clone(), info.clone(), contract.clone())?;
    _move_voting_power(deps, env, owner.clone(), contract.clone(), amount)?;

    let res = Response::new()
        .add_attributes(res_cw20.unwrap().attributes)
        .add_attributes(vec![
        ("action", "ve_send_from"),
    ]);
    Ok(res)
}


/**
 * @dev Delegate votes from the sender to `delegatee`.
 */


pub fn delegate(deps: DepsMut, env: Env, info: MessageInfo, delegatee: Addr) -> StdResult<Response> {
    let sender = deps.api.addr_validate(info.sender.as_str())?;
    _delegate(deps, env, sender, delegatee)
}


/**
 * @dev Change delegation for `delegator` to `delegatee`.
 *
 */

fn _delegate(
    deps: DepsMut,
    env: Env,
    delegator: Addr,
    delegatee: Addr,
) -> StdResult<Response> {
    let current_delegate = read_delegates_default(deps.storage, delegator.clone())?;
    let delegator_balance = query_balance(deps.as_ref(), delegator.clone().to_string())?.balance.u128();
    store_delegates(deps.storage, delegator.clone(), &delegatee)?;
    _move_voting_power(deps, env, current_delegate.clone(), delegatee.clone(), delegator_balance)?;
    let res = Response::new().add_attributes(vec![
        ("action", "delegate"),
        ("delegator", delegator.as_str()),
        ("current_delegate", current_delegate.as_str()),
        ("delegatee", delegatee.as_str()),
    ]);
    Ok(res)
}

fn _move_voting_power(
    mut deps: DepsMut,
    env: Env,
    src: Addr,
    dst: Addr,
    amount: u128,
) -> StdResult<Response> {
    if src != dst && amount > 0 {
        let block_number = env.block.height;

        let mut attrs = vec![];
        attrs.push(attr("action", "move_voting_power"));


        if !is_empty_address(src.clone().as_str()) {
            let (src_old_weight, src_new_weight) = _write_checkpoint(&mut deps, block_number, src.clone(), _subtract, amount.clone());
            attrs.push(attr("src", src.as_str()));
            attrs.push(attr("src_old_weight", &src_old_weight.to_string()));
            attrs.push(attr("src_new_weight", &src_new_weight.to_string()));
        }

        if !is_empty_address(dst.clone().as_str()) {
            let (dst_old_weight, dst_new_weight) = _write_checkpoint(&mut deps, block_number, dst.clone(), _add, amount.clone());
            attrs.push(attr("dst", dst.as_str()));
            attrs.push(attr("dst_old_weight", &dst_old_weight.to_string()));
            attrs.push(attr("dst_new_weight", &dst_new_weight.to_string()));
        }

        let res = Response::new().add_attributes(attrs);
        Ok(res)
    } else {
        Ok(Response::default())
    }
}

pub fn is_empty_address(address: &str) -> bool {
    address.trim().is_empty()
}

fn _write_checkpoint(
    deps: &mut DepsMut,
    block_number: u64,
    account: Addr,
    op: fn(Uint128, Uint128) -> u128,
    delta: u128,
) -> (u128, u128) {
    let mut check_points = read_checkpoints_default(deps.storage, account.clone()).unwrap();
    let pos = check_points.len();
    let old_checkpoint = if pos == 0 {
        Checkpoint { from_block: 0, votes: 0 }
    } else {
        check_points[pos.sub(1usize)].clone()
    };
    let old_weight = old_checkpoint.votes;
    let new_weight = op(Uint128::from(old_weight), Uint128::from(delta));
    if pos > 0 && old_checkpoint.from_block == 0 {
        check_points[pos.sub(1usize)].votes = new_weight;
    } else {
        check_points.push(Checkpoint { from_block: block_number, votes: new_weight });
    }
    let _ = store_checkpoints(deps.storage, account, &check_points);

    (old_weight, new_weight)

}

fn _add(a: Uint128, b: Uint128) -> u128 {
    a.checked_add(b).unwrap().u128()
}

fn _subtract(a: Uint128, b: Uint128) -> u128 {
    a.checked_sub(b).unwrap().u128()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, StdError,
    };


    #[test]
    fn test_delegate() -> Result<(), Box<dyn std::error::Error>> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        println!("info: {:?}", info);
        let delegatee = Addr::unchecked("delegatee");
        // Test positive case
        let res = delegate(deps.as_mut(), env.clone(), info.clone(), delegatee.clone())?;
        assert_eq!(res.attributes.len(), 4);
        assert_eq!(res.attributes[0], ("action", "delegate"));
        assert_eq!(res.attributes[1], ("delegator", "sender"));
        assert_eq!(res.attributes[2], ("current_delegate", ""));
        assert_eq!(res.attributes[3], ("delegatee", "delegatee"));
        // Test negative case with invalid sender address
        let info = mock_info("", &[]);
        let res = delegate(deps.as_mut(), env.clone(), info.clone(), delegatee.clone());
        match res {
            Err(StdError::GenericErr { msg, .. }) => {
                assert_eq!(msg, "Invalid input: human address too short for this mock implementation (must be >= 3).")
            }
            _ => panic!("unexpected error"),
        }
        Ok(())
    }

    #[test]
    fn test_write_checkpoint() {
        let mut deps = mock_dependencies();
        let block_number = 1;
        let account = Addr::unchecked("account");
        let delta = 100;
        // Test positive case
        let (old_weight, new_weight) = _write_checkpoint(&mut deps.as_mut(), block_number, account.clone(), _add, delta);
        assert_eq!(old_weight, 0);
        assert_eq!(new_weight, 100);

        let (old_weight, new_weight) = _write_checkpoint(&mut deps.as_mut(), block_number, account.clone(), _subtract, delta);
        assert_eq!(old_weight, 100);
        assert_eq!(new_weight, 0);
    }
}