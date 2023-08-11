# Treasure

## Description

This contract is for managing the treasure. It is used for storing treasures and distributing them.

### Random rules

The given code is a Rust implementation of a random number generation and checking algorithm. Here is a breakdown of the
code:

1. The  `CHARACTERS`  constant is an array of characters used for generating a random seed.

2. The  `_get_random_seed`  function takes in the environment variables, a unique factor, and a vector of random
   factors. It generates a random seed by appending various values like block time, unique factor, transaction index,
   etc. The seed is created by concatenating these values with commas.

3. The  `_cal_random_number`  function takes a seed as input and computes a random number using a hash function. It
   takes the first and last 6 bytes of the hash result, converts them to u64 (base 16), and adds them together to get
   the final random number.

4. The  `_compute_hash`  function takes an input string and computes its SHA-256 hash using the  `sha2`  crate. The
   resulting hash is then encoded in hexadecimal format.

5. The  `get_winning`  function is the main function that is called to check if a given set of winning numbers contains
   the luck number generated from the random seed. It calculates the seed, generates the random number, and checks if
   the luck number is present in the winning numbers set.

In summary, this code generates a random number based on various factors and checks if it matches any of the winning
numbers. The random number generation is based on a seed derived from environment variables and other factors.

## TreasureConfig

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureConfig {
    pub gov: Addr,
    pub lock_token: Addr,
    pub start_lock_time: u64,
    pub end_lock_time: u64,
    //dust reward per second
    pub dust_reward_per_second: Uint128,
    pub withdraw_delay_duration: u64,
    // no delay punish coefficient
    pub no_delay_punish_coefficient: Uint128,
    // punish receiver
    pub punish_receiver: Addr,
    // nft start pre mint time
    pub nft_start_pre_mint_time: u64,
    // nft end pre mint time
    pub nft_end_pre_mint_time: u64,
    // nft cost dust
    pub mint_nft_cost_dust: Uint128,
    pub winning_num: HashSet<u64>,
    pub mod_num: u64,
}
```

| Key                           | Type           | Description                     |
|-------------------------------|----------------|---------------------------------|
| `gov`                         | `Addr`         | The governance contract address |
| `lock_token`                  | `Addr`         | The lock token contract address |
| `start_lock_time`             | `u64`          | The start time of the game      |
| `end_lock_time`               | `u64`          | The end time of the game        |
| `dust_reward_per_second`      | `Uint128`      | The dust reward per second      |
| `withdraw_delay_duration`     | `u64`          | The withdraw delay duration     |
| `no_delay_punish_coefficient` | `Uint128`      | The no delay punish coefficient |
| `punish_receiver`             | `Addr`         | The punish receiver address     |
| `nft_start_pre_mint_time`     | `u64`          | The NFT start pre mint time     |
| `nft_end_pre_mint_time`       | `u64`          | The NFT end pre mint time       |
| `mint_nft_cost_dust`          | `Uint128`      | The mint NFT cost dust          |
| `winning_num`                 | `HashSet<u64>` | The winning numbers             |
| `mod_num`                     | `u64`          | The mod number                  |

## InstantiateMsg {.tabset}

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub lock_token: Addr,
    pub start_lock_time: u64,
    pub end_lock_time: u64,
    pub dust_reward_per_second: Uint128,
    pub withdraw_delay_duration: u64,
    pub no_delay_punish_coefficient: Uint128,
    pub punish_receiver: Addr,
    pub nft_start_pre_mint_time: u64,
    pub nft_end_pre_mint_time: u64,
    pub mint_nft_cost_dust: Uint128,
    pub winning_num: HashSet<u64>,
    pub mod_num: u64,
}
```

### JSON

```json
{
  "gov": "sei1...",
  "lock_token": "sei1...",
  "start_lock_time": "1620000000",
  "end_lock_time": "1620000000",
  "dust_reward_per_second": "1000000000",
  "withdraw_delay_duration": "86400",
  "no_delay_punish_coefficient": "1000000000",
  "punish_receiver": "sei1...",
  "nft_start_pre_mint_time": "1620000000",
  "nft_end_pre_mint_time": "1620000000",
  "mint_nft_cost_dust": "1000000000",
  "winning_num": [
    1,
    2,
    3
  ],
  "mod_num": 100
}
```

| Key                           | Type            | Description                     |
|-------------------------------|-----------------|---------------------------------|
| `gov`                         | `Option<Addr>`* | The governance contract address |
| `lock_token`                  | `Addr`          | The lock token contract address |
| `start_lock_time`             | `u64`           | The start time of the game      |
| `end_lock_time`               | `u64`           | The end time of the game        |
| `dust_reward_per_second`      | `Uint128`       | The dust reward per second      |
| `withdraw_delay_duration`     | `u64`           | The withdraw delay duration     |
| `no_delay_punish_coefficient` | `Uint128`       | The no delay punish coefficient |
| `punish_receiver`             | `Addr`          | The punish receiver address     |
| `nft_start_pre_mint_time`     | `u64`           | The NFT start pre mint time     |
| `nft_end_pre_mint_time`       | `u64`           | The NFT end pre mint time       |
| `mint_nft_cost_dust`          | `Uint128`       | The mint NFT cost dust          |
| `winning_num`                 | `HashSet<u64>`  | The winning numbers             |
| `mod_num`                     | `u64`           | The mod number                  |

* = optional

## ExecuteMsg

### Receive {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
}
```

### JSON

```json
 {
  "amount": "1000000000",
  "sender": "sei1...",
  "msg": "eyJhY2NvdW50IjoiMTAw"
}
```

| Key       | Type             | Description                                                                       |
|-----------|------------------|-----------------------------------------------------------------------------------|
| `receive` | `Cw20ReceiveMsg` | The CW20 receive message, which is used to deposit the lock token to the contract |

### UpdateConfig {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(TreasureConfigMsg),
}
```

#### JSON

```json
{
  "gov": "sei1...",
  "lock_token": "sei1...",
  "start_time": "1688128677",
  "end_time": "1690720710",
  "integral_reward_coefficient": "10",
  "lock_duration": "2592000",
  "punish_coefficient": "300000",
  "mint_nft_cost_integral": "10000000000",
  "winning_num": "[0,1,2,...,22,23,24,]",
  "mod_num": "100",
  "punish_receiver": "sei1..."
}
```

| Key                           | Type            | Description                     |
|-------------------------------|-----------------|---------------------------------|
| `gov`                         | `Option<Addr>`* | The governance contract address |
| `lock_token`                  | `Addr`*         | The lock token contract address |
| `start_time`                  | `u64`*          | The start time of the game      |
| `end_time`                    | `u64`*          | The end time of the game        |
| `integral_reward_coefficient` | `Uint128`*      | The integral reward coefficient |
| `lock_duration`               | `u64`*          | The lock duration               |
| `punish_coefficient`          | `Uint128`*      | The punish coefficient          |
| `mint_nft_cost_integral`      | `Uint128`*      | The mint NFT cost integral      |
| `winning_num`                 | `HashSet<u64>`* | The winning numbers             |
| `mod_num`                     | `u64`*          | The mod number                  |
| `punish_receiver`             | `Addr`*         | The punish receiver address     |

* = optional

### UserWithdraw {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UserWithdraw { amount: Uint128 },
}
```

#### JSON

```json
{
  "user_withdraw": {
    "amount": "1000000000"
  }
}
```

| Key      | Type      | Description                                            |
|----------|-----------|--------------------------------------------------------|
| `amount` | `Uint128` | The amount of lock token to withdraw from the contract |

### UserUnlock {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UserUnlock { amount: Uint128 },
}
```

#### JSON

```json
{
  "user_unlock": {
    "amount": "1000000000"
  }
}
```

| Key      | Type      | Description                                          |
|----------|-----------|------------------------------------------------------|
| `amount` | `Uint128` | The amount of lock token to unlock from the contract |

### PreMintNft {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    PreMintNft { mint_num: u64 },
}
```

#### JSON

```json
{
  "pre_mint_nft": {
    "mint_num": 1
  }
}
```

| Key        | Type  | Description     |
|------------|-------|-----------------|
| `mint_num` | `u64` | The mint number |

## QueryMsg

### QueryConfigInfos {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigInfosResponse)]
    QueryConfigInfos {},
}
```

#### JSON

```json
{
  "config_infos": {}
}
```

### ConfigInfosResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct ConfigInfosResponse {
    pub config: crate::state::TreasureConfig,
    pub state: crate::state::TreasureState,
}
```

#### JSON

```json
{
  "config": {},
  "state": {}
}
```

| Key      | Type             | Description         |
|----------|------------------|---------------------|
| `config` | `TreasureConfig` | The treasure config |
| `state`  | `TreasureState`  | The treasure state  |

### TreasureConfig {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureConfig {
    pub gov: Addr,
    pub lock_token: Addr,
    pub start_time: u64,
    pub end_time: u64,
    // Integral reward coefficient
    pub integral_reward_coefficient: Uint128,
    pub lock_duration: u64,
    // punish coefficient
    pub punish_coefficient: Uint128,
    // nft cost integral
    pub mint_nft_cost_integral: Uint128,
    pub winning_num: HashSet<u64>,
    pub mod_num: u64,
    // punish receiver
    pub punish_receiver: Addr,
}
```

#### JSON

```json
{
  "gov": "sei1...",
  "lock_token": "sei1...",
  "start_time": "1688128677",
  "end_time": "1690720710",
  "integral_reward_coefficient": "10",
  "lock_duration": "2592000",
  "punish_coefficient": "300000",
  "mint_nft_cost_integral": "10000000000",
  "winning_num": "[0,1,2,...,22,23,24,]",
  "mod_num": "100",
  "punish_receiver": "sei1..."
}
```

| Key                           | Type           | Description                     |
|-------------------------------|----------------|---------------------------------|
| `gov`                         | `Addr`         | The governance contract address |
| `lock_token`                  | `Addr`         | The lock token contract address |
| `start_time`                  | `u64`          | The start time of the game      |
| `end_time`                    | `u64`          | The end time of the game        |
| `integral_reward_coefficient` | `Uint128`      | The integral reward coefficient |
| `lock_duration`               | `u64`          | The lock duration               |
| `punish_coefficient`          | `Uint128`      | The punish coefficient          |
| `mint_nft_cost_integral`      | `Uint128`      | The mint NFT cost integral      |
| `winning_num`                 | `HashSet<u64>` | The winning numbers             |
| `mod_num`                     | `u64`          | The mod number                  |
| `punish_receiver`             | `Addr`         | The punish receiver address     |

### TreasureState {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureState {
    pub current_locked_amount: Uint128,
    pub current_integral_amount: Uint128,
    pub total_locked_amount: Uint128,
    pub total_withdraw_amount: Uint128,
    pub total_punish_amount: Uint128,
    pub total_win_nft_num: u64,
    pub total_lose_nft_num: u64,
}
```

#### JSON

```json
{
  "current_locked_amount": "1000000000",
  "current_integral_amount": "1000000000",
  "total_locked_amount": "1000000000",
  "total_withdraw_amount": "1000000000",
  "total_punish_amount": "1000000000",
  "total_win_nft_num": "1000000000",
  "total_lose_nft_num": "1000000000"
}
```

| Key                       | Type      | Description                 |
|---------------------------|-----------|-----------------------------|
| `current_locked_amount`   | `Uint128` | The current locked amount   |
| `current_integral_amount` | `Uint128` | The current integral amount |
| `total_locked_amount`     | `Uint128` | The total locked amount     |
| `total_withdraw_amount`   | `Uint128` | The total withdraw amount   |
| `total_punish_amount`     | `Uint128` | The total punish amount     |
| `total_win_nft_num`       | `u64`     | The total win NFT number    |
| `total_lose_nft_num`      | `u64`     | The total lose NFT number   |

### QueryUserInfos {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserInfosResponse)]
    QueryUserInfos { user: Addr },
}
```

#### JSON

```json
{
  "user_infos": {
    "user": "sei1..."
  }
}
```

### UserInfosResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UserInfosResponse {
    pub user_state: Option<crate::state::TreasureUserState>,
}
```

#### JSON

```json
{
  "user_state": {}
}
```

| Key          | Type                                      | Description    |
|--------------|-------------------------------------------|----------------|
| `user_state` | `Option<crate::state::TreasureUserState>` | The user state |

### TreasureUserState {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureUserState {
    pub current_locked_amount: Uint128,
    pub last_lock_time: u64,
    pub current_unlock_amount: Uint128,
    pub last_unlock_time: u64,
    pub current_dust_amount: Uint128,

    pub win_nft_num: u64,
    pub lose_nft_num: u64,

    pub total_locked_amount: Uint128,
    pub total_unlock_amount: Uint128,
    pub total_withdraw_amount: Uint128,
    pub total_punish_amount: Uint128,
    pub total_cost_dust_amount: Uint128,
}
```

#### JSON

```json
{
  "current_locked_amount": "1000000000",
  "last_lock_time": "1000000000",
  "current_unlock_amount": "1000000000",
  "last_unlock_time": "1000000000",
  "current_dust_amount": "1000000000",
  "win_nft_num": "1000000000",
  "lose_nft_num": "1000000000",
  "total_locked_amount": "1000000000",
  "total_unlock_amount": "1000000000",
  "total_withdraw_amount": "1000000000",
  "total_punish_amount": "1000000000",
  "total_cost_dust_amount": "1000000000"
}
```
