use std::collections::HashMap;
use cosmwasm_std::{Env, StdError, TransactionInfo};
// use rand::Rng;
use crate::error::ContractError;
use crate::state::{RandomBoxRewardRuleConfig, RandomBoxRewardRuleConfigState};

const CHARACTERS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z'
];

fn _get_random_seed(env: Env, unique_factor: String, rand_factor: Vec<String>) -> String {
    let mut seed = rand_factor;
    let block_time = env.block.time.seconds();
    let mod_time = block_time % 62;
    seed.push(CHARACTERS[mod_time as usize].to_string());
    seed.push(unique_factor);
    seed.push(env.block.time.nanos().to_string());
    seed.push(env.block.height.to_string());
    seed.push(env.transaction.unwrap_or(TransactionInfo { index: 0 }).index.to_string());
    seed.join(",").to_string()
}

fn _cal_random_number(seed: &str) -> Result<u64, ContractError> {
    let result = sha256::digest(seed);
    let random_number_bytes_first = &result[..6]; // Take the first 5 bytes
    let random_number_bytes_last = &result[result.len() - 6..]; // Take the last 5 bytes
    let random_number_first = u64::from_str_radix(random_number_bytes_first, 16)
        .map_err(|_| ContractError::InvalidInput {})?;
    let random_number_last = u64::from_str_radix(random_number_bytes_last, 16)
        .map_err(|_| ContractError::InvalidInput {})?;
    let random_number = random_number_first.checked_add(random_number_last)
        .ok_or(ContractError::ArithmeticOverflow {})?;
    Ok(random_number)
}

pub fn find_random_rule(
    env: Env,
    token_id: String,
    rules: &Vec<RandomBoxRewardRuleConfig>, rules_state: &Vec<RandomBoxRewardRuleConfigState>)
    -> Result<RandomBoxRewardRuleConfig, ContractError> {
    if rules.len() != rules_state.len() {
        return Err(ContractError::Std(StdError::generic_err("rules and rules_state length not equal")));
    }

    // get valid rules

    let mut interval_rate_map: HashMap<usize, (u64, u64)> = HashMap::new();
    let mut div_mod = 0;
    for (index, (rule, rule_state)) in rules.iter().zip(rules_state.iter()).enumerate() {
        if rule_state.total_open_box_count < rule.random_total_count {
            let end = div_mod + rule.random_total_count;
            interval_rate_map.insert(index, (div_mod, end));
            div_mod = end;
        }
    }
    if div_mod == 0 {
        return Err(ContractError::Std(StdError::generic_err("mod is zero,can not find valid rule")));
    }

    // let mut rng = rand::thread_rng();
    // let random_number = rng.gen_range(0..div_mod);

    let seed = _get_random_seed(env, token_id.clone(), vec![]);
    let random_number = _cal_random_number(&seed)?;
    let luck_num = random_number % div_mod;


    let res_rule = interval_rate_map.iter()
        .find(|(_, (start, end))| luck_num >= *start && luck_num < *end)
        .map(|(index, _)| rules.get(*index))
        .flatten()
        .cloned()
        .ok_or(ContractError::Std(StdError::generic_err("find rule error")))?;

    Ok(res_rule)
}

pub fn random_num(env: Env, token_ids: Vec<String>) -> Result<HashMap<String, u64>, ContractError> {
    let mut res_map = HashMap::new();
    for token_id in token_ids {
        let seed = _get_random_seed(env.clone(), token_id.clone(), vec![]);
        let random_number = _cal_random_number(&seed)?;
        res_map.insert(token_id, random_number);
    }
    Ok(res_map)
}


#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_env;
    use crate::random_role::{_cal_random_number, find_random_rule};
    use crate::state::{RandomBoxRewardRuleConfig, RandomBoxRewardRuleConfigState};

    #[test]
    fn test_cal_random_number() {
        let seed = "test2";
        let result = _cal_random_number(seed);
        println!("result:{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_random_rule_with_valid_seed() {
        let rules = vec![
            RandomBoxRewardRuleConfig { random_box_index: 0, random_total_count: 5, random_reward_amount: 0, max_reward_count: 0 },
            RandomBoxRewardRuleConfig { random_box_index: 0, random_total_count: 10, random_reward_amount: 0, max_reward_count: 0 },
            RandomBoxRewardRuleConfig { random_box_index: 0, random_total_count: 15, random_reward_amount: 0, max_reward_count: 0 },
        ];
        let rules_state = vec![
            RandomBoxRewardRuleConfigState { total_reward_amount: 0, total_open_box_count: 5 },
            RandomBoxRewardRuleConfigState { total_reward_amount: 0, total_open_box_count: 1 },
            RandomBoxRewardRuleConfigState { total_reward_amount: 0, total_open_box_count: 15 },
        ];
        let env = mock_env();
        let token_id = "001";
        let result = find_random_rule(env, token_id.to_string(), &rules, &rules_state);
        println!("result:{:?}", result);
        assert!(result.is_ok());
        let res_rule = result.unwrap();
        assert_eq!(res_rule.random_total_count, 10);
    }
}