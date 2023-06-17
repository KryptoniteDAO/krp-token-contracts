use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, Addr, StdResult, StdError, Deps, to_binary, Binary};
use cw20::{MinterResponse};
use cw20_base::allowances::{execute_decrease_allowance, execute_increase_allowance, execute_send_from, execute_transfer_from, query_allowance};
use cw20_base::contract::{execute_send, execute_transfer, execute_update_marketing, execute_update_minter, execute_upload_logo, query_balance, query_download_logo, query_marketing_info, query_minter, query_token_info};
use cw2::set_contract_version;
use cw_utils::nonpayable;
use crate::error::ContractError;
use crate::handler::{burn, mint, update_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::query_kpt_config;
use crate::state::{KptConfig, store_kpt_config};
use cw20_base::msg::{InstantiateMsg as Cw20InstantiateMsg, InstantiateMarketingInfo};
use cw20_base::contract::instantiate as cw20_instantiate;
use cw20_base::enumerable::{query_all_accounts, query_owner_allowances, query_spender_allowances};

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:cw20-kpt";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let r = nonpayable(&info.clone());
    if r.is_err() {
        return Err(ContractError::Std(StdError::generic_err(r.err().unwrap().to_string())));
    }


    let mut cw20_instantiate_msg: Cw20InstantiateMsg = msg.cw20_init_msg;

    let gov = if let Some(gov_addr) = msg.gov {
        gov_addr
    } else {
        info.clone().sender
    };

    cw20_instantiate_msg.mint = Some(MinterResponse {
        minter: env.contract.address.to_string(),
        cap: Some(msg.max_supply.into()),
    });

    let marketing = cw20_instantiate_msg.marketing.unwrap();
    cw20_instantiate_msg.marketing = Some(InstantiateMarketingInfo {
        project: marketing.project,
        description: marketing.description,
        logo: marketing.logo,
        marketing: Option::from(gov.clone().to_string()),
    });


    let ins_res = cw20_instantiate(deps.branch(), env, info, cw20_instantiate_msg);
    if ins_res.is_err() {
        return Err(ContractError::Std(StdError::generic_err(ins_res.err().unwrap().to_string())));
    }

    let kpt_config = KptConfig {
        max_supply: msg.max_supply,
        kpt_fund: Addr::unchecked(""),
        gov,
    };

    store_kpt_config(deps.storage, &kpt_config)?;

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
        ExecuteMsg::UpdateConfig { max_supply, kpt_fund, gov } => {
            update_config(deps, info, max_supply, kpt_fund, gov)
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
        // these all come from cw20-base to implement the cw20 standard
        ExecuteMsg::Transfer { recipient, amount } => {
            let cw20_res = execute_transfer(deps, env, info, recipient, amount);
            if cw20_res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(cw20_res.err().unwrap().to_string())));
            }
            Ok(Response::default().add_attributes(cw20_res.unwrap().attributes))
        }
        ExecuteMsg::Send {
            contract,
            amount,
            send_msg,
        } => {
            let cw20_res = execute_send(deps, env, info, contract, amount, send_msg);
            if cw20_res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(cw20_res.err().unwrap().to_string())));
            }
            Ok(Response::default().add_attributes(cw20_res.unwrap().attributes))
        }
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => {
            let cw20_res = execute_increase_allowance(deps, env, info, spender, amount, expires);
            if cw20_res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(cw20_res.err().unwrap().to_string())));
            }
            Ok(Response::default().add_attributes(cw20_res.unwrap().attributes))
        }
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => {
            let cw20_res = execute_decrease_allowance(deps, env, info, spender, amount, expires);
            if cw20_res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(cw20_res.err().unwrap().to_string())));
            }
            Ok(Response::default().add_attributes(cw20_res.unwrap().attributes))
        }
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => {
            let cw20_res = execute_transfer_from(deps, env, info, owner, recipient, amount);
            if cw20_res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(cw20_res.err().unwrap().to_string())));
            }
            Ok(Response::default().add_attributes(cw20_res.unwrap().attributes))
        }
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            send_msg,
        } => {
            let cw20_res = execute_send_from(deps, env, info, owner, contract, amount, send_msg);
            if cw20_res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(cw20_res.err().unwrap().to_string())));
            }
            Ok(Response::default().add_attributes(cw20_res.unwrap().attributes))
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
        },
        ExecuteMsg::UploadLogo(logo) => {
            let res = execute_upload_logo(deps, env, info, logo);
            if res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(res.err().unwrap().to_string())));
            }
            Ok(res.unwrap())
        },
        ExecuteMsg::UpdateMinter { new_minter } => {
            let res = execute_update_minter(deps, env, info, new_minter);
            if res.is_err() {
                return Err(ContractError::Std(StdError::generic_err(res.err().unwrap().to_string())));
            }
            Ok(res.unwrap())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // custom queries
        QueryMsg::KptConfig {} => to_binary(&query_kpt_config(deps)?),

        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        QueryMsg::AllAllowances {
            owner,
            start_after,
            limit,
        } => to_binary(&query_owner_allowances(deps, owner, start_after, limit)?),
        QueryMsg::AllSpenderAllowances {
            spender,
            start_after,
            limit,
        } => to_binary(&query_spender_allowances(
            deps,
            spender,
            start_after,
            limit,
        )?),
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
    use cosmwasm_std::{Addr};
    use cw20_base::msg::{InstantiateMsg as Cw20InitMsg};
    use crate::state::read_kpt_config;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let max_supply = 1000000u128;
        let cw20_init_msg = Cw20InitMsg {
            name: "Test Token".to_string(),
            symbol: "TEST".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: None,
            marketing: Some(InstantiateMarketingInfo {
                project: Option::from("Test Project".to_string()),
                description: Option::from("Test Description".to_string()),
                logo: None,
                marketing: None,
            }),
        };
        let msg = InstantiateMsg {
            cw20_init_msg,
            max_supply,
            gov: None
        };
        // Positive test case
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res, Response::default());
        // Check kpt config
        let kpt_config = read_kpt_config(deps.as_ref().storage).unwrap();
        println!("{:?}", kpt_config);
        assert_eq!(kpt_config.max_supply, max_supply);
        assert_eq!(kpt_config.kpt_fund, Addr::unchecked(""));
        assert_eq!(kpt_config.gov, info.sender);

    }
}