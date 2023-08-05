#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate};
    use crate::error::ContractError;
    use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, StateResponse};
    use crate::querier::{query_config, query_state};
    use cosmwasm_std::testing::{
        mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{coins, Uint128};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let threshold = 1000000u128;

        let _msg = InstantiateMsg {
            owner: "creator".to_string(),
            threshold: Uint128::from(threshold),
            rewards_contract: "kpt_fund".to_string(),
            rewards_denom: "kUSD".to_string(),
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the config is stored correctly
        assert_eq!(
            query_config(deps.as_ref()).unwrap(),
            ConfigResponse {
                owner: "creator".to_string(),
                threshold: Uint128::from(threshold),
                rewards_contract: "kpt_fund".to_string(),
                rewards_denom: "kUSD".to_string(),
            }
        );
    }

    #[test]
    fn test_update_config() {
        let mut deps = mock_dependencies();
        let threshold = 1000000u128;
        let threshold2 = 2000000u128;

        let _msg = InstantiateMsg {
            owner: "creator".to_string(),
            threshold: Uint128::from(threshold),
            rewards_contract: "kpt_fund".to_string(),
            rewards_denom: "kUSD".to_string(),
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Negative test case with insufficient permissions
        let _msg = ExecuteMsg::UpdateConfig {
            owner: Some("new_creator".to_string()),
            threshold: Some(Uint128::from(threshold2)),
            rewards_contract: Some("new_fund".to_string()),
            rewards_denom: Some("new_kUSD".to_string()),
        };
        let _info = mock_info("random_user", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg.clone());
        match _res {
            Err(ContractError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // Positive test case
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg.clone()).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the updated values in the storage
        assert_eq!(
            query_config(deps.as_ref()).unwrap(),
            ConfigResponse {
                owner: "new_creator".to_string(),
                threshold: Uint128::from(threshold2),
                rewards_contract: "new_fund".to_string(),
                rewards_denom: "new_kUSD".to_string(),
            }
        );

        // Verify old gov with insufficient permissions
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg);
        match _res {
            Err(ContractError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }
    }

    #[test]
    fn test_distribute() {
        let mut deps = mock_dependencies_with_balance(&coins(1000000u128, String::from("kUSD")));
        let threshold = 1000000u128;

        let _msg = InstantiateMsg {
            owner: "creator".to_string(),
            threshold: Uint128::from(threshold),
            rewards_contract: "kpt_fund".to_string(),
            rewards_denom: "kUSD".to_string(),
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        let _msg = ExecuteMsg::Distribute {};
        let _info = mock_info(MOCK_CONTRACT_ADDR, &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(1, _res.messages.len());

        assert_eq!(
            query_state(deps.as_ref()).unwrap(),
            StateResponse {
                update_time: Uint128::from(1571797419u128),
                distributed_amount: Uint128::from(threshold),
                distributed_total: Uint128::from(threshold),
            }
        );

        // assert_eq!(
        //     query_balance(
        //         deps.as_ref(),
        //         Addr::unchecked(MOCK_CONTRACT_ADDR),
        //         "kUSD".to_string()
        //     )
        //     .unwrap(),
        //     Uint128::zero()
        // );
    }
}
