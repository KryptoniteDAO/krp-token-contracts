use crate::testing::mock_fn::{CREATOR, KUSD_DENOM, KUSD_REWARD_ADDR};
use cosmwasm_std::{Addr, Uint128, Uint64};
use ve_kpt_boost::state::VeKptLockSetting;

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
        marketing: None,
    };
    cw20_init_msg
}

pub fn mock_ve_kpt_instance_msg() -> ve_kpt::msg::InstantiateMsg {
    let msg = ve_kpt::msg::InstantiateMsg {
        cw20_init_msg: cw20_base::msg::InstantiateMsg {
            name: "Ve kpt token".to_string(),
            symbol: "VEKPT".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: None,
            marketing: None,
        },
        max_supply: 990000000000000u128,
        gov: None,
        max_minted: 60500000000000u128,
    };
    msg
}

pub fn mock_ve_kpt_boost_instance_msg() -> ve_kpt_boost::msg::InstantiateMsg {
    let msg = ve_kpt_boost::msg::InstantiateMsg {
        gov: None,
        ve_kpt_lock_settings: vec![
            VeKptLockSetting {
                duration: Uint128::from(2592000u128),
                mining_boost: Uint128::from(20000000u128),
            },
            VeKptLockSetting {
                duration: Uint128::from(7776000u128),
                mining_boost: Uint128::from(30000000u128),
            },
            VeKptLockSetting {
                duration: Uint128::from(15552000u128),
                mining_boost: Uint128::from(50000000u128),
            },
            VeKptLockSetting {
                duration: Uint128::from(31536000u128),
                mining_boost: Uint128::from(100000000u128),
            },
        ],
    };
    msg
}

pub fn mock_kpt_instance_msg() -> kpt::msg::InstantiateMsg {
    let msg = kpt::msg::InstantiateMsg {
        cw20_init_msg: cw20_base::msg::InstantiateMsg {
            name: "KPT".to_string(),
            symbol: "KPT".to_string(),
            decimals: 6,
            initial_balances: vec![cw20::Cw20Coin {
                address: CREATOR.to_string(),
                amount: Uint128::from(10000000000000u128),
            }],
            mint: None,
            marketing: None,
        },
        max_supply: 1000000000000000u128,
        gov: None,
    };
    msg
}

pub fn mock_kpt_fund_instance_msg(ve_kpt: &Addr, kpt: &Addr) -> kpt_fund::msg::InstantiateMsg {
    let msg = kpt_fund::msg::InstantiateMsg {
        gov: None,
        ve_kpt_addr: ve_kpt.clone(),
        kpt_addr: kpt.clone(),
        kusd_denom: KUSD_DENOM.clone().to_string(),
        kusd_reward_addr: Addr::unchecked(KUSD_REWARD_ADDR.clone().to_string()),
        exit_cycle: Uint64::from(2592000u64),
        claim_able_time: Uint64::from(1687190400u64),
    };
    msg
}
