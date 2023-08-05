# Staking Rewards

veSEILOR holders receive a varied percentage of yield boost depending on the lock-up length. Details TBD.

## StakingConfig

| Key                      | Type | Description                   |
|--------------------------|------|-------------------------------|
| `gov`                    | Addr | The governance contract       |
| `staking_token`          | Addr | The token to be staked        |
| `rewards_token`          | Addr | The token to be reward        |
| `boost`           | Addr | The veSEILOR boost  contract     |
| `fund`               | Addr | The SEILOR fund contract         |
| `reward_controller_addr` | Addr | The reward controller address |

## InstantiateMsg .{tabset}

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub staking_token: Addr,
    pub rewards_token: Addr,
    pub boost: Addr,
    pub fund: Addr,
    pub reward_controller_addr: Addr,
    pub duration: Uint128,
}
```

### JSON

```json
{
  "gov": "sei1...",
  "staking_token": "sei1...",
  "rewards_token": "sei1...",
  "boost": "sei1...",
  "fund": "sei1...",
  "reward_controller_addr": "sei1...",
  "duration": "2592000"
}
```

| Key                      | Type      | Description                   |
|--------------------------|-----------|-------------------------------|
| `gov`                    | `Addr`    | The governance contract       |
| `staking_token`          | `Addr`    | The token to be staked        |
| `rewards_token`          | `Addr`    | The token to be reward        |
| `boost`           | `Addr`    | The veSEILOR boost  contract     |
| `fund`               | `Addr`    | The SEILOR fund contract         |
| `reward_controller_addr` | `Addr`    | The reward controller address |
| `duration`               | `Uint128` | The duration of the lock-up   |

## ExecuteMsg

### Receive .{tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    /// Receives a message of type [`Cw20ReceiveMsg`]
    Receive(Cw20ReceiveMsg),
}
```

#### JSON

```json
{
  "receive": {
    "cw20_receive_msg": {}
  }
}
```

### Cw20ReceiveMsg .{tabset}

#### Rust

```rust
/// Cw20ReceiveMsg should be de/serialized under `Receive()` variant in a ExecuteMsg
#[cw_serde]
pub struct Cw20ReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}
```

#### JSON

```json
{
  "sender": "sei1...",
  "amount": "1000000",
  "msg": "eyJ0eXBlIjoiY3cyMCIsImFtb3VudCI6IjEwMDAwMCJ9"
}
```

### UpdateStakingConfig .{tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateStakingConfig {
        config_msg: UpdateStakingConfigStruct,
    },
}
```

#### JSON

```json
{
  "update_staking_config": {
    "config_msg": {}
  }
}
```

### UpdateStakingConfigStruct .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct UpdateStakingConfigStruct {
    pub gov: Option<Addr>,
    pub staking_token: Option<Addr>,
    pub rewards_token: Option<Addr>,
    pub boost: Option<Addr>,
    pub fund: Option<Addr>,
    pub reward_controller_addr: Option<Addr>,
}
```

#### JSON

```json
{
  "gov": "sei1...",
  "staking_token": "sei1...",
  "rewards_token": "sei1...",
  "boost": "sei1...",
  "fund": "sei1...",
  "reward_controller_addr": "sei1..."
}
```

| Key                      | Type   | Description                   |
|--------------------------|--------|-------------------------------|
| `gov`                    | `Addr` | The governance contract       |
| `staking_token`          | `Addr` | The token to be staked        |
| `rewards_token`          | `Addr` | The token to be reward        |
| `boost`           | `Addr` | The veSEILOR boost  contract     |
| `fund`               | `Addr` | The SEILOR fund contract         |
| `reward_controller_addr` | `Addr` | The reward controller address |

### UpdateStakingState .{tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateStakingState {
        duration: Uint128,
    },
}
```

#### JSON

```json
{
  "update_staking_state": {
    "duration": "2592000"
  }
}
```

| Key        | Type      | Description                 |
|------------|-----------|-----------------------------|
| `duration` | `Uint128` | The duration of the lock-up |

### GetReward .{tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    GetReward {},
}
```

#### JSON

```json
{
  "get_reward": {}
}
```

| Key | Type | Description |
|-----|------|-------------|

### Withdraw .{tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Withdraw {
        amount: Uint128,
    },
}
```

#### JSON

```json
{
  "withdraw": {
    "amount": "1000000"
  }
}
```

| Key      | Type      | Description |
|----------|-----------|-------------|
| `amount` | `Uint128` | amount      |

### NotifyRewardAmount .{tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    NotifyRewardAmount {
        amount: Uint128,
    },
}
```

#### JSON

```json
{
  "notify_reward_amount": {
    "amount": "1000000"
  }
}
```

| Key      | Type      | Description |
|----------|-----------|-------------|
| `amount` | `Uint128` | amount      |

## QueryMsg

### RewardPerToken .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(RewardPerTokenResponse)]
    RewardPerToken {},
}
```

#### JSON

```json
{
  "reward_per_token": {}
}
```

### RewardPerTokenResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct RewardPerTokenResponse {
    pub reward_per_token: Uint128,
}
```

#### JSON

```json
{
  "reward_per_token": "1000000"
}
```

| Key                | Type      | Description      |
|--------------------|-----------|------------------|
| `reward_per_token` | `Uint128` | Reward per token |

### LastTimeRewardApplicable .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(LastTimeRewardApplicableResponse)]
    LastTimeRewardApplicable {},
}
```

#### JSON

```json
{
  "last_time_reward_applicable": {}
}
```

### LastTimeRewardApplicableResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct LastTimeRewardApplicableResponse {
    pub last_time_reward_applicable: Uint128,
}
```

#### JSON

```json
{
  "last_time_reward_applicable": "1000000"
}
```

| Key                           | Type      | Description                 |
|-------------------------------|-----------|-----------------------------|
| `last_time_reward_applicable` | `Uint128` | Last time reward applicable |

### GetBoost .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetBoostResponse)]
    GetBoost { account: Addr },
}
```

#### JSON

```json
{
  "get_boost": {
    "account": "sei1..."
  }
}
```

### GetBoostResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetBoostResponse {
    pub boost: Uint128,
}
```

#### JSON

```json
{
  "boost": "1000000"
}
```

| Key     | Type      | Description |
|---------|-----------|-------------|
| `boost` | `Uint128` | boost       |

### Earned .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(EarnedResponse)]
    Earned { account: Addr },
}
```

#### JSON

```json
{
  "earned": {
    "account": "sei1..."
  }
}
```

### EarnedResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct EarnedResponse {
    pub earned: Uint128,
}
```

#### JSON

```json
{
  "earned": "1000000"
}
```

| Key      | Type      | Description |
|----------|-----------|-------------|
| `earned` | `Uint128` | earned      |

### QueryStakingConfig .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StakingConfigResponse)]
    QueryStakingConfig {},
}
```

#### JSON

```json
{
  "query_staking_config": {}
}
```

### StakingConfigResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct StakingConfigResponse {
    pub gov: Addr,
    pub staking_token: Addr,
    pub rewards_token: Addr,
    pub boost: Addr,
    pub fund: Addr,
    pub reward_controller_addr: Addr,
}
```

#### JSON

```json
{
  "gov": "sei1...",
  "staking_token": "sei1...",
  "rewards_token": "sei1...",
  "boost": "sei1...",
  "fund": "sei1...",
  "reward_controller_addr": "sei1..."
}
```

| Key                      | Type   | Description              |
|--------------------------|--------|--------------------------|
| `gov`                    | `Addr` | Gov address              |
| `staking_token`          | `Addr` | Staking token address    |
| `rewards_token`          | `Addr` | Rewards token address    |
| `boost`                  | `Addr` | Boost address            |
| `fund`                   | `Addr` | Fund address             |
| `reward_controller_addr` | `Addr` | RewardController address |

### QueryStakingState .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StakingStateResponse)]
    QueryStakingState {},
}
```

#### JSON

```json
{
  "query_staking_state": {}
}
```

### StakingStateResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct StakingStateResponse {
    pub duration: Uint128,
    pub finish_at: Uint128,
    pub updated_at: Uint128,
    pub reward_rate: Uint256,
    pub reward_per_token_stored: Uint128,
    pub total_supply: Uint128,
}
```

#### JSON

```json
{
  "duration": "1000000",
  "finish_at": "1000000",
  "updated_at": "1000000",
  "reward_rate": "1000000",
  "reward_per_token_stored": "1000000",
  "total_supply": "1000000"
}
```

| Key                       | Type      | Description             |
|---------------------------|-----------|-------------------------|
| `duration`                | `Uint128` | Duration                |
| `finish_at`               | `Uint128` | Finish at               |
| `updated_at`              | `Uint128` | Updated at              |
| `reward_rate`             | `Uint256` | Reward rate             |
| `reward_per_token_stored` | `Uint128` | Reward per token stored |
| `total_supply`            | `Uint128` | Total supply            |

### GetUserUpdatedAt .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetUserUpdatedAtResponse)]
    GetUserUpdatedAt { account: Addr },
}
```

#### JSON

```json
{
  "get_user_updated_at": {
    "account": "sei1..."
  }
}
```

| Key       | Type   | Description |
|-----------|--------|-------------|
| `account` | `Addr` | Account     |

### GetUserUpdatedAtResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetUserUpdatedAtResponse {
    pub updated_at: Uint128,
}
```

#### JSON

```json
{
  "updated_at": "1000000"
}
```

| Key          | Type      | Description |
|--------------|-----------|-------------|
| `updated_at` | `Uint128` | Updated at  |

### GetUserRewardPerTokenPaid .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetUserRewardPerTokenPaidResponse)]
    GetUserRewardPerTokenPaid { account: Addr },
}
```

#### JSON

```json
{
  "get_user_reward_per_token_paid": {
    "account": "sei1..."
  }
}
```

| Key       | Type   | Description |
|-----------|--------|-------------|
| `account` | `Addr` | Account     |

### GetUserRewardPerTokenPaidResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetUserRewardPerTokenPaidResponse {
    pub reward_per_token_paid: Uint128,
}
```

#### JSON

```json
{
  "reward_per_token_paid": "1000000"
}
```

| Key                     | Type      | Description           |
|-------------------------|-----------|-----------------------|
| `reward_per_token_paid` | `Uint128` | Reward per token paid |

### BalanceOf .{tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BalanceOfResponse)]
    BalanceOf { account: Addr },
}
```

#### JSON

```json
{
  "balance_of": {
    "account": "sei1..."
  }
}
```

| Key       | Type   | Description |
|-----------|--------|-------------|
| `account` | `Addr` | Account     |

### BalanceOfResponse .{tabset}

#### Rust

```rust
#[cw_serde]
pub struct BalanceOfResponse {
    pub balance_of: Uint128,
}
```

#### JSON

```json
{
  "balance_of": "1000000"
}
```

| Key          | Type      | Description |
|--------------|-----------|-------------|
| `balance_of` | `Uint128` | Balance of  |