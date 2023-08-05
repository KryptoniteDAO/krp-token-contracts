# SEILOR Fund

Fund is a derivative version of Synthetix Staking Rewards , distributing Protocol revenue to veSEILOR stakers.

## FundConfig

| Key                             | Type            | Description                   |
|---------------------------------|-----------------|-------------------------------|
| `gov`                           | Addr            | gov contract                  |
| `ve_seilor_addr`                   | Addr            | veSEILOR contract                |
| `seilor_addr`                      | Addr            | SEILOR contract                  |
| `kusd_denom`                    | String          | KUSD denom                    |
| `kusd_reward_addr`              | Addr            | KUSD reward contract          |
| `kusd_reward_total_amount`      | Uint128         | KUSD reward total amount      |
| `kusd_reward_total_paid_amount` | Uint128         | KUSD reward total paid amount |
| `reward_per_token_stored`       | Uint128         | reward per token stored       |
| `exit_cycle`                    | claim_able_time | exit cycle                    |
| `claim_able_time`               | claim_able_time | claim able time               |

## InstantiateMsg {.tabset}

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub ve_seilor_addr: Addr,
    pub seilor_addr: Addr,
    pub kusd_denom: String,
    pub kusd_reward_addr: Addr,
    pub exit_cycle: Uint64,
    pub claim_able_time: Uint64,
}
```

```json
{
  "gov": "sei1...",
  "ve_seilor_addr": "sei1...",
  "seilor_addr": "sei1...",
  "kusd_denom": "factor/sei1.../KUSD",
  "kusd_reward_addr": "sei2...",
  "exit_cycle": "2592000",
  "claim_able_time": "1687190400"
}
```

| Key                | Type            | Description          |
|--------------------|-----------------|----------------------|
| `gov`              | Addr*           | gov contract         |
| `ve_seilor_addr`      | Addr            | veSEILOR contract       |
| `seilor_addr`         | Addr            | SEILOR contract         |
| `kusd_denom`       | String          | KUSD denom           |
| `kusd_reward_addr` | Addr            | KUSD reward contract |
| `exit_cycle`       | claim_able_time | exit cycle           |
| `claim_able_time`  | claim_able_time | claim able time      |

* = optional

## ExecuteMsg

### UpdateFundConfig {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateFundConfig { update_config_msg: UpdateConfigMsg },
}
```

#### JSON

```json
{
  "update_fund_config": {
    "update_config_msg": {}
  }
}
```

| Key                      | Type            | Description       |
|--------------------------|-----------------|-------------------|
| `update_fund_config` | UpdateConfigMsg | update config msg |

### UpdateConfigMsg {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UpdateConfigMsg {
    pub gov: Option<Addr>,
    pub ve_seilor_addr: Option<Addr>,
    pub seilor_addr: Option<Addr>,
    pub kusd_denom: Option<String>,
    pub kusd_reward_addr: Option<Addr>,
    pub claim_able_time: Option<Uint64>,
}
```

#### JSON

```json
{
  "gov": "sei1...",
  "ve_seilor_addr": "sei1...",
  "seilor_addr": "sei1...",
  "kusd_denom": "factor/sei1.../KUSD",
  "kusd_reward_addr": "sei2...",
  "claim_able_time": "1687190400"
}
```

| Key                | Type            | Description          |
|--------------------|-----------------|----------------------|
| `gov`              | Addr*           | gov contract         |
| `ve_seilor_addr`      | Addr*           | veSEILOR contract       |
| `seilor_addr`         | Addr*           | SEILOR contract         |
| `kusd_denom`       | String*         | KUSD denom           |
| `kusd_reward_addr` | Addr*           | KUSD reward contract |
| `claim_able_time`  | claim_able_time | claim able time      |

* = optional

### RefreshReward {.tabset}

Update user reward.

#### Rust

```rust

#[cw_serde]
pub enum ExecuteMsg {
    RefreshReward { account: Addr },
}
```

#### JSON

```json
{
  "refresh_reward": {
    "account": "sei1..."
  }
}
```

| Key              | Type | Description |
|------------------|------|-------------|
| `refresh_reward` | Addr | account     |

### Stake {.tabset}

Stake SEILOR.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Stake { amount: Uint128 },
}
```

#### JSON

```json
{
  "stake": {
    "amount": "1000000000000000000"
  }
}
```

| Key     | Type    | Description |
|---------|---------|-------------|
| `stake` | Uint128 | amount      |

### Unstake {.tabset}

Unstake SEILOR.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Unstake { amount: Uint128 },
}
```

#### JSON

```json
{
  "unstake": {
    "amount": "1000000000000000000"
  }
}
```

| Key       | Type    | Description |
|-----------|---------|-------------|
| `unstake` | Uint128 | amount      |

### Withdraw {.tabset}

Withdraw SEILOR.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Withdraw { amount: Uint128 },
}
```

#### JSON

```json
{
  "withdraw": {
    "amount": "1000000000000000000"
  }
}
```

| Key        | Type    | Description |
|------------|---------|-------------|
| `withdraw` | Uint128 | amount      |

### ReStake {.tabset}

Re-stake user SEILOR.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    ReStake {},
}
```

#### JSON

```json
{
  "re_stake": {}
}
```

| Key | Type | Description |
|-----|------|-------------|

### GetReward {.tabset}

Get user KUSD reward.

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

### NotifyRewardAmount {.tabset}

Notify KUSD reward amount.(access control)

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    NotifyRewardAmount { reward: Uint128 },
}
```

#### JSON

```json
{
  "notify_reward_amount": {
    "reward": "1000000000000000000"
  }
}
```

## QueryMsg

### FundConfig {.tabset}

Query FundConfig info.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(FundConfigResponse)]
    FundConfig {},
}
```

#### JSON

```json
{
  "fund_config": {}
}
```

### FundConfigResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct FundConfigResponse {
    pub gov: Addr,
    pub ve_seilor_addr: Addr,
    pub seilor_addr: Addr,
    pub kusd_denom: String,
    pub kusd_reward_addr: Addr,
    pub kusd_reward_total_amount: Uint128,
    pub kusd_reward_total_paid_amount: Uint128,
    // Sum of (reward rate * dt * 1e18 / total supply)
    pub reward_per_token_stored: Uint128,
    // uint256 immutable exitCycle = 30 days;
    pub exit_cycle: Uint64,
    // uint256 public claimAbleTime;
    pub claim_able_time: Uint64,
}
```

#### JSON

```json
{
  "gov": "sei1...",
  "ve_seilor_addr": "sei1...",
  "seilor_addr": "sei1...",
  "kusd_denom": "factor/sei1.../KUSD",
  "kusd_reward_addr": "sei2...",
  "kusd_reward_total_amount": "1000000000000000000",
  "kusd_reward_total_paid_amount": "1000000000000000000",
  "reward_per_token_stored": "1000000000000000000",
  "exit_cycle": "2592000",
  "claim_able_time": "1687190400"
}
```

| Key                             | Type    | Description                                     |
|---------------------------------|---------|-------------------------------------------------|
| `gov`                           | Addr    | gov contract                                    |
| `ve_seilor_addr`                   | Addr    | veSEILOR contract                                  |
| `seilor_addr`                      | Addr    | SEILOR contract                                    |
| `kusd_denom`                    | String  | KUSD denom                                      |
| `kusd_reward_addr`              | Addr    | KUSD reward contract                            |
| `kusd_reward_total_amount`      | Uint128 | KUSD reward total amount                        |
| `kusd_reward_total_paid_amount` | Uint128 | KUSD reward total paid amount                   |
| `reward_per_token_stored`       | Uint128 | Sum of (reward rate * dt * 1e18 / total supply) |
| `exit_cycle`                    | Uint64  | uint256 immutable exitCycle = 30 days;          |
| `claim_able_time`               | Uint64  | uint256 public claimAbleTime;                   |

### GetClaimAbleSeilor {.tabset}

Query claim able SEILOR amount.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetClaimAbleSeilorResponse)]
    GetClaimAbleSeilor { user: Addr },
}
```

#### JSON

```json
{
  "get_claim_able_seilor": {
    "user": "sei1..."
  }
}
```

| Key                  | Type | Description  |
|----------------------|------|--------------|
| `get_claim_able_seilor` | Addr | user account |

### GetClaimAbleSeilorResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetClaimAbleSeilorResponse {
    pub amount: Uint128,
}
```

#### JSON

```json
{
  "amount": "1000000000000000000"
}
```

| Key      | Type    | Description |
|----------|---------|-------------|
| `amount` | Uint128 | amount      |

### GetReservedKptForVesting {.tabset}

Query reserved SEILOR for vesting.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetReservedKptForVestingResponse)]
    GetReservedKptForVesting { user: Addr },
}
```

#### JSON

```json
{
  "get_reserved_kpt_for_vesting": {
    "user": "sei1..."
  }
}
```

| Key                            | Type | Description  |
|--------------------------------|------|--------------|
| `get_reserved_kpt_for_vesting` | Addr | user account |

### GetReservedKptForVestingResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetReservedKptForVestingResponse {
    pub amount: Uint128,
}
```

#### JSON

```json
{
  "amount": "1000000000000000000"
}
```

| Key      | Type    | Description |
|----------|---------|-------------|
| `amount` | Uint128 | amount      |

### Earned {.tabset}

Query earned KUSD reward amount.

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

| Key      | Type | Description  |
|----------|------|--------------|
| `earned` | Addr | user account |

### EarnedResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct EarnedResponse {
    pub amount: Uint128,
}
```

#### JSON

```json
{
  "amount": "1000000000000000000"
}
```

| Key      | Type    | Description |
|----------|---------|-------------|
| `amount` | Uint128 | amount      |

### GetClaimAbleKusd {.tabset}

Query claim able KUSD amount.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetClaimAbleKusdResponse)]
    GetClaimAbleKusd { account: Addr },
}
```

#### JSON

```json
{
  "get_claim_able_kusd": {
    "account": "sei1..."
  }
}
```

| Key       | Type | Description  |
|-----------|------|--------------|
| `account` | Addr | user account |

### GetClaimAbleKusdResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetClaimAbleKusdResponse {
    pub amount: Uint128,
}
```

#### JSON

```json
{
  "amount": "1000000000000000000"
}
```

| Key      | Type    | Description |
|----------|---------|-------------|
| `amount` | Uint128 | amount      |

### GetUserRewardPerTokenPaid {.tabset}

Query user reward per token paid.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserRewardPerTokenPaidResponse)]
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

| Key       | Type | Description  |
|-----------|------|--------------|
| `account` | Addr | user account |

### GetUserRewardPerTokenPaidResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UserRewardPerTokenPaidResponse {
    pub user_reward_per_token_paid: Uint128,
}
```

#### JSON

```json
    {
  "user_reward_per_token_paid": "1000000000000000000"
}
```

| Key                          | Type    | Description |
|------------------------------|---------|-------------|
| `user_reward_per_token_paid` | Uint128 | amount      |

### GetUserRewards {.tabset}

Query user rewards.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserRewardsResponse)]
    GetUserRewards { account: Addr },
}
```

#### JSON

```json
{
  "get_user_rewards": {
    "account": "sei1..."
  }
}
```

| Key       | Type | Description  |
|-----------|------|--------------|
| `account` | Addr | user account |

### UserRewardsResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UserRewardsResponse {
    pub user_rewards: Uint128,
}
```

#### JSON

```json
{
  "user_rewards": "1000000000000000000"
}
```

| Key            | Type    | Description |
|----------------|---------|-------------|
| `user_rewards` | Uint128 | amount      |

### GetUserTime2fullRedemption {.tabset}

Query user time to full redemption.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserTime2fullRedemptionResponse)]
    GetUserTime2fullRedemption { account: Addr },
}
```

#### JSON

```json
{
  "get_user_time2full_redemption": {
    "account": "sei1..."
  }
}
```

| Key       | Type | Description  |
|-----------|------|--------------|
| `account` | Addr | user account |

### UserTime2fullRedemptionResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UserTime2fullRedemptionResponse {
    pub user_time2full_redemption: Uint64,
}
```

#### JSON

```json
{
  "user_time2full_redemption": "1000000000000000000"
}
```

| Key                         | Type   | Description |
|-----------------------------|--------|-------------|
| `user_time2full_redemption` | Uint64 | amount      |

### GetUserUnstakeRate {.tabset}

Query user unstake rate.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserUnstakeRateResponse)]
    GetUserUnstakeRate { account: Addr },
}
```

#### JSON

```json
{
  "get_user_unstake_rate": {
    "account": "sei1..."
  }
}
```

| Key       | Type | Description  |
|-----------|------|--------------|
| `account` | Addr | user account |

### UserUnstakeRateResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UserUnstakeRateResponse {
    pub user_unstake_rate: Uint256,
}
```

#### JSON

```json
{
  "user_unstake_rate": "1000000000000000000"
}
```

| Key                 | Type    | Description |
|---------------------|---------|-------------|
| `user_unstake_rate` | Uint256 | amount      |

### GetUserLastWithdrawTime {.tabset}

Query user last withdraw time.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserLastWithdrawTimeResponse)]
    GetUserLastWithdrawTime { account: Addr },
}
```

#### JSON

```json
{
  "get_user_last_withdraw_time": {
    "account": "sei1..."
  }
}
```

| Key       | Type | Description  |
|-----------|------|--------------|
| `account` | Addr | user account |

### UserLastWithdrawTimeResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UserLastWithdrawTimeResponse {
    pub user_last_withdraw_time: Uint64,
}
```

#### JSON

```json
{
  "user_last_withdraw_time": "1000000000000000000"
}
```

| Key                       | Type   | Description |
|---------------------------|--------|-------------|
| `user_last_withdraw_time` | Uint64 | amount      |