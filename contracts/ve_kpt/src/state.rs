use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdError, StdResult, Storage, Uint128};

use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Checkpoint {
    pub from_block: u64,
    pub votes: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoteInfo {
    pub total_supply_checkpoints: Vec<Checkpoint>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoteConfig {
    pub max_supply: u128,
    pub kpt_fund: Addr,
    pub gov: Addr,
    pub max_minted:Uint128,
    pub total_minted:Uint128,
}

// const DELEGATES: Map<Addr, Addr> = Map::new("delegates");
const CHECK_POINTS: Map<Addr, Vec<Checkpoint>> = Map::new("checkpoints");
const VOTE_INFO: Item<VoteInfo> = Item::new("vote_info");
const VOTE_CONFIG: Item<VoteConfig> = Item::new("vote_config");
const MINTERS: Map<Addr, bool> = Map::new("minters");


// pub fn store_delegates(storage: &mut dyn Storage, delegator: Addr, delegate: &Addr) -> StdResult<()> {
//     DELEGATES.save(storage, delegator, delegate)?;
//     Ok(())
// }

// pub fn read_delegates(storage: &dyn Storage, delegator: Addr) -> StdResult<Addr> {
//     DELEGATES.may_load(storage, delegator)?.ok_or_else(|| StdError::generic_err("Delegate not found"))
// }


// pub fn read_delegates_default(storage: &dyn Storage, delegator: Addr) -> StdResult<Addr> {
//     Ok(DELEGATES.may_load(storage, delegator)?.unwrap_or(Addr::unchecked("")))
// }

pub fn store_checkpoints(storage: &mut dyn Storage, delegator: Addr, checkpoints: &Vec<Checkpoint>) -> StdResult<()> {
    CHECK_POINTS.save(storage, delegator, checkpoints)?;
    Ok(())
}
//
// pub fn store_single_checkpoints(storage: &mut dyn Storage, delegator: Addr, checkpoint: &Checkpoint) -> Result<Vec<Checkpoint>, StdError> {
//     CHECK_POINTS.update(storage, delegator, |old| match old {
//         Some(mut checkpoints) => {
//             checkpoints.push(checkpoint.clone());
//             Ok(checkpoints)
//         }
//         None => {
//             let mut checkpoints = Vec::new();
//             checkpoints.push(checkpoint.clone());
//             Ok(checkpoints)
//         }
//     })
// }

// pub fn read_checkpoints(storage: &dyn Storage, delegator: Addr) -> StdResult<Vec<Checkpoint>> {
//     CHECK_POINTS.may_load(storage, delegator)?.ok_or_else(|| StdError::generic_err("Checkpoints not found"))
// }

pub fn read_checkpoints_default(storage: &dyn Storage, delegator: Addr) -> StdResult<Vec<Checkpoint>> {
    Ok(CHECK_POINTS.may_load(storage, delegator)?.unwrap_or_default())
}

// pub fn store_vote_info(storage: &mut dyn Storage, vote_info: &VoteInfo) -> StdResult<()> {
//     VOTE_INFO.save(storage, vote_info)?;
//     Ok(())
// }

// pub fn read_vote_info(storage: &dyn Storage) -> StdResult<VoteInfo> {
//     VOTE_INFO.may_load(storage)?.ok_or_else(|| StdError::generic_err("Vote info not found"))
// }

pub fn read_vote_info_default(storage: &dyn Storage) -> StdResult<VoteInfo> {
    Ok(VOTE_INFO.may_load(storage)?.unwrap_or(VoteInfo {
        total_supply_checkpoints: Vec::new()
    }))
}

pub fn store_vote_config(storage: &mut dyn Storage, vote_config: &VoteConfig) -> StdResult<()> {
    VOTE_CONFIG.save(storage, vote_config)?;
    Ok(())
}

pub fn read_vote_config(storage: &dyn Storage) -> StdResult<VoteConfig> {
    VOTE_CONFIG.may_load(storage)?.ok_or_else(|| StdError::generic_err("Vote config not found"))
}

pub fn store_minters(storage: &mut dyn Storage, minter: Addr, is_minter: &bool) -> StdResult<()> {
    MINTERS.save(storage, minter, is_minter)?;
    Ok(())
}

// pub fn read_minters(storage: &dyn Storage, minter: Addr) -> StdResult<bool> {
//     MINTERS.may_load(storage, minter)?.ok_or_else(|| StdError::generic_err("Minter not found"))
// }

pub fn is_minter(storage: &dyn Storage, minter: Addr) -> StdResult<bool> {
    Ok(MINTERS.may_load(storage, minter)?.unwrap_or(false))
}


