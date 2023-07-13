use crate::state::{RuleConfig, RuleConfigState};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary};
use std::collections::HashMap;

#[cw_serde]
pub struct QueryClaimableInfoResponse {
    pub can_claim_amount: u128,
    pub release_amount: u128,
    pub linear_release_amount: u128,
}

#[cw_serde]
pub struct QueryRuleInfoResponse {
    pub rule_config: RuleConfig,
    pub rule_config_state: RuleConfigState,
}

#[cw_serde]
pub struct QueryConfigResponse {
    pub gov: Addr,
    pub total_amount: u128,
    pub distribute_token: Addr,
    pub rules_total_amount: u128,
}

#[cw_serde]
pub struct UpdateRuleConfigMsg {
    pub rule_type: String,
    pub rule_name: Option<String>,
    pub rule_owner: Option<Addr>,
}

#[cw_serde]
pub struct RuleConfigMsg {
    pub rule_name: String,
    pub rule_owner: Addr,
    pub rule_total_amount: u128,
    pub start_release_amount: u128,
    pub lock_start_time: u64,
    pub lock_end_time: u64,
    pub start_linear_release_time: u64,
    pub unlock_linear_release_amount: u128,
    pub unlock_linear_release_time: u64,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub total_amount: u128,
    pub distribute_token: Addr,
    pub rule_configs_map: HashMap<String, RuleConfigMsg>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Claim {
        rule_type: String,
        msg: Option<Binary>,
    },
    UpdateConfig {
        gov: Option<Addr>,
        distribute_token: Option<Addr>,
    },
    UpdateRuleConfig {
        update_rule_msg: UpdateRuleConfigMsg,
    },
    AddRuleConfig {
        rule_type: String,
        rule_msg: RuleConfigMsg,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryClaimableInfoResponse)]
    QueryClaimableInfo { rule_type: String },
    #[returns(QueryRuleInfoResponse)]
    QueryRuleInfo { rule_type: String },
    #[returns(QueryConfigResponse)]
    QueryConfig {},
}

#[cw_serde]
pub struct MigrateMsg {}
