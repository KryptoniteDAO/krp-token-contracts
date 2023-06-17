use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


// Define a struct for the lock settings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VeKptLockSetting {
    pub duration: Uint128,
    pub mining_boost: Uint128,
}

// Define a struct for the user's lock status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LockStatus {
    pub unlock_time: Uint128,
    pub duration: Uint128,
    pub mining_boost: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BoostConfig {
    pub gov: Addr,
    pub ve_kpt_lock_settings: Vec<VeKptLockSetting>,
}

const BOOST_CONFIG: Item<BoostConfig> = Item::new("boost_config");

const USER_LOCK_STATUS: Map<Addr, LockStatus> = Map::new("user_lock_status");

pub fn store_boost_config(storage: &mut dyn Storage, boost_config: &BoostConfig) -> StdResult<()> {
    BOOST_CONFIG.save(storage, boost_config)?;
    Ok(())
}

pub fn read_boost_config(storage: &dyn Storage) -> StdResult<BoostConfig> {
    BOOST_CONFIG.load(storage)
}

pub fn store_user_lock_status(storage: &mut dyn Storage, user: Addr, lock_status: &LockStatus) -> StdResult<()> {
    USER_LOCK_STATUS.save(storage, user, lock_status)?;
    Ok(())
}

pub fn read_user_lock_status(storage: &dyn Storage, user: Addr) -> StdResult<LockStatus> {
    Ok(USER_LOCK_STATUS.load(storage, user).unwrap_or(LockStatus {
        unlock_time: Uint128::zero(),
        duration: Uint128::zero(),
        mining_boost: Uint128::zero(),
    }))
}



