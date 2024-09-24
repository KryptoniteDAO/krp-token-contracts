use std::ops::Mul;
use crate::testing::mock_fn::{CREATOR, KUSD_DENOM, KUSD_REWARD_ADDR};
use boost::state::VeSeilorLockSetting;
use cosmwasm_std::{Addr, Uint128, Uint64};
use cw20_base::msg::InstantiateMarketingInfo;

pub fn mock_staking_token_instance_msg() -> cw20_base::msg::InstantiateMsg {
    let cw20_init_msg = cw20_base::msg::InstantiateMsg {
        name: "Staking Token".to_string(),
        symbol: "STK".to_string(),
        decimals: 18,
        initial_balances: vec![cw20::Cw20Coin {
            address: CREATOR.to_string(),
            amount: Uint128::from(10u128).pow(18).mul(Uint128::from(100000000000u128)),
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

pub fn mock_ve_seilor_instance_msg() -> ve_seilor::msg::InstantiateMsg {
    let msg = ve_seilor::msg::InstantiateMsg {
        cw20_init_msg: cw20_base::msg::InstantiateMsg {
            name: "Ve seilor token".to_string(),
            symbol: "VESEILOR".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: None,
            marketing: Some(InstantiateMarketingInfo {
                project: None,
                description: None,
                marketing: Some("aass".to_string()),
                logo: None,
            }),
        },
        max_supply: 990000000000000u128,
        gov: None,
        max_minted: 60500000000000u128,
    };
    msg
}

pub fn mock_boost_instance_msg() -> boost::msg::InstantiateMsg {
    let msg = boost::msg::InstantiateMsg {
        gov: None,
        ve_seilor_lock_settings: vec![
            VeSeilorLockSetting {
                duration: Uint128::from(2592000u128),
                mining_boost: Uint128::from(20000000u128),
            },
            VeSeilorLockSetting {
                duration: Uint128::from(7776000u128),
                mining_boost: Uint128::from(30000000u128),
            },
            VeSeilorLockSetting {
                duration: Uint128::from(15552000u128),
                mining_boost: Uint128::from(50000000u128),
            },
            VeSeilorLockSetting {
                duration: Uint128::from(31536000u128),
                mining_boost: Uint128::from(100000000u128),
            },
        ],
    };
    msg
}

pub fn mock_seilor_instance_msg() -> seilor::msg::InstantiateMsg {
    let msg = seilor::msg::InstantiateMsg {
        cw20_init_msg: cw20_base::msg::InstantiateMsg {
            name: "SEILOR".to_string(),
            symbol: "SEILOR".to_string(),
            decimals: 6,
            initial_balances: vec![cw20::Cw20Coin {
                address: CREATOR.to_string(),
                amount: Uint128::from(10000000000000u128),
            }],
            mint: None,
            marketing: Some(InstantiateMarketingInfo {
                project: None,
                description: None,
                marketing: Some("aass".to_string()),
                logo: None,
            }),
        },
        max_supply: 1000000000000000u128,
        gov: None,
    };
    msg
}

pub fn mock_fund_instance_msg(ve_seilor: &Addr, seilor: &Addr) -> fund::msg::InstantiateMsg {
    let msg = fund::msg::InstantiateMsg {
        gov: None,
        ve_seilor_addr: ve_seilor.clone(),
        seilor_addr: seilor.clone(),
        kusd_denom: KUSD_DENOM.clone().to_string(),
        kusd_reward_addr: Addr::unchecked(KUSD_REWARD_ADDR.clone().to_string()),
        exit_cycle: Uint64::from(2592000u64),
        claim_able_time: Uint64::from(1687190400u64),
    };
    msg
}
