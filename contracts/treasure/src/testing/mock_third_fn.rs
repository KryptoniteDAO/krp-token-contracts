use crate::testing::mock_fn::CREATOR;
use cosmwasm_std::Uint128;
use cw20::Cw20Coin;
use cw20_base::msg::InstantiateMarketingInfo;

pub fn mock_cw20_instantiate_msg() -> cw20_base::msg::InstantiateMsg {
    cw20_base::msg::InstantiateMsg {
        name: "Test cw20".to_string(),
        symbol: "test".to_string(),
        decimals: 6,
        initial_balances: vec![Cw20Coin {
            address: CREATOR.to_string(),
            amount: Uint128::from(10000000000000000000u128),
        }],
        mint: None,
        marketing: Some(InstantiateMarketingInfo {
            project: None,
            description: None,
            marketing: Some("aass".to_string()),
            logo: None,
        }),
    }
}
