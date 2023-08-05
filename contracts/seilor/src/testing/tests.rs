#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate};
    use crate::msg::{ExecuteMsg, InstantiateMsg, SeilorConfigResponse};
    use crate::querier::query_seilor_config;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
    };
    use cosmwasm_std::StdError::GenericErr;
    use cosmwasm_std::{coins, Addr, Deps, Response, Uint128};
    use cw20_base::contract::query_balance;
    use cw20_base::msg::InstantiateMarketingInfo;
    use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
    use cw20_base::ContractError;

    fn get_balance<T: Into<String>>(deps: Deps, address: T) -> Uint128 {
        query_balance(deps, address.into()).unwrap().balance
    }

    fn mock_cw20_init_msg() -> Cw20InstantiateMsg {
        Cw20InstantiateMsg {
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
        }
    }

    fn default_instantiate(max_supply: u128) -> InstantiateMsg {
        let cw20_init_msg = mock_cw20_init_msg();
        return InstantiateMsg {
            cw20_init_msg,
            max_supply,
            gov: None,
        };
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let max_supply = 1000000u128;

        let _msg = InstantiateMsg {
            cw20_init_msg: mock_cw20_init_msg(),
            gov: Some(Addr::unchecked("gov")),
            max_supply,
        };
        let _info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(_res, Response::default());

        // Verify the KptConfig is stored correctly
        assert_eq!(
            query_seilor_config(deps.as_ref()).unwrap(),
            SeilorConfigResponse {
                max_supply,
                gov: Addr::unchecked("gov"),
                fund: Addr::unchecked(""),
                distribute: Addr::unchecked(""),
            }
        );
    }

    #[test]
    fn test_update_config() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let max_supply = 1000000u128;

        // make sure we can instantiate with this
        let instantiate_msg = default_instantiate(max_supply);
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, instantiate_msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Negative test case with insufficient permissions
        let _msg = ExecuteMsg::UpdateConfig {
            fund: Some(Addr::unchecked("new_fund")),
            distribute: Some(Addr::unchecked("new_distribute")),
            gov: Some(Addr::unchecked("new_gov")),
        };
        let _info = mock_info("random_user", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // Verify that the config values remain unchanged
        assert_eq!(
            query_seilor_config(deps.as_ref()).unwrap(),
            SeilorConfigResponse {
                max_supply,
                gov: Addr::unchecked("creator"),
                fund: Addr::unchecked(""),
                distribute: Addr::unchecked(""),
            }
        );

        // Positive test case
        let _msg = ExecuteMsg::UpdateConfig {
            fund: Some(Addr::unchecked("new_fund")),
            distribute: Some(Addr::unchecked("new_distribute")),
            gov: Some(Addr::unchecked("new_gov")),
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the updated values in the storage
        assert_eq!(
            query_seilor_config(deps.as_ref()).unwrap(),
            SeilorConfigResponse {
                max_supply,
                gov: Addr::unchecked("new_gov"),
                fund: Addr::unchecked("new_fund"),
                distribute: Addr::unchecked("new_distribute"),
            }
        );

        // Verify old gov with insufficient permissions
        let _msg = ExecuteMsg::UpdateConfig {
            fund: Some(Addr::unchecked("new_fund")),
            distribute: Some(Addr::unchecked("new_distribute")),
            gov: Some(Addr::unchecked("new_gov")),
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }
    }

    #[test]
    fn test_mint() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let max_supply = 1000000u128;
        let amount = Uint128::from(112233u128);

        // make sure we can instantiate with this
        let instantiate_msg = default_instantiate(max_supply);
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, instantiate_msg).unwrap();
        assert_eq!(0, _res.messages.len());

        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
            contract: None,
            msg: None,
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap_err();
        match _res {
            ContractError::Std(GenericErr { msg, .. }) => {
                assert_eq!(
                    msg,
                    "Fund or distribute contract must to be configured".to_string()
                )
            }
            _ => panic!("Do not enter in"),
        }

        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
            contract: None,
            msg: None,
        };
        let _info = mock_info("random_user", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap_err();
        match _res {
            ContractError::Std(GenericErr { msg, .. }) => {
                assert_eq!(
                    msg,
                    "Fund or distribute contract must to be configured".to_string()
                )
            }
            _ => panic!("Do not enter in"),
        }

        // proper update config
        let _msg = ExecuteMsg::UpdateConfig {
            fund: Some(Addr::unchecked("new_fund".to_string())),
            distribute: Some(Addr::unchecked("new_distribute".to_string())),
            gov: Some(Addr::unchecked("new_gov".to_string())),
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Negative test case with insufficient permissions, only seilor_fund && seilor_distribute
        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
            contract: None,
            msg: None,
        };
        let _info = mock_info("random_user", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
            contract: None,
            msg: None,
        };
        let _info = mock_info("new_gov", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // Positive test case, only seilor_fund && seilor_distribute
        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
            contract: None,
            msg: None,
        };
        let _info = mock_info("new_fund", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        assert_eq!(get_balance(deps.as_ref(), "lucky"), Uint128::new(112233));

        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
            contract: None,
            msg: None,
        };
        let _info = mock_info("new_distribute", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        assert_eq!(get_balance(deps.as_ref(), "lucky"), Uint128::new(224466));
    }

    #[test]
    fn test_burn() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let max_supply = 1000000u128;
        let amount = Uint128::from(112233u128);

        // make sure we can instantiate with this
        let instantiate_msg = default_instantiate(max_supply);
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, instantiate_msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // proper update config
        let _msg = ExecuteMsg::UpdateConfig {
            fund: Some(Addr::unchecked("new_fund".to_string())),
            distribute: Some(Addr::unchecked("new_distribute".to_string())),
            gov: Some(Addr::unchecked("creator".to_string())),
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // proper mint
        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
            contract: None,
            msg: None,
        };
        let _info = mock_info("new_fund", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        assert_eq!(get_balance(deps.as_ref(), "lucky"), Uint128::new(112233));

        // Negative test case with insufficient permissions, only seilor_fund
        let _msg = ExecuteMsg::Burn {
            user: "lucky".to_string(),
            amount,
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        let _msg = ExecuteMsg::Burn {
            user: "lucky".to_string(),
            amount,
        };
        let _info = mock_info("new_distribute", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // Positive test case, only seilor_fund
        let _msg = ExecuteMsg::Burn {
            user: "lucky".to_string(),
            amount,
        };
        let _info = mock_info("new_fund", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        assert_eq!(get_balance(deps.as_ref(), "lucky"), Uint128::zero());
    }
}
