pub fn mock_kpt_instantiate_msg() -> kpt::msg::InstantiateMsg {
    let max_supply = 1000000000000000u128;
    let cw20_init_msg = cw20_base::msg::InstantiateMsg {
        name: "kpt dev".to_string(),
        symbol: "kpt".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: None,
        marketing: None,
    };
    let msg = kpt::msg::InstantiateMsg {
        cw20_init_msg,
        max_supply,
        gov: None,
    };
    msg
}
