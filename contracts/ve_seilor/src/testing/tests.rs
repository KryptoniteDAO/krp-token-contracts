#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, IsMinterResponse, VoteConfigResponse};
    use crate::querier::{query_is_minter, query_vote_config};
    use cosmwasm_std::testing::{
        mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
    };
    use cosmwasm_std::StdError::GenericErr;
    use cosmwasm_std::{Addr, Deps, Response, Uint128};
    use cw20::Cw20Coin;
    use cw20_base::contract::query_balance;
    use cw20_base::msg::InstantiateMarketingInfo;
    use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;

    fn get_balance<T: Into<String>>(deps: Deps, address: T) -> Uint128 {
        query_balance(deps, address.into()).unwrap().balance
    }

    fn mock_cw20_marketing_init_msg() -> InstantiateMarketingInfo {
        InstantiateMarketingInfo {
            project: Option::from("Test Project".to_string()),
            description: Option::from("Test Description".to_string()),
            logo: None,
            marketing: None,
        }
    }

    fn mock_cw20_init_msg() -> Cw20InstantiateMsg {
        Cw20InstantiateMsg {
            name: "Test Token".to_string(),
            symbol: "TEST".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: None,
            marketing: Some(mock_cw20_marketing_init_msg()),
        }
    }

    fn default_instantiate(max_supply: u128, max_minted: u128) -> InstantiateMsg {
        let cw20_init_msg = mock_cw20_init_msg();
        return InstantiateMsg {
            cw20_init_msg,
            max_supply,
            gov: None,
            max_minted,
        };
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let max_supply = 1000000u128;
        let max_minted = 500000u128;

        let _msg = InstantiateMsg {
            cw20_init_msg: mock_cw20_init_msg(),
            gov: Some(Addr::unchecked("gov")),
            max_supply,
            max_minted,
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(_res, Response::default());

        // Verify the Config is stored correctly
        assert_eq!(
            query_vote_config(deps.as_ref()).unwrap(),
            VoteConfigResponse {
                max_supply,
                gov: Addr::unchecked("gov"),
                fund: Addr::unchecked(""),
                max_minted: Uint128::from(max_minted),
                total_minted: Uint128::zero(),
            }
        );
    }

    #[test]
    fn test_instantiate_negative() {
        let mut deps = mock_dependencies();
        let max_supply = 1000000u128;
        let max_minted = 500000u128;

        let _msg = InstantiateMsg {
            cw20_init_msg: Cw20InstantiateMsg {
                name: "Test Token".to_string(),
                symbol: "TEST".to_string(),
                decimals: 6,
                initial_balances: vec![Cw20Coin {
                    address: "lucky".to_string(),
                    amount: Uint128::from(1000u128),
                }],
                mint: None,
                marketing: Some(mock_cw20_marketing_init_msg()),
            },
            gov: Some(Addr::unchecked("gov")),
            max_supply,
            max_minted,
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::UnableInitialBalances {}) => {}
            _ => panic!("Must return unable initial balances error"),
        }
    }

    #[test]
    fn test_update_config() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let max_supply = 1000000u128;
        let max_minted = 500000u128;

        // make sure we can instantiate with this
        let _msg = default_instantiate(max_supply, max_minted);
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Negative test case with insufficient permissions
        let _msg = ExecuteMsg::UpdateConfig {
            max_minted: Some(Uint128::from(max_minted)),
            fund: Some(Addr::unchecked("new_fund")),
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
            query_vote_config(deps.as_ref()).unwrap(),
            VoteConfigResponse {
                max_supply,
                gov: Addr::unchecked("creator"),
                fund: Addr::unchecked(""),
                max_minted: Uint128::from(max_minted),
                total_minted: Uint128::zero(),
            }
        );

        // Positive test case
        let _msg = ExecuteMsg::UpdateConfig {
            max_minted: Some(Uint128::from(max_minted)),
            fund: Some(Addr::unchecked("new_fund")),
            gov: Some(Addr::unchecked("new_gov")),
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the updated values in the storage
        assert_eq!(
            query_vote_config(deps.as_ref()).unwrap(),
            VoteConfigResponse {
                max_supply,
                gov: Addr::unchecked("new_gov"),
                fund: Addr::unchecked("new_fund"),
                max_minted: Uint128::from(max_minted),
                total_minted: Uint128::zero(),
            }
        );

        // Verify old gov with insufficient permissions
        let _msg = ExecuteMsg::UpdateConfig {
            max_minted: Some(Uint128::from(max_minted)),
            fund: Some(Addr::unchecked("new_fund")),
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
    fn test_set_minters() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let max_supply = 1000000u128;
        let max_minted = 500000u128;

        let _msg = default_instantiate(max_supply, max_minted);
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        let _msg = ExecuteMsg::SetMinters {
            contracts: vec![
                Addr::unchecked("address000"),
                Addr::unchecked("address111"),
                Addr::unchecked("address222"),
            ],
            is_minter: vec![true, false, true],
        };
        let _info = mock_info("random_user", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg.clone());
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the result
        assert_eq!(
            query_is_minter(deps.as_ref(), Addr::unchecked("address000")).unwrap(),
            IsMinterResponse { is_minter: true }
        );
        assert_eq!(
            query_is_minter(deps.as_ref(), Addr::unchecked("address111")).unwrap(),
            IsMinterResponse { is_minter: false }
        );
        assert_eq!(
            query_is_minter(deps.as_ref(), Addr::unchecked("address222")).unwrap(),
            IsMinterResponse { is_minter: true }
        );
    }

    #[test]
    fn test_mint() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let max_supply = 1000000u128;
        let max_minted = 500000u128;
        let amount = Uint128::from(112233u128);

        // make sure we can instantiate with this
        let _msg = default_instantiate(max_supply, max_minted);
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Mint Contract Not Config"),
        }

        // proper update config
        let _msg = ExecuteMsg::UpdateConfig {
            max_minted: Some(Uint128::from(max_minted)),
            fund: Some(Addr::unchecked("new_fund".to_string())),
            gov: None,
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Negative test case with insufficient permissions
        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
        };
        let _info = mock_info("random_user", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // Positive test case, with fund
        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
        };
        let _info = mock_info("new_fund", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        assert_eq!(get_balance(deps.as_ref(), "lucky"), Uint128::new(112233));

        assert_eq!(
            query_vote_config(deps.as_ref()).unwrap(),
            VoteConfigResponse {
                max_supply,
                gov: Addr::unchecked("creator"),
                fund: Addr::unchecked("new_fund"),
                max_minted: Uint128::from(max_minted),
                total_minted: Uint128::zero(),
            }
        );
    }

    #[test]
    fn test_mint_with_minter() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let max_supply = 1000000u128;
        let max_minted = 500000u128;
        let amount = Uint128::from(500000u128);
        let amount2 = Uint128::from(0u128);

        // make sure we can instantiate with this
        let _msg = default_instantiate(max_supply, max_minted);
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // proper update config
        let _msg = ExecuteMsg::UpdateConfig {
            max_minted: Some(Uint128::from(max_minted)),
            fund: Some(Addr::unchecked("new_fund".to_string())),
            gov: None,
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // proper set minters
        let _msg = ExecuteMsg::SetMinters {
            contracts: vec![Addr::unchecked("address000")],
            is_minter: vec![true],
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Positive test case, with fund
        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
        };
        let _info = mock_info("address000", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(1, _res.messages.len());

        assert_eq!(get_balance(deps.as_ref(), "lucky"), Uint128::from(amount));

        assert_eq!(
            query_vote_config(deps.as_ref()).unwrap(),
            VoteConfigResponse {
                max_supply,
                gov: Addr::unchecked("creator"),
                fund: Addr::unchecked("new_fund"),
                max_minted: Uint128::from(max_minted),
                total_minted: Uint128::from(amount),
            }
        );

        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount: amount2,
        };
        let _info = mock_info("address000", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap_err();
        match _res {
            ContractError::Std(GenericErr { msg, .. }) => {
                assert_eq!(msg, "Invalid zero amount".to_string())
            }
            _ => panic!("Mint return over error"),
        }

        assert_eq!(get_balance(deps.as_ref(), "lucky"), Uint128::from(amount));
        assert_eq!(
            query_vote_config(deps.as_ref()).unwrap(),
            VoteConfigResponse {
                max_supply,
                gov: Addr::unchecked("creator"),
                fund: Addr::unchecked("new_fund"),
                max_minted: Uint128::from(max_minted),
                total_minted: Uint128::from(max_minted),
            }
        );
    }

    #[test]
    fn test_burn() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let max_supply = 1000000u128;
        let max_minted = 500000u128;
        let amount = Uint128::from(112233u128);

        // make sure we can instantiate with this
        let instantiate_msg = default_instantiate(max_supply, max_minted);
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, instantiate_msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // proper update config
        let _msg = ExecuteMsg::UpdateConfig {
            max_minted: Some(Uint128::from(max_minted)),
            fund: Some(Addr::unchecked("new_fund".to_string())),
            gov: Some(Addr::unchecked("creator".to_string())),
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // proper set minters
        let _msg = ExecuteMsg::SetMinters {
            contracts: vec![Addr::unchecked("address000")],
            is_minter: vec![true],
        };
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // proper mint
        let _msg = ExecuteMsg::Mint {
            recipient: "lucky".to_string(),
            amount,
        };
        let _info = mock_info("new_fund", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        assert_eq!(get_balance(deps.as_ref(), "lucky"), Uint128::new(112233));

        // Negative test case with insufficient permissions
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

        // Positive test case by fund
        let _msg = ExecuteMsg::Burn {
            user: "lucky".to_string(),
            amount: Uint128::from(1u128),
        };
        let _info = mock_info("new_fund", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        assert_eq!(
            get_balance(deps.as_ref(), "lucky"),
            Uint128::from(112232u128)
        );

        // Positive test case by minter
        let _msg = ExecuteMsg::Burn {
            user: "lucky".to_string(),
            amount: Uint128::from(1u128),
        };
        let _info = mock_info("address000", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(1, _res.messages.len());

        assert_eq!(
            get_balance(deps.as_ref(), "lucky"),
            Uint128::from(112231u128)
        );
    }
}
