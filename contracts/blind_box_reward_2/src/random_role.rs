use crate::error::ContractError;

fn cal_random_number(seed: &str) -> Result<u128, ContractError> {
    let result = sha256::digest(seed);
    let random_number_bytes_first = &result[..8]; // Take the first 5 bytes
    let random_number_bytes_last = &result[result.len() - 8..]; // Take the last 5 bytes
    let random_number_first = u128::from_str_radix(random_number_bytes_first, 16)
        .map_err(|_| ContractError::InvalidInput {})?;
    let random_number_last = u128::from_str_radix(random_number_bytes_last, 16)
        .map_err(|_| ContractError::InvalidInput {})?;
    let random_number = random_number_first.checked_add(random_number_last)
        .ok_or(ContractError::ArithmeticOverflow {})?;
    Ok(random_number)
}

pub fn cal_random_reward(seed: &str, remain_rewards_quantity: Vec<u128>) -> Result<(u128, u128, bool), ContractError> {
    let random_number = cal_random_number(seed)?;
    let remain_len = u128::try_from(remain_rewards_quantity.len())
        .map_err(|_| ContractError::InvalidInput {})?;
    if remain_len == 0 {
        return Ok((0, 0, false));
    }
    let reward_index = random_number % remain_len;
    let remain_reward_quantity = remain_rewards_quantity.get(reward_index as usize)
        .ok_or(ContractError::InvalidInput {})?
        .checked_sub(1)
        .ok_or(ContractError::ArithmeticUnderflow {})?;
    Ok((reward_index, remain_reward_quantity, true))
}


#[cfg(test)]
mod tests {
    use crate::random_role::cal_random_reward;

    #[test]
    fn test_cal_random_number() {
        let seed = "0D7207E77DD03C7C7ABA8548F56C7F1C02D30388BFB1B000C184AD1267163933".to_string();
        let random_number = super::cal_random_number(seed.as_str()).unwrap();
        println!("random_number: {:?}", random_number);
        assert_eq!(random_number, 6029877566u128);
    }

    #[test]
    fn test_cal_random_reward() {
        // Positive test case
        let seed = "0";
        let remain_rewards_quantity = vec![1];
        let (reward_index, remain_reward_quantity, op) = cal_random_reward(seed, remain_rewards_quantity).unwrap();
        assert_eq!(reward_index, 0);
        assert_eq!(remain_reward_quantity, 0);
        assert_eq!(op, true);
        // Positive test case
        let seed = "0";
        let remain_rewards_quantity = vec![1,10];
        let (reward_index, remain_reward_quantity, op) = cal_random_reward(seed, remain_rewards_quantity).unwrap();

        assert_eq!(reward_index, 1);
        assert_eq!(remain_reward_quantity, 9);
        assert_eq!(op, true);
        // Positive test case
        let seed = "6";
        let remain_rewards_quantity = vec![1,10,20];
        let (reward_index, remain_reward_quantity, op) = cal_random_reward(seed, remain_rewards_quantity).unwrap();
        println!("reward_index: {:?}", reward_index);
        println!("remain_reward_quantity: {:?}", remain_reward_quantity);
        println!("op: {:?}", op);
        assert_eq!(reward_index, 2);
        assert_eq!(remain_reward_quantity, 19);
        assert_eq!(op, true);
        // Negative test case with empty remain_rewards_quantity
        let seed = "0";
        let remain_rewards_quantity: Vec<u128> = vec![];
        let (reward_index, remain_reward_quantity, op) = cal_random_reward(seed, remain_rewards_quantity).unwrap();
        assert_eq!(reward_index, 0);
        assert_eq!(remain_reward_quantity, 0);
        println!("reward_index: {:?}", reward_index);
        println!("op: {:?}", op);
    }
}