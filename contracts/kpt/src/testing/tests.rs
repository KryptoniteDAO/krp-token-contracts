#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr, Response};
    use cw20_base::msg::InstantiateMarketingInfo;
    use cw20_base::msg::{InstantiateMsg as Cw20InstantiateMsg};
    use crate::contract::instantiate;
    use crate::msg::InstantiateMsg;
    use crate::state::read_kpt_config;

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

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &coins(1000, "token"));

        let msg = InstantiateMsg {
            cw20_init_msg: mock_cw20_init_msg(),
            gov: Some(Addr::unchecked("gov")),
            max_supply: 1000000,
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res, Response::default());
        // Verify the KptConfig is stored correctly
        let kpt_config = read_kpt_config(deps.as_ref().storage).unwrap();
        assert_eq!(kpt_config.max_supply, 1000000);
        assert_eq!(kpt_config.gov, Addr::unchecked("gov"));
        assert_eq!(kpt_config.kpt_distribute, Addr::unchecked(""));
    }

}