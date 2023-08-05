use cosmwasm_std::{Env, TransactionInfo};
use std::collections::HashSet;
use std::num::ParseIntError;

const CHARACTERS: [char; 62] = [
    '2', 'f', 'x', 'n', 'l', 'G', 'j', 'O', 'T', 'h', 'u', 'k', 'r', 'Q', 'D', 'w', 'b', 'I', 'a',
    'Y', '4', 'Z', 'H', 'c', 'W', 'N', 'X', 'v', 'C', 'm', '7', 's', 'F', '1', '9', 'q', '5', 'J',
    'e', 'P', '0', '6', 'y', 'K', 'L', 'E', '8', 'p', 'i', 'M', 'R', 'A', 'B', '3', 'z', 't', 'U',
    'g', 'V', 'S', 'o', 'd',
];

fn _get_random_seed(env: Env, unique_factor: String, rand_factor: Vec<String>) -> String {
    let mut seed = rand_factor;
    let block_time = env.block.time.seconds();
    let mod_time = block_time % 62;
    seed.push(CHARACTERS[mod_time as usize].to_string());
    seed.push(unique_factor);
    seed.push(env.block.time.nanos().to_string());
    seed.push(env.block.height.to_string());
    seed.push(
        env.transaction
            .unwrap_or(TransactionInfo { index: 0 })
            .index
            .to_string(),
    );
    seed.join(",").to_string()
}

fn _cal_random_number(seed: &str) -> Result<u64, ParseIntError> {
    let result = sha256::digest(seed);
    let random_number_bytes_first = &result[..6]; // Take the first 5 bytes
    let random_number_bytes_last = &result[result.len() - 6..]; // Take the last 5 bytes
    let random_number_first = u64::from_str_radix(random_number_bytes_first, 16)?; // Convert to u64 (base 16
    let random_number_last = u64::from_str_radix(random_number_bytes_last, 16)?; // Convert to u64 (base 16
    let random_number = random_number_first + random_number_last; // Add the two numbers
    Ok(random_number)
}

pub fn get_winning(
    env: Env,
    unique_factor: String,
    rand_factor: Vec<String>,
    winning_num: &HashSet<u64>,
    mod_num: &u64,
) -> Result<bool, ParseIntError> {
    let seed = _get_random_seed(env, unique_factor.clone(), rand_factor);
    let random_number = _cal_random_number(&seed)?;
    let luck_num = random_number % mod_num;
    Ok(winning_num.contains(&luck_num))
}

#[cfg(test)]
mod tests {
    use crate::random_rules::get_winning;
    use cosmwasm_std::testing::mock_env;

    #[test]
    fn test_cal_random_number() {
        let env = mock_env();
        let res = get_winning(
            env,
            "test2".to_string(),
            vec!["0".to_string()],
            &vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24,
            ]
            .into_iter()
            .collect(),
            &100,
        );
        println!("res:{:?}", res);
    }
}
