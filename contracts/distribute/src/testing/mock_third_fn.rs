use cw20_base::msg::InstantiateMarketingInfo;

pub fn mock_seilor_instantiate_msg() -> seilor::msg::InstantiateMsg {
    let max_supply = 1000000000000000u128;
    let cw20_init_msg = cw20_base::msg::InstantiateMsg {
        name: "seilor dev".to_string(),
        symbol: "seilor".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: None,
        marketing: Some(InstantiateMarketingInfo {
            project: None,
            description: None,
            marketing: Some("aass".to_string()),
            logo: None,
        }),
    };
    let msg = seilor::msg::InstantiateMsg {
        cw20_init_msg,
        max_supply,
        gov: None,
    };
    msg
}
