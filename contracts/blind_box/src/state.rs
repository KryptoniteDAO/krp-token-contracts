use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlindBoxLevel {
    pub price: u128,
    pub mint_total_count: u128,
    pub minted_count: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlindBoxInfo {
    pub level_index: u8,
    pub price: u128,
    pub block_number: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlindBoxConfig {
    pub nft_base_url: String,
    pub nft_uri_suffix: String,
    pub gov: Addr,
    pub price_token: String,
    pub token_id_prefix: String,
    pub token_id_index: u128,
    pub start_mint_time: u64,
    pub level_infos: Vec<BlindBoxLevel>,
    pub receiver_price_addr: Addr,
}


const BLIND_BOX_CONFIG: Item<BlindBoxConfig> = Item::new("blind_box_config");
// token_id => BlindBoxInfo
const BLIND_BOX_INFO: Map<String, BlindBoxInfo> = Map::new("blind_box_info");

pub fn store_blind_box_config(storage: &mut dyn Storage, blind_box_config: &BlindBoxConfig) -> StdResult<()> {
    BLIND_BOX_CONFIG.save(storage, blind_box_config)?;
    Ok(())
}

pub fn read_blind_box_config(storage: &dyn Storage) -> StdResult<BlindBoxConfig> {
    BLIND_BOX_CONFIG.load(storage)
}

pub fn store_blind_box_info(storage: &mut dyn Storage, token_id: String, blind_box_info: &BlindBoxInfo) -> StdResult<()> {
    BLIND_BOX_INFO.save(storage, token_id, blind_box_info)?;
    Ok(())
}


pub fn read_blind_box_info(storage: &dyn Storage, token_id: String) -> BlindBoxInfo {
    BLIND_BOX_INFO.load(storage, token_id).unwrap_or(BlindBoxInfo {
        level_index: 0,
        price: 0,
        block_number: 0,
    })
}