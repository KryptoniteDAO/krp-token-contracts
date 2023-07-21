use cosmwasm_std::Uint128;
use cw20::Cw20Coin;
use crate::testing::mock_fn::CREATOR;

pub fn mock_kpt_instantiate_msg() -> kpt::msg::InstantiateMsg {
    let max_supply = 1000000000000000u128;
    let cw20_init_msg = cw20_base::msg::InstantiateMsg {
        name: "kpt dev".to_string(),
        symbol: "kpt".to_string(),
        decimals: 6,
        initial_balances: vec![
            Cw20Coin {
                address: CREATOR.to_string(),
                amount: Uint128::from(200000000000000u128),
            },
        ],
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

pub fn mock_ve_kpt_instantiate_msg() -> ve_kpt::msg::InstantiateMsg {
    let cw20_instantiate_msg = cw20_base::msg::InstantiateMsg {
        name: String::from("ve kpt dev"),
        symbol: String::from("vekpt"),
        decimals: 6u8,
        initial_balances: vec![],
        mint: None,
        marketing: None,
    };
    let msg = ve_kpt::msg::InstantiateMsg {
        max_supply: 60500000000000u128,
        max_minted: 60500000000000u128,
        gov: None,
        cw20_init_msg: cw20_instantiate_msg,
    };
    msg
}