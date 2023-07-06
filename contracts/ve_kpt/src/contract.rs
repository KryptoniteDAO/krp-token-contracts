use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, Addr, StdResult, StdError, Deps, Binary, to_binary};
use cosmwasm_std::Uint128;
use cw20::MinterResponse;
use cw20_base::contract::{execute_update_marketing, execute_upload_logo, query_balance, query_download_logo, query_marketing_info, query_minter, query_token_info};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::handler::{burn, mint, set_minters, update_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_is_minter, query_vote_config};
use crate::state::{store_vote_config, VoteConfig};
use crate::ve_querier::{checkpoints, get_past_total_supply, get_past_votes, get_votes, num_checkpoints};
use cw20_base::msg::{InstantiateMsg as Cw20InstantiateMsg, InstantiateMarketingInfo};
use cw20_base::contract::instantiate as cw20_instantiate;
use cw20_base::enumerable::{query_all_accounts};

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-ve-kpt";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut cw20_instantiate_msg: Cw20InstantiateMsg = msg.cw20_init_msg;

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
            marketing: Option::from(gov.clone().to_string()),
        })
    } else {
        None
    };


    let ins_res = cw20_instantiate(deps.branch(), env, info, cw20_instantiate_msg);
    if ins_res.is_err() {
        return Err(ContractError::Std(StdError::generic_err(ins_res.err().unwrap().to_string())));
    }

    let vote_config = VoteConfig {
        max_supply: msg.max_supply,
        kpt_fund: Addr::unchecked(""),
        gov,
        max_minted: Uint128::from(msg.max_minted),
        total_minted: Uint128::from(0u128),
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
        ExecuteMsg::UpdateConfig { max_minted, kpt_fund, gov } => {
            update_config(deps, info, max_minted, kpt_fund, gov)
        }
        ExecuteMsg::SetMinters { contracts, is_minter } => {
            set_minters(deps, info, contracts, is_minter)
        }
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
                return Err(ContractError::Std(StdError::generic_err(res.err().unwrap().to_string())));
            }
            Ok(res.unwrap())
        }
        ExecuteMsg::UploadLogo(logo) => {
            let res = execute_upload_logo(deps, env, info, logo);
            if res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(res.err().unwrap().to_string())));
            }
            Ok(res.unwrap())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // custom queries
        QueryMsg::VoteConfig {} => to_binary(&query_vote_config(deps)?),
        QueryMsg::IsMinter { address } => to_binary(&query_is_minter(deps, deps.api.addr_validate(&address)?)?),
        QueryMsg::Checkpoints { account, pos } => to_binary(&checkpoints(deps, account, pos)?),
        QueryMsg::NumCheckpoints { account } => to_binary(&num_checkpoints(deps, account)?),
        // QueryMsg::Delegates { account } => to_binary(&delegates(deps, account)?),
        QueryMsg::GetVotes { account } => to_binary(&get_votes(deps, account)?),
        QueryMsg::GetPastVotes { account, block_number } => to_binary(&get_past_votes(deps, env, account, block_number)?),
        QueryMsg::GetPastTotalSupply { block_number } => to_binary(&get_past_total_supply(deps, env, block_number)?),

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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, Response, Uint128};
    use crate::msg::{InstantiateMsg};
    use crate::state::{read_vote_config};
    use cw20_base::msg::{InstantiateMsg as Cw20InitMsg};
    use cw2::get_contract_version;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let max_supply = 1000000u128;
        let max_minted = 500000u128;
        let gov = Addr::unchecked("gov");
        let marketing = InstantiateMarketingInfo {
            project: Option::from("Test Project".to_string()),
            description: Option::from("Test Description".to_string()),
            logo: None,
            marketing: Some(gov.clone().to_string()),
        };
        let cw20_instantiate_msg = Cw20InitMsg {
            name: String::from("name"),
            symbol: String::from("symbol"),
            decimals: 6u8,
            initial_balances: vec![],
            mint: None,
            marketing: Some(marketing),
        };
        let msg = InstantiateMsg {
            max_supply,
            max_minted,
            gov: Some(gov.clone()),
            cw20_init_msg: cw20_instantiate_msg,
        };
        // test positive case
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res, Response::default());

        // test negative case: cw20_instantiate error
        let marketing = InstantiateMarketingInfo {
            project: Option::from("Test Project".to_string()),
            description: Option::from("Test Description".to_string()),
            logo: None,
            marketing: Some(gov.clone().to_string()),
        };
        let invalid_cw20_instantiate_msg = Cw20InitMsg {
            name: String::from("name"),
            symbol: String::from("symbol"),
            decimals: 6u8,
            initial_balances: vec![],
            mint: None,
            marketing: Some(marketing),

        };
        let invalid_msg = InstantiateMsg {
            max_supply,
            max_minted,
            gov: Some(gov.clone()),
            cw20_init_msg: invalid_cw20_instantiate_msg,
        };
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), invalid_msg);
        println!("res:{:?}", res);
        // test stored vote config
        let vote_config = read_vote_config(deps.as_ref().storage).unwrap();
        println!("vote_config:{:?}", vote_config);
        assert_eq!(vote_config.max_supply, max_supply);
        assert_eq!(vote_config.gov, Addr::unchecked(gov.clone()));
        assert_eq!(vote_config.max_minted, Uint128::from(max_minted));
        assert_eq!(vote_config.total_minted, Uint128::from(0u128));
        // test stored minter

        // test stored contract version
        let contract_version = get_contract_version(deps.as_ref().storage).unwrap();
        println!("version:{}", contract_version.version);
        println!("name:{}", contract_version.contract);
        assert_eq!(contract_version.contract, CONTRACT_NAME);
        assert_eq!(contract_version.version, CONTRACT_VERSION);
    }
}