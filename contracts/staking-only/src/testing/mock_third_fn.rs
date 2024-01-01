use crate::testing::mock_fn::CREATOR;
use cosmwasm_std::Uint128;
use cw20_base::msg::InstantiateMarketingInfo;

pub fn mock_staking_token_instance_msg() -> cw20_base::msg::InstantiateMsg {
    let cw20_init_msg = cw20_base::msg::InstantiateMsg {
        name: "Staking Token".to_string(),
        symbol: "STK".to_string(),
        decimals: 6,
        initial_balances: vec![cw20::Cw20Coin {
            address: CREATOR.to_string(),
            amount: Uint128::from(100000000000u128),
        }],
        mint: None,
        marketing: Some(InstantiateMarketingInfo {
            project: None,
            description: None,
            marketing: Some("aass".to_string()),
            logo: None,
        }),
    };
    cw20_init_msg
}

pub fn mock_reward_token_instance_msg() -> cw20_base::msg::InstantiateMsg {
    let msg = cw20_base::msg::InstantiateMsg {
        name: "Reward Token".to_string(),
        symbol: "RToken".to_string(),
        decimals: 6,
        initial_balances: vec![cw20::Cw20Coin {
            address: CREATOR.to_string(),
            amount: Uint128::from(20000000000000u128),
        }],
        mint: None,
        marketing: Some(InstantiateMarketingInfo {
            project: None,
            description: None,
            marketing: Some("aass".to_string()),
            logo: None,
        }),
    };
    msg
}
