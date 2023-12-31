// Function to add a new lock setting
// function addLockSetting(esLBRLockSetting memory setting) external onlyOwner {
// esLBRLockSettings.push(setting);
// }

use crate::state::{
    read_boost_config, read_user_lock_status, store_boost_config, store_user_lock_status,
    VeSeilorLockSetting,
};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};

pub fn add_lock_setting(
    deps: DepsMut,
    info: MessageInfo,
    duration: Uint128,
    mining_boost: Uint128,
) -> StdResult<Response> {
    let mut config = read_boost_config(deps.storage)?;
    if info.sender != config.gov {
        return Err(StdError::generic_err("unauthorized"));
    }

    let mut ve_seilor_lock_settings = config.ve_seilor_lock_settings;
    ve_seilor_lock_settings.push(VeSeilorLockSetting {
        duration,
        mining_boost,
    });
    config.ve_seilor_lock_settings = ve_seilor_lock_settings;
    store_boost_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "add_lock_setting"),
        ("duration", duration.to_string().as_str()),
        ("mining_boost", mining_boost.to_string().as_str()),
    ]))
}

pub fn set_gov(deps: DepsMut, info: MessageInfo, gov: Addr) -> StdResult<Response> {
    let mut config = read_boost_config(deps.storage)?;
    if info.sender != config.gov {
        return Err(StdError::generic_err("unauthorized"));
    }
    deps.api.addr_validate(gov.clone().as_str())?;

    config.new_gov = Some(gov.clone());
    store_boost_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "set_gov"),
        ("gov", gov.to_string().as_str()),
    ]))
}

pub fn accept_gov(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut config = read_boost_config(deps.storage)?;
    if config.new_gov.is_none() {
        return Err(StdError::generic_err("no new gov"));
    }
    if info.sender != config.new_gov.unwrap() {
        return Err(StdError::generic_err("unauthorized"));
    }

    config.gov = info.sender.clone();
    config.new_gov = None;
    store_boost_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "accept_gov"),
        ("gov", config.gov.to_string().as_str()),
    ]))
}
// Function to set the user's lock status
pub fn set_lock_status(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    index: usize,
) -> StdResult<Response> {
    let sender = info.sender;
    let config = read_boost_config(deps.storage)?;
    let setting: VeSeilorLockSetting = config.ve_seilor_lock_settings[index].clone();
    let mut user_status = read_user_lock_status(deps.storage, sender.clone())?;
    if user_status
        .unlock_time
        .gt(&Uint128::from(env.block.time.seconds()))
    {
        if user_status.duration.gt(&setting.duration) {
            return Err(StdError::generic_err("Your lock-in period has not ended, and the term can only be extended, not reduced."));
        }
    }
    user_status.unlock_time = Uint128::from(env.block.time.seconds())
        .checked_add(setting.duration)
        .unwrap();
    user_status.duration = setting.duration;
    user_status.mining_boost = setting.mining_boost;
    store_user_lock_status(deps.storage, sender.clone(), &user_status)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "set_lock_status"),
        ("user", sender.to_string().as_str()),
        ("index", index.to_string().as_str()),
    ]))
}
