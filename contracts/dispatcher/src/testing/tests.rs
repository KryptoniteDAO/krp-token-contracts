use crate::handler::{add_users, update_config};
use crate::msg::{AddUserMsg, UpdateGlobalConfigMsg};
use crate::testing::mock_fn::{mock_instantiate, mock_instantiate_msg, CLAIM_TOKEN};
use cosmwasm_std::{Addr, Uint256};

#[test]
fn test_instantiate() {
    let msg = mock_instantiate_msg(Addr::unchecked(CLAIM_TOKEN.clone()));
    let (_, _, _, res) = mock_instantiate(msg.clone());
    assert_eq!(0, res.messages.len());
    assert_eq!(res.attributes.len(), 1);
}
#[test]
fn test_update_config() {
    let msg = mock_instantiate_msg(Addr::unchecked(CLAIM_TOKEN.clone()));
    let (mut deps, _env, info, _) = mock_instantiate(msg.clone());

    let res = add_users(
        deps.as_mut(),
        info.clone(),
        vec![AddUserMsg {
            user: Addr::unchecked("user1".to_string()),
            unlock_amount: Uint256::from(11114u128),
            lock_amount: Uint256::from(11114u128),
            replace: false,
        }],
    );
    assert!(res.is_ok());

    let res = update_config(
        deps.as_mut(),
        info.clone(),
        UpdateGlobalConfigMsg {
            gov: Option::from(Addr::unchecked("new_gov".to_string())),
            claim_token: Option::from(Addr::unchecked("new_claim_token".to_string())),
            start_time: Option::from(11111),
            end_regret_time: Option::from(11112),
            regret_token_receiver: Option::from(Addr::unchecked(
                "new_regret_token_receiver".to_string(),
            )),
            total_lock_amount: Option::from(Uint256::from(11113u128)),
            total_unlock_amount: Option::from(Uint256::from(11114u128)),
        },
    );
    assert!(res.is_err());

    let res = update_config(
        deps.as_mut(),
        info.clone(),
        UpdateGlobalConfigMsg {
            gov: Option::from(Addr::unchecked("new_gov".to_string())),
            claim_token: Option::from(Addr::unchecked("new_claim_token".to_string())),
            start_time: Option::from(11111),
            end_regret_time: Option::from(11112),
            regret_token_receiver: Option::from(Addr::unchecked(
                "new_regret_token_receiver".to_string(),
            )),
            total_lock_amount: Option::from(Uint256::from(11115u128)),
            total_unlock_amount: Option::from(Uint256::from(11116u128)),
        },
    );
    assert!(res.is_ok());

    let new_config = crate::querier::query_global_infos(deps.as_ref()).unwrap();
    assert_eq!(
        new_config.config.gov,
        Addr::unchecked("new_gov".to_string())
    );
    assert_eq!(
        new_config.config.claim_token,
        Addr::unchecked("new_claim_token".to_string())
    );
    assert_eq!(new_config.config.start_time, 11111);
    assert_eq!(new_config.config.end_regret_time, 11112);
    assert_eq!(
        new_config.config.regret_token_receiver,
        Addr::unchecked("new_regret_token_receiver".to_string())
    );
    assert_eq!(
        new_config.config.total_lock_amount,
        Uint256::from(11115u128)
    );
    assert_eq!(
        new_config.config.total_unlock_amount,
        Uint256::from(11116u128)
    );
}
