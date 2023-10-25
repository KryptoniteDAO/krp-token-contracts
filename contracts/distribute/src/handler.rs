use crate::error::ContractError;
use crate::helper::BASE_RATE_12;
use crate::msg::{RuleConfigMsg, UpdateRuleConfigMsg};
use crate::querier::query_claimable_info;
use crate::state::{
    check_rule_config_exist, read_distribute_config, read_rule_config, read_rule_config_state,
    store_distribute_config, store_rule_config, store_rule_config_state, RuleConfig,
    RuleConfigState,
};
use cosmwasm_std::{
    attr, to_binary, Addr, Binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError,
    Uint128, WasmMsg,
};

pub fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rule_type: String,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    let claim_user = info.sender;
    // check rule type owner
    let rule_config = read_rule_config(deps.storage, &rule_type)?;
    if rule_config.rule_owner.ne(&claim_user) {
        return Err(ContractError::Unauthorized {});
    }
    let block_time = env.block.time.seconds();
    let distribute_config = read_distribute_config(deps.storage)?;
    let mut rule_config_state = read_rule_config_state(deps.storage, &rule_type)?;

    let total_can_claimed_amount = rule_config.rule_total_amount - rule_config_state.claimed_amount;
    // check if can claim
    if total_can_claimed_amount == 0u128 {
        return Err(ContractError::NoMoreAmountClaim {});
    }

    if rule_config.lock_start_time != 0 && rule_config.lock_start_time > block_time {
        return Err(ContractError::NotStartClaimTimeError {});
    }

    let claimable_info = query_claimable_info(deps.as_ref(), env.clone(), rule_type.clone())?;

    let claim_amount = claimable_info.can_claim_amount;

    if claimable_info.release_amount > 0 && !rule_config_state.is_start_release {
        //update the start release state
        rule_config_state.is_start_release = true;
        rule_config_state.released_amount += claimable_info.release_amount;
    }

    rule_config_state.claimed_amount += claim_amount;
    if rule_config_state.claimed_amount > rule_config.rule_total_amount {
        return Err(ContractError::AmountClaimOverTotal(
            rule_config_state.claimed_amount.clone(),
            rule_config.rule_total_amount.clone(),
        ));
    }

    store_rule_config_state(deps.storage, &rule_type, &rule_config_state)?;

    let mut cosmos_msgs = vec![];
    if claim_amount > 0u128 {
        // send the claim amount to user
        let seilor_mint_msg = seilor::msg::ExecuteMsg::Mint {
            recipient: claim_user.clone().to_string(),
            amount: Uint128::from(claim_amount.clone()),
            contract: Option::from(claim_user.clone().to_string()),
            msg,
        };
        let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: distribute_config.distribute_token.to_string(),
            msg: to_binary(&seilor_mint_msg)?,
            funds: vec![],
        });
        cosmos_msgs.push(mint_msg);
    }

    Ok(Response::new()
        .add_attributes(vec![
            ("action", "claim"),
            ("claim_user", claim_user.as_str()),
            ("claim_amount", claim_amount.to_string().as_str()),
        ])
        .add_messages(cosmos_msgs))
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    distribute_token: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut distribute_config = read_distribute_config(deps.storage)?;
    if info.sender != distribute_config.gov {
        return Err(ContractError::Unauthorized {});
    }
    let mut attrs = vec![
        attr("action", "update_config"),
        attr("sender", info.sender.to_string()),
    ];
    if let Some(distribute_token) = distribute_token {
        deps.api.addr_validate(distribute_token.clone().as_str())?;
        distribute_config.distribute_token = distribute_token.clone();
        attrs.push(attr("distribute_token", distribute_token.to_string()));
    }
    store_distribute_config(deps.storage, &distribute_config)?;
    Ok(Response::new().add_attributes(attrs))
}

pub fn update_rule_config(
    deps: DepsMut,
    info: MessageInfo,
    update_rule_msg: UpdateRuleConfigMsg,
) -> Result<Response, ContractError> {
    let distribute_config = read_distribute_config(deps.storage)?;
    if info.sender != distribute_config.gov {
        return Err(ContractError::Unauthorized {});
    }
    let mut rule_config = read_rule_config(deps.storage, &update_rule_msg.rule_type)?;
    let mut attrs = vec![
        attr("action", "update_rule_config"),
        attr("sender", info.sender.to_string()),
    ];

    let rule_name = update_rule_msg.rule_name;
    if let Some(rule_name) = rule_name {
        rule_config.rule_name = rule_name.clone();
        attrs.push(attr("rule_name", rule_name.to_string()));
    }

    let rule_owner = update_rule_msg.rule_owner;
    if let Some(rule_owner) = rule_owner {
        rule_config.rule_owner = rule_owner.clone();
        attrs.push(attr("rule_owner", rule_owner.to_string()));
    }

    store_rule_config(deps.storage, &update_rule_msg.rule_type, &rule_config)?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn add_rule_config(
    deps: DepsMut,
    info: MessageInfo,
    rule_type: String,
    rule_msg: RuleConfigMsg,
) -> Result<Response, ContractError> {
    let mut distribute_config = read_distribute_config(deps.storage)?;
    if distribute_config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    let exist_rule_config = check_rule_config_exist(deps.storage, &rule_type)?;

    if exist_rule_config {
        return Err(ContractError::Std(StdError::generic_err(
            "rule config already exist",
        )));
    }

    if rule_msg.rule_total_amount <= 0 {
        return Err(ContractError::Std(StdError::generic_err(
            "rule total amount must be greater than zero",
        )));
    }

    let rule_total_amount = rule_msg.rule_total_amount + distribute_config.rules_total_amount;

    if rule_total_amount > distribute_config.total_amount {
        return Err(ContractError::Std(StdError::generic_err(
            "rule total amount over distribute token amount",
        )));
    }

    let end_linear_release_time =
        rule_msg.start_linear_release_time + rule_msg.unlock_linear_release_time;
    let linear_release_per_second = rule_msg.unlock_linear_release_amount * BASE_RATE_12
        / u128::from(rule_msg.unlock_linear_release_time);

    distribute_config.rules_total_amount = rule_total_amount;

    store_distribute_config(deps.storage, &distribute_config)?;

    let rule_config = RuleConfig {
        rule_name: rule_msg.rule_name,
        rule_owner: rule_msg.rule_owner,
        rule_total_amount: rule_msg.rule_total_amount,
        start_release_amount: rule_msg.start_release_amount,
        lock_start_time: rule_msg.lock_start_time,
        lock_end_time: rule_msg.lock_end_time,
        start_linear_release_time: rule_msg.start_linear_release_time,
        end_linear_release_time,
        unlock_linear_release_amount: rule_msg.unlock_linear_release_amount,
        unlock_linear_release_time: rule_msg.unlock_linear_release_time,
        linear_release_per_second,
    };
    store_rule_config(deps.storage, &rule_type, &rule_config)?;

    let rule_config_state = RuleConfigState {
        is_start_release: false,
        claimed_amount: 0u128,
        released_amount: 0u128,
        last_claim_linear_release_time: 0,
    };
    store_rule_config_state(deps.storage, &rule_type, &rule_config_state)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "add_rule_config"),
        ("rule_type", rule_type.as_str()),
    ]))
}

pub fn set_gov(deps: DepsMut, info: MessageInfo, gov: Addr) -> Result<Response, ContractError> {
    let mut distribute_config = read_distribute_config(deps.storage)?;
    if distribute_config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    deps.api.addr_validate(gov.clone().as_str())?;

    distribute_config.new_gov = Some(gov.clone());
    store_distribute_config(deps.storage, &distribute_config)?;
    Ok(Response::new().add_attributes(vec![
        attr("action", "set_gov"),
        attr("gov", gov.to_string()),
    ]))
}

pub fn accept_gov(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut distribute_config = read_distribute_config(deps.storage)?;
    if distribute_config.new_gov.is_none() {
        return Err(ContractError::NoNewGov {});
    }
    if info.sender != distribute_config.new_gov.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    distribute_config.gov = info.sender.clone();
    distribute_config.new_gov = None;
    store_distribute_config(deps.storage, &distribute_config)?;
    Ok(Response::new().add_attributes(vec![
        attr("action", "accept_gov"),
        attr("gov", distribute_config.gov.to_string()),
        attr("new_gov", ""),
    ]))
}
