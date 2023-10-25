use crate::error::ContractError;
use crate::handler::{accept_gov, burn, mint, set_gov, set_minters, update_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_is_minter, query_vote_config};
use crate::state::{store_vote_config, VoteConfig};
use crate::ve_querier::{checkpoints, get_past_votes, get_votes, num_checkpoints};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Uint128;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use cw20::MinterResponse;
use cw20_base::contract::instantiate as cw20_instantiate;
use cw20_base::contract::{
    execute_update_marketing, execute_upload_logo, query_balance, query_download_logo,
    query_marketing_info, query_minter, query_token_info,
};
use cw20_base::enumerable::query_all_accounts;
use cw20_base::msg::{InstantiateMarketingInfo, InstantiateMsg as Cw20InstantiateMsg};

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-ve-seilor";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut cw20_instantiate_msg: Cw20InstantiateMsg = msg.cw20_init_msg;

    if cw20_instantiate_msg.initial_balances.len() > 0 {
        return Err(ContractError::UnableInitialBalances {});
    }

    let gov = msg.gov.unwrap_or_else(|| info.sender.clone());

    cw20_instantiate_msg.mint = Some(MinterResponse {
        minter: env.contract.address.to_string(),
        cap: Some(msg.max_supply.into()),
    });

    cw20_instantiate_msg.marketing = if let Some(marketing) = cw20_instantiate_msg.marketing {
        Some(InstantiateMarketingInfo {
            project: marketing.project,
            description: marketing.description,
            logo: marketing.logo,
            marketing: Some(
                marketing
                    .marketing
                    .unwrap_or_else(|| gov.clone().to_string()),
            ),
        })
    } else {
        return Err(ContractError::MissingMarketingInfo {});
    };

    let ins_res = cw20_instantiate(deps.branch(), env, info, cw20_instantiate_msg);
    if ins_res.is_err() {
        return Err(ContractError::Std(StdError::generic_err(
            ins_res.err().unwrap().to_string(),
        )));
    }

    let vote_config = VoteConfig {
        max_supply: msg.max_supply,
        fund: Addr::unchecked(""),
        gov,
        max_minted: Uint128::from(msg.max_minted),
        total_minted: Uint128::from(0u128),
        new_gov: None,
    };

    store_vote_config(deps.storage, &vote_config)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { max_minted, fund } => {
            update_config(deps, info, max_minted, fund)
        }
        ExecuteMsg::SetMinters {
            contracts,
            is_minter,
        } => set_minters(deps, info, contracts, is_minter),
        ExecuteMsg::Mint { recipient, amount } => {
            let recipient = deps.api.addr_validate(&recipient)?;
            mint(deps, env, info, recipient, amount.u128())
        }

        // we override these from cw20
        ExecuteMsg::Burn { user, amount } => {
            let user = deps.api.addr_validate(&user)?;
            burn(deps, env, info, user, amount.u128())
        }
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => {
            let res = execute_update_marketing(deps, env, info, project, description, marketing);
            if res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(
                    res.err().unwrap().to_string(),
                )));
            }
            Ok(res.unwrap())
        }
        ExecuteMsg::UploadLogo(logo) => {
            let res = execute_upload_logo(deps, env, info, logo);
            if res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(
                    res.err().unwrap().to_string(),
                )));
            }
            Ok(res.unwrap())
        }
        ExecuteMsg::SetGov { gov } => set_gov(deps, info, gov),
        ExecuteMsg::AcceptGov {} => accept_gov(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // custom queries
        QueryMsg::VoteConfig {} => to_binary(&query_vote_config(deps)?),
        QueryMsg::IsMinter { address } => {
            to_binary(&query_is_minter(deps, deps.api.addr_validate(&address)?)?)
        }
        QueryMsg::Checkpoints { account, pos } => to_binary(&checkpoints(deps, account, pos)?),
        QueryMsg::NumCheckpoints { account } => to_binary(&num_checkpoints(deps, account)?),
        // QueryMsg::Delegates { account } => to_binary(&delegates(deps, account)?),
        QueryMsg::GetVotes { account } => to_binary(&get_votes(deps, account)?),
        QueryMsg::GetPastVotes {
            account,
            block_number,
        } => to_binary(&get_past_votes(deps, env, account, block_number)?),
        // QueryMsg::GetPastTotalSupply { block_number } => {
        //     to_binary(&get_past_total_supply(deps, env, block_number)?)
        // }

        // inherited from cw20-base
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        // QueryMsg::Allowance { owner, spender } => {
        //     to_binary(&query_allowance(deps, owner, spender)?)
        // }
        // QueryMsg::AllAllowances {
        //     owner,
        //     start_after,
        //     limit,
        // } => to_binary(&query_owner_allowances(deps, owner, start_after, limit)?),
        // QueryMsg::AllSpenderAllowances {
        //     spender,
        //     start_after,
        //     limit,
        // } => to_binary(&query_spender_allowances(
        //     deps,
        //     spender,
        //     start_after,
        //     limit,
        // )?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
        QueryMsg::MarketingInfo {} => to_binary(&query_marketing_info(deps)?),
        QueryMsg::DownloadLogo {} => to_binary(&query_download_logo(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
