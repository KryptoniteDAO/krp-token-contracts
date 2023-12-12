#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate};
    use crate::msg::{ExecuteMsg, GetBoostConfigResponse, InstantiateMsg, LockStatusResponse};
    use crate::querier::{get_boost_config, get_user_lock_status};
    use crate::state::VeSeilorLockSetting;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::StdError::GenericErr;
    use cosmwasm_std::{Addr, Uint128};

    fn ve_seilor_lock_settings() -> Vec<VeSeilorLockSetting> {
        vec![
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
        ]
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let _msg = InstantiateMsg {
            gov: Some(Addr::unchecked("gov")),
            ve_seilor_lock_settings: ve_seilor_lock_settings(),
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the config is stored correctly
        assert_eq!(
            get_boost_config(deps.as_ref()).unwrap(),
            GetBoostConfigResponse {
                gov: Addr::unchecked("gov"),
                ve_seilor_lock_settings: ve_seilor_lock_settings(),
                new_gov: None,
            }
        );
    }

    #[test]
    fn test_add_lock_setting() {
        let mut deps = mock_dependencies();

        let _msg = InstantiateMsg {
            gov: Some(Addr::unchecked("gov")),
            ve_seilor_lock_settings: ve_seilor_lock_settings(),
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the config is stored correctly
        assert_eq!(
            get_boost_config(deps.as_ref()).unwrap(),
            GetBoostConfigResponse {
                gov: Addr::unchecked("gov"),
                ve_seilor_lock_settings: ve_seilor_lock_settings(),
                new_gov: None,
            }
        );

        let _msg = ExecuteMsg::AddLockSetting {
            duration: Uint128::from(61536000u128),
            mining_boost: Uint128::from(300000000u128),
        };
        let _info = mock_info("gov", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        assert_eq!(
            get_boost_config(deps.as_ref()).unwrap(),
            GetBoostConfigResponse {
                gov: Addr::unchecked("gov"),
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
                    VeSeilorLockSetting {
                        duration: Uint128::from(61536000u128),
                        mining_boost: Uint128::from(300000000u128),
                    },
                ],
                new_gov: None,
            }
        );
    }

    #[test]
    fn test_set_lock_status() {
        let mut deps = mock_dependencies();

        let _msg = InstantiateMsg {
            gov: Some(Addr::unchecked("gov")),
            ve_seilor_lock_settings: ve_seilor_lock_settings(),
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the config is stored correctly
        assert_eq!(
            get_boost_config(deps.as_ref()).unwrap(),
            GetBoostConfigResponse {
                gov: Addr::unchecked("gov"),
                ve_seilor_lock_settings: ve_seilor_lock_settings(),
                new_gov: None,
            }
        );

        let _msg = ExecuteMsg::SetLockStatus { index: 0 };
        let _info = mock_info("lucky", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // unlock_time = block.time.seconds(1_571_797_419) + duration
        assert_eq!(
            get_user_lock_status(deps.as_ref(), Addr::unchecked("lucky")).unwrap(),
            LockStatusResponse {
                unlock_time: Uint128::from(1574389419u128),
                duration: Uint128::from(2592000u128),
                mining_boost: Uint128::from(20000000u128),
            }
        );

        let _msg = ExecuteMsg::SetLockStatus { index: 1 };
        let _info = mock_info("lucky", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // unlock_time = block.time.seconds(1_571_797_419) + duration
        assert_eq!(
            get_user_lock_status(deps.as_ref(), Addr::unchecked("lucky")).unwrap(),
            LockStatusResponse {
                unlock_time: Uint128::from(1579573419u128),
                duration: Uint128::from(7776000u128),
                mining_boost: Uint128::from(30000000u128),
            }
        );

        // negative test case with lower duration
        let _msg = ExecuteMsg::SetLockStatus { index: 0 };
        let _info = mock_info("lucky", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg).unwrap_err();
        match _res {
            GenericErr { msg, .. } => {
                assert_eq!(msg, "Your lock-in period has not ended, and the term can only be extended, not reduced.".to_string())
            }
            _ => panic!("Set lock status return error"),
        }
    }
}
