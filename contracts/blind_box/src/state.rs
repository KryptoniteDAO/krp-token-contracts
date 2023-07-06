use std::collections::HashMap;
use cosmwasm_std::{Addr, Order, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_storage::{Bucket, ReadonlyBucket};
use cw_utils::calc_range_start;


const BLIND_BOX_CONFIG: Item<BlindBoxConfig> = Item::new("blind_box_config");
// token_id => BlindBoxInfo
const BLIND_BOX_INFO: Map<String, BlindBoxInfo> = Map::new("blind_box_info");

const REFERRAL_REWARD_CONFIG: Item<ReferralRewardConfig> = Item::new("referral_reward_config");

const REFERRAL_REWARD_TOTAL_STATE: Item<ReferralRewardTotalState> = Item::new("referral_reward_total_state");


// user_addr => UserInfo
const USER_INFO: Map<&Addr, UserInfo> = Map::new("user_info");

// user referral_code => user_addr
const USER_REFERRAL_CODE: Map<String, Addr> = Map::new("user_referral_code");


// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlindBoxLevel {
    pub level_index: u8,
    pub price: u128,
    pub mint_total_count: u128,
    pub minted_count: u128,
    pub received_total_amount: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlindBoxInfo {
    pub level_index: u8,
    pub price: u128,
    pub block_number: u64,
    pub is_reward_box: bool,
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
    pub end_mint_time: u64,
    pub level_infos: Vec<BlindBoxLevel>,
    pub receiver_price_addr: Addr,
    pub can_transfer_time: u64,
    pub inviter_reward_box_contract: Addr,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReferralLevelRewardBoxConfig {
    pub recommended_quantity: u128,
    // reward_level => reward_count
    pub reward_box: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReferralLevelConfig {
    // pub min_reward_count: u32,
    // pub max_reward_count: u32,
    pub min_referral_total_amount: u128,
    pub max_referral_total_amount: u128,
    pub inviter_reward_rate: u128,
    pub invitee_discount_rate: u128,
    pub reward_box_config: ReferralLevelRewardBoxConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReferralRewardConfig {
    //referral_level => ReferralLevelConfig
    pub referral_level_config: HashMap<u8, ReferralLevelConfig>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReferralRewardTotalState {
    pub referral_reward_total_base_amount: u128,
    //referral_level => reward_count
    pub referral_reward_box_total: HashMap<u8, u32>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo {
    pub referral_code: String,
    pub inviter_referral_code: String,
    pub inviter: Addr,
    pub invitee_count: u32,
    pub last_mint_discount_rate: u128,
    pub current_reward_level: u8,
    // pub user_reward_token_type: String,
    pub user_reward_total_base_amount: u128,
    pub user_referral_total_amount: u128,
    // referral_level => invitee count
    pub user_referral_level_count: HashMap<u8, u32>,
    // referral_level => reward_box_count
    pub user_reward_box: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InviterReferralRecord {
    pub invitee: Addr,
    pub token_ids: Vec<String>,
    pub mint_time: u64,
    pub reward_level: u8,
    pub invitee_index: u32,
    pub mint_box_level_index: u8,
    pub mint_price: u128,
    pub mint_pay_amount: u128,
    pub reward_to_inviter_base_amount: u128,
}


pub fn store_referral_reward_config(storage: &mut dyn Storage, referral_reward_config: &ReferralRewardConfig) -> StdResult<()> {
    REFERRAL_REWARD_CONFIG.save(storage, referral_reward_config)
}

pub fn read_referral_reward_config(storage: &dyn Storage) -> StdResult<ReferralRewardConfig> {
    REFERRAL_REWARD_CONFIG.load(storage)
}

pub fn store_referral_reward_total_state(storage: &mut dyn Storage, referral_reward_total_state: &ReferralRewardTotalState) -> StdResult<()> {
    REFERRAL_REWARD_TOTAL_STATE.save(storage, referral_reward_total_state)
}

pub fn read_referral_reward_total_state(storage: &dyn Storage) -> ReferralRewardTotalState {
    REFERRAL_REWARD_TOTAL_STATE.load(storage).unwrap_or(ReferralRewardTotalState {
        referral_reward_total_base_amount: 0,
        referral_reward_box_total: Default::default(),
    })
}

pub fn store_user_info(storage: &mut dyn Storage, user_addr: &Addr, user_info: &UserInfo) -> StdResult<()> {
    USER_INFO.save(storage, user_addr, user_info)
}

pub fn read_user_info(storage: &dyn Storage, user_addr: &Addr) -> UserInfo {
    USER_INFO.load(storage, user_addr).unwrap_or(UserInfo {
        referral_code: Default::default(),
        inviter_referral_code: Default::default(),
        inviter: Addr::unchecked(""),
        invitee_count: 0,
        last_mint_discount_rate: 0,
        current_reward_level: 0,
        // user_reward_token_type: Default::default(),
        user_reward_total_base_amount: 0,
        user_referral_total_amount: 0,
        user_referral_level_count: Default::default(),
        user_reward_box: Default::default(),
    })
}


pub fn store_blind_box_config(storage: &mut dyn Storage, blind_box_config: &BlindBoxConfig) -> StdResult<()> {
    BLIND_BOX_CONFIG.save(storage, blind_box_config)
}

pub fn read_blind_box_config(storage: &dyn Storage) -> StdResult<BlindBoxConfig> {
    BLIND_BOX_CONFIG.load(storage)
}

pub fn store_blind_box_info(storage: &mut dyn Storage, token_id: String, blind_box_info: &BlindBoxInfo) -> StdResult<()> {
    BLIND_BOX_INFO.save(storage, token_id, blind_box_info)
}


pub fn read_blind_box_info(storage: &dyn Storage, token_id: String) -> BlindBoxInfo {
    BLIND_BOX_INFO.load(storage, token_id).unwrap_or(BlindBoxInfo {
        level_index: 0,
        price: 0,
        block_number: 0,
        is_reward_box: false
    })
}

pub fn store_user_referral_code(storage: &mut dyn Storage, user_referral_code: String, user_addr: &Addr) -> StdResult<()> {
    USER_REFERRAL_CODE.save(storage, user_referral_code, user_addr)
}

pub fn read_user_referral_code(storage: &dyn Storage, user_referral_code: String) -> Addr {
    USER_REFERRAL_CODE.load(storage, user_referral_code).unwrap_or(Addr::unchecked(""))
}

pub fn store_inviter_record_elem(storage: &mut dyn Storage, inviter: &Addr, record: &InviterReferralRecord) -> StdResult<()> {
    let binding = inviter.clone().to_string();
    let namespace = binding.as_bytes();
    let start_token_id = record.token_ids.first().unwrap();
    let invitee_key = start_token_id.as_bytes();
    let inviter_namespace = &format!("{}_check", binding);
    let invitee = record.clone().invitee;

    Bucket::new(storage, inviter_namespace.as_bytes()).save(invitee.as_bytes(), &true)?;
    Bucket::new(storage, namespace).save(invitee_key, record)
}

pub fn check_invitee_existence(storage: &dyn Storage, inviter: &Addr, invitee: &Addr) -> StdResult<bool> {
    let binding = inviter.to_string();
    let inviter_namespace = &format!("{}_check", binding);
    let record_bucket: ReadonlyBucket<bool> = ReadonlyBucket::new(storage, inviter_namespace.as_bytes());
    let maybe_record = record_bucket.may_load(invitee.as_bytes())?;
    Ok(maybe_record.is_some())
}

pub fn read_inviter_records(
    storage: &dyn Storage,
    inviter: &Addr,
    start_after: Option<Addr>,
    limit: Option<u32>,
) -> StdResult<Vec<InviterReferralRecord>> {
    let binding = inviter.clone().to_string();
    let namespace = binding.as_bytes();

    let record_bucket: ReadonlyBucket<InviterReferralRecord> =
        ReadonlyBucket::new(storage, namespace);

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = calc_range_start(start_after);
    record_bucket
        .range(start.as_deref(), None, Order::Descending)
        .take(limit)
        .map(|elem| {
            let (_, v) = elem?;
            Ok(InviterReferralRecord {
                invitee: v.invitee,
                token_ids: v.token_ids,
                mint_time: v.mint_time,
                reward_level: v.reward_level,
                invitee_index: v.invitee_index,
                mint_box_level_index: v.mint_box_level_index,
                mint_price: v.mint_price,
                mint_pay_amount: v.mint_pay_amount,
                reward_to_inviter_base_amount: v.reward_to_inviter_base_amount,
            })
        })
        .collect()
}



