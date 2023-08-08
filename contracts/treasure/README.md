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

## InstantiateMsg {.tabset}

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub lock_token: Addr,
    pub start_time: u64,
    pub end_time: u64,
    pub integral_reward_coefficient: Uint128,
    pub lock_duration: u64,
    pub punish_coefficient: Uint128,
    pub mint_nft_cost_integral: Uint128,
    pub winning_num: HashSet<u64>,
    pub mod_num: u64,
    pub punish_receiver: Addr,
}
```

### JSON

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
| `lock_token`                  | `Addr`          | The lock token contract address |
| `start_time`                  | `u64`           | The start time of the game      |
| `end_time`                    | `u64`           | The end time of the game        |
| `integral_reward_coefficient` | `Uint128`       | The integral reward coefficient |
| `lock_duration`               | `u64`           | The lock duration               |
| `punish_coefficient`          | `Uint128`       | The punish coefficient          |
| `mint_nft_cost_integral`      | `Uint128`       | The mint NFT cost integral      |
| `winning_num`                 | `HashSet<u64>`  | The winning numbers             |
| `mod_num`                     | `u64`           | The mod number                  |
| `punish_receiver`             | `Addr`          | The punish receiver address     |

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

### PreMintNft {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    PreMintNft { token_id: String },
}
```

#### JSON

```json
{
  "pre_mint_nft": {
    "mint_num": "1"
  }
}
```

| Key        | Type     | Description     |
|------------|----------|-----------------|
| `mint_num` | `String` | The mint number |

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
    QueryUserInfos { msg: QueryUserInfosMsg },
}
```

#### JSON

```json
{
  "user_infos": {}
}
```

### QueryUserInfosMsg {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct QueryUserInfosMsg {
    pub user_addr: Addr,
    pub query_user_state: bool,
    pub query_lock_records: bool,
    pub query_withdraw_records: bool,
    pub query_mint_nft_records: bool,
    pub start_after: Option<String>,
    pub limit: Option<u32>,
}
```

#### JSON

```json
{
  "user_addr": "sei1...",
  "query_user_state": true,
  "query_lock_records": true,
  "query_withdraw_records": true,
  "query_mint_nft_records": true,
  "start_after": "sei1...",
  "limit": 100
}
```

| Key                      | Type             | Description             |
|--------------------------|------------------|-------------------------|
| `user_addr`              | `Addr`           | The user address        |
| `query_user_state`       | `bool`           | Query user state        |
| `query_lock_records`     | `bool`           | Query lock records      |
| `query_withdraw_records` | `bool`           | Query withdraw records  |
| `query_mint_nft_records` | `bool`           | Query mint NFT records  |
| `start_after`            | `Option<String>` | The start after address |

### UserInfosResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UserInfosResponse {
    pub user_state: Option<crate::state::TreasureUserState>,
    pub lock_records: Option<Vec<crate::state::UserLockRecord>>,
    pub withdraw_records: Option<Vec<crate::state::UserWithdrawRecord>>,
    pub mint_nft_records: Option<Vec<crate::state::UserMintNftRecord>>,
}
```

#### JSON

```json
{
  "user_state": {},
  "lock_records": [],
  "withdraw_records": [],
  "mint_nft_records": []
}
```

| Key                | Type                                            | Description          |
|--------------------|-------------------------------------------------|----------------------|
| `user_state`       | `Option<crate::state::TreasureUserState>`       | The user state       |
| `lock_records`     | `Option<Vec<crate::state::UserLockRecord>>`     | The lock records     |
| `withdraw_records` | `Option<Vec<crate::state::UserWithdrawRecord>>` | The withdraw records |
| `mint_nft_records` | `Option<Vec<crate::state::UserMintNftRecord>>`  | The mint NFT records |

### TreasureUserState {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TreasureUserState {
    pub current_locked_amount: Uint128,
    pub current_integral_amount: Uint128,

    pub win_nft_num: u64,
    pub lose_nft_num: u64,

    pub start_lock_time: u64,
    pub end_lock_time: u64,

    pub total_locked_amount: Uint128,
    pub total_withdraw_amount: Uint128,
    pub total_punish_amount: Uint128,
    pub total_cost_integral_amount: Uint128,
}
```

#### JSON

```json
{
  "current_locked_amount": "1000000000",
  "current_integral_amount": "1000000000",
  "win_nft_num": 1000000000,
  "lose_nft_num": 1000000000,
  "start_lock_time": 1000000000,
  "end_lock_time": 1000000000,
  "total_locked_amount": "1000000000",
  "total_withdraw_amount": "1000000000",
  "total_punish_amount": "1000000000",
  "total_cost_integral_amount": "1000000000"
}
```

| Key                          | Type      | Description                    |
|------------------------------|-----------|--------------------------------|
| `current_locked_amount`      | `Uint128` | The current locked amount      |
| `current_integral_amount`    | `Uint128` | The current integral amount    |
| `win_nft_num`                | `u64`     | The win NFT number             |
| `lose_nft_num`               | `u64`     | The lose NFT number            |
| `start_lock_time`            | `u64`     | The start lock time            |
| `end_lock_time`              | `u64`     | The end lock time              |
| `total_locked_amount`        | `Uint128` | The total locked amount        |
| `total_withdraw_amount`      | `Uint128` | The total withdraw amount      |
| `total_punish_amount`        | `Uint128` | The total punish amount        |
| `total_cost_integral_amount` | `Uint128` | The total cost integral amount |

### UserLockRecord {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserLockRecord {
    pub record_id: u64,
    pub user_addr: Addr,
    pub lock_amount: Uint128,
    pub integral_reward_coefficient: Uint128,
    pub lock_duration: u64,
    pub start_lock_time: u64,
    pub end_lock_time: u64,
}
```

#### JSON

```json
{
  "record_id": 1000000000,
  "user_addr": "sei1...",
  "lock_amount": "1000000000",
  "integral_reward_coefficient": "1000000000",
  "lock_duration": 1000000000,
  "start_lock_time": 1000000000,
  "end_lock_time": 1000000000
}
```

| Key                           | Type      | Description                     |
|-------------------------------|-----------|---------------------------------|
| `record_id`                   | `u64`     | The record ID                   |
| `user_addr`                   | `Addr`    | The user address                |
| `lock_amount`                 | `Uint128` | The lock amount                 |
| `integral_reward_coefficient` | `Uint128` | The integral reward coefficient |
| `lock_duration`               | `u64`     | The lock duration               |
| `start_lock_time`             | `u64`     | The start lock time             |
| `end_lock_time`               | `u64`     | The end lock time               |

### UserWithdrawRecord {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserWithdrawRecord {
    pub record_id: u64,
    pub user_addr: Addr,
    pub withdraw_amount: Uint128,
    pub withdraw_time: u64,
    pub punish_amount: Uint128,
}
```

#### JSON

```json
{
  "record_id": 1000000000,
  "user_addr": "sei1...",
  "withdraw_amount": "1000000000",
  "withdraw_time": 1000000000,
  "punish_amount": "1000000000"
}
```

| Key               | Type      | Description         |
|-------------------|-----------|---------------------|
| `record_id`       | `u64`     | The record ID       |
| `user_addr`       | `Addr`    | The user address    |
| `withdraw_amount` | `Uint128` | The withdraw amount |
| `withdraw_time`   | `u64`     | The withdraw time   |
| `punish_amount`   | `Uint128` | The punish amount   |

### UserMintNftRecord {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserMintNftRecord {
    pub record_id: u64,
    pub user_addr: Addr,
    pub mint_nft_num: u64,
    pub win_nft_num: u64,
    pub mint_nft_cost_integral_amount: Uint128,
    pub mint_time: u64,
}
```

#### JSON

```json
{
  "record_id": 1000000000,
  "user_addr": "sei1...",
  "mint_nft_num": 1000000000,
  "win_nft_num": 1000000000,
  "mint_nft_cost_integral_amount": "1000000000",
  "mint_time": 1000000000
}
```

| Key                             | Type      | Description                       |
|---------------------------------|-----------|-----------------------------------|
| `record_id`                     | `u64`     | The record ID                     |
| `user_addr`                     | `Addr`    | The user address                  |
| `mint_nft_num`                  | `u64`     | The mint NFT number               |
| `win_nft_num`                   | `u64`     | The win NFT number                |
| `mint_nft_cost_integral_amount` | `Uint128` | The mint NFT cost integral amount |
| `mint_time`                     | `u64`     | The mint time                     |
