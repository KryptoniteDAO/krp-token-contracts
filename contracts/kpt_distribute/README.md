# KPT Distribute

kpt token allocation contract, which contains linear release of locks and warehouses. Including: Community Offering, Liquidity, MM, Incentives, Community Rewards,
Team , Partners & Advisors, details to be determined.

## DistributeConfig

| Key                  | Type   | Description                     |
|----------------------|--------|---------------------------------|
| `gov`                | `Addr` | Governance contract address     |
| `total_amount`       | `u128` | Total amount of kpt             |
| `distribute_token`   | `Addr` | Token address to be distributed |
| `rules_total_amount` | `u128` | Total amount of rules           |

## InstantiateMsg

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub total_amount: u128,
    pub distribute_token: Addr,
    pub rule_configs_map: HashMap<String, RuleConfigMsg>,
}
```

### JSON

```json
{
  "gov": "sei1...",
  "total_amount": "1000000000000",
  "distribute_token": "sei1...",
  "rule_configs_map": {
    "key": {}
  }
}
```

| Key                | Type                             | Description                     |
|--------------------|----------------------------------|---------------------------------|
| `gov`              | `Addr`*                          | Governance contract address     |
| `total_amount`     | `u128`                           | Total amount of kpt             |
| `distribute_token` | `Addr`                           | Token address to be distributed |
| `rule_configs_map` | `HashMap<String, RuleConfigMsg>` | Rule configuration map          |

* = optional

## RuleConfigMsg

### Rust

```rust

#[cw_serde]
pub struct RuleConfigMsg {
    pub rule_name: String,
    pub rule_owner: Addr,
    pub rule_total_amount: u128,
    pub start_release_amount: u128,
    pub lock_start_time: u64,
    pub lock_end_time: u64,
    pub start_linear_release_time: u64,
    pub unlock_linear_release_amount: u128,
    pub unlock_linear_release_time: u64,
}
```

### JSON

```json
{
  "rule_name": "Test",
  "rule_owner": "sei1...",
  "rule_total_amount": "1000000000000",
  "start_release_amount": "0",
  "lock_start_time": "175052201",
  "lock_end_time": "185052201",
  "start_linear_release_time": "175052201",
  "unlock_linear_release_amount": "185052201",
  "unlock_linear_release_time": "185052201"
}
```

| Key                            | Type     | Description                  |
|--------------------------------|----------|------------------------------|
| `rule_name`                    | `String` | Rule name                    |
| `rule_owner`                   | `Addr`   | Rule owner                   |
| `rule_total_amount`            | `u128`   | Rule total amount            |
| `start_release_amount`         | `u128`   | Start release amount         |
| `lock_start_time`              | `u64`    | Lock start time              |
| `lock_end_time`                | `u64`    | Lock end time                |
| `start_linear_release_time`    | `u64`    | Start linear release time    |
| `unlock_linear_release_amount` | `u128`   | Unlock linear release amount |
| `unlock_linear_release_time`   | `u64`    | Unlock linear release time   |

## ExecuteMsg

### Claim {.tabset}

Claim the released kpt.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Claim {
        rule_type: String,
        msg: Option<Binary>,
    },
}
```

#### JSON

```json
{
  "claim": {
    "rule_type": "String",
    "msg": "Binary"
  }
}
```

| Key         | Type      | Description |
|-------------|-----------|-------------|
| `rule_type` | `String`  | Rule type   |
| `msg`       | `Binary`* | Message     |

* = optional

### UpdateConfig {.tabset}

Update the configuration of the contract.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        gov: Option<Addr>,
        distribute_token: Option<Addr>,
    },
}
```

#### JSON

```json
{
  "update_config": {
    "gov": "Addr",
    "distribute_token": "Addr"
  }
}
```

| Key                | Type    | Description                     |
|--------------------|---------|---------------------------------|
| `gov`              | `Addr`* | Governance contract address     |
| `distribute_token` | `Addr`* | Token address to be distributed |

* = optional

### UpdateRuleConfig {.tabset}

Update the configuration of the rule.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateRuleConfig {
        update_rule_msg: UpdateRuleConfigMsg,
    },
}
```

#### JSON

```json
{
  "update_rule_config": {
    "update_rule_msg": {}
  }
}
```

| Key               | Type                  | Description |
|-------------------|-----------------------|-------------|
| `update_rule_msg` | `UpdateRuleConfigMsg` |             |

### UpdateRuleConfigMsg {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UpdateRuleConfigMsg {
    pub rule_type: String,
    pub rule_name: Option<String>,
    pub rule_owner: Option<Addr>,
}
```

#### JSON

```json
{
  "rule_type": "String",
  "rule_name": "String",
  "rule_owner": "Addr"
}
```

| Key          | Type      | Description |
|--------------|-----------|-------------|
| `rule_type`  | `String`  | Rule type   |
| `rule_name`  | `String`* | Rule name   |
| `rule_owner` | `Addr`*   | Rule owner  |

* = optional

### AddRuleConfig {.tabset}

Add the configuration of the rule.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    AddRuleConfig {
        rule_type: String,
        rule_msg: RuleConfigMsg,
    },
}
```

#### JSON

```json
{
  "add_rule_config": {
    "rule_type": "String",
    "rule_msg": {}
  }
}
```

| Key         | Type             | Description |
|-------------|------------------|-------------|
| `rule_type` | `String`         | Rule type   |
| `rule_msg`  | `RuleConfigMsg`* | Rule config |

### RuleConfigMsg {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct RuleConfigMsg {
    pub rule_name: String,
    pub rule_owner: Addr,
    pub rule_total_amount: u128,
    pub start_release_amount: u128,
    pub lock_start_time: u64,
    pub lock_end_time: u64,
    pub start_linear_release_time: u64,
    pub unlock_linear_release_amount: u128,
    pub unlock_linear_release_time: u64,
}
```

#### JSON

```json
{
  "rule_name": "String",
  "rule_owner": "Addr",
  "rule_total_amount": "u128",
  "start_release_amount": "u128",
  "lock_start_time": "u64",
  "lock_end_time": "u64",
  "start_linear_release_time": "u64",
  "unlock_linear_release_amount": "u128",
  "unlock_linear_release_time": "u64"
}
```

| Key                            | Type     | Description                  |
|--------------------------------|----------|------------------------------|
| `rule_name`                    | `String` | Rule name                    |
| `rule_owner`                   | `Addr`   | Rule owner                   |
| `rule_total_amount`            | `u128`   | Rule total amount            |
| `start_release_amount`         | `u128`   | Start release amount         |
| `lock_start_time`              | `u64`    | Lock start time              |
| `lock_end_time`                | `u64`    | Lock end time                |
| `start_linear_release_time`    | `u64`    | Start linear release time    |
| `unlock_linear_release_amount` | `u128`   | Unlock linear release amount |
| `unlock_linear_release_time`   | `u64`    | Unlock linear release time   |

## QueryMsg

### QueryClaimableInfo {.tabset}

Query the claimable info.

#### Rust

```rust
#[cw_serde]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryClaimableInfoResponse)]
    QueryClaimableInfo { rule_type: String },
}
```

#### JSON

```json
{
  "query_claimable_info": {
    "rule_type": "String"
  }
}
```

| Key         | Type     | Description |
|-------------|----------|-------------|
| `rule_type` | `String` | Rule type   |

### QueryClaimableInfoResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct QueryClaimableInfoResponse {
    pub can_claim_amount: u128,
    pub release_amount: u128,
    pub linear_release_amount: u128,
}
```

#### JSON

```json
{
  "can_claim_amount": "u128",
  "release_amount": "u128",
  "linear_release_amount": "u128"
}
```

| Key                     | Type   | Description           |
|-------------------------|--------|-----------------------|
| `can_claim_amount`      | `u128` | Can claim amount      |
| `release_amount`        | `u128` | Release amount        |
| `linear_release_amount` | `u128` | Linear release amount |

### QueryRuleInfo {.tabset}

Query the rule info.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryRuleInfoResponse)]
    QueryRuleInfo { rule_type: String },
}
```

#### JSON

```json
{
  "query_rule_info": {
    "rule_type": "String"
  }
}
```

| Key         | Type     | Description |
|-------------|----------|-------------|
| `rule_type` | `String` | Rule type   |

### QueryRuleInfoResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct QueryRuleInfoResponse {
    pub rule_config: RuleConfig,
    pub rule_config_state: RuleConfigState,
}
```

#### JSON

```json
{
  "rule_config": {},
  "rule_config_state": {}
}
```

| Key                | Type             | Description      |
|--------------------|------------------|------------------|
| `rule_config`      | `RuleConfig`     | Rule config      |
| `rule_config_type` | `RuleConfigType` | Rule config type |

### RuleConfig {.tabset}

Query the rule config.

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RuleConfig {
    pub rule_name: String,
    pub rule_owner: Addr,
    pub rule_total_amount: u128,
    pub start_release_amount: u128,
    pub lock_start_time: u64,
    pub lock_end_time: u64,
    pub start_linear_release_time: u64,
    pub end_linear_release_time: u64,
    pub unlock_linear_release_amount: u128,
    pub unlock_linear_release_time: u64,
    pub linear_release_per_second: u128,
}
```

#### JSON

```json
{
  "rule_name": "String",
  "rule_owner": "Addr",
  "rule_total_amount": "u128",
  "start_release_amount": "u128",
  "lock_start_time": "u64",
  "lock_end_time": "u64",
  "start_linear_release_time": "u64",
  "end_linear_release_time": "u64",
  "unlock_linear_release_amount": "u128",
  "unlock_linear_release_time": "u64",
  "linear_release_per_second": "u128"
}
```

| Key                            | Type     | Description                  |
|--------------------------------|----------|------------------------------|
| `rule_name`                    | `String` | Rule name                    |
| `rule_owner`                   | `Addr`   | Rule owner                   |
| `rule_total_amount`            | `u128`   | Rule total amount            |
| `start_release_amount`         | `u128`   | Start release amount         |
| `lock_start_time`              | `u64`    | Lock start time              |
| `lock_end_time`                | `u64`    | Lock end time                |
| `start_linear_release_time`    | `u64`    | Start linear release time    |
| `end_linear_release_time`      | `u64`    | End linear release time      |
| `unlock_linear_release_amount` | `u128`   | Unlock linear release amount |
| `unlock_linear_release_time`   | `u64`    | Unlock linear release time   |
| `linear_release_per_second`    | `u128`   | Linear release per second    |

### RuleConfigState {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RuleConfigState {
    pub is_start_release: bool,
    pub released_amount: u128,
    pub claimed_amount: u128,
    pub last_claim_linear_release_time: u64,
}
```

#### JSON

```json
{
  "is_start_release": "bool",
  "released_amount": "u128",
  "claimed_amount": "u128",
  "last_claim_linear_release_time": "u64"
}
```

| Key                              | Type   | Description                    |
|----------------------------------|--------|--------------------------------|
| `is_start_release`               | `bool` | Is start release               |
| `released_amount`                | `u128` | Released amount                |
| `claimed_amount`                 | `u128` | Claimed amount                 |
| `last_claim_linear_release_time` | `u64`  | Last claim linear release time |

### QueryConfig {.tabset}

Query the config.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryConfigResponse)]
    QueryConfig {},
}
```

#### JSON

```json
{
  "query_config": {}
}
```

### QueryConfigResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct QueryConfigResponse {
    pub gov: Addr,
    pub total_amount: u128,
    pub distribute_token: Addr,
    pub rules_total_amount: u128,
}
```

#### JSON

```json
{
  "gov": "Addr",
  "total_amount": "u128",
  "distribute_token": "Addr",
  "rules_total_amount": "u128"
}
```

| Key                  | Type   | Description        |
|----------------------|--------|--------------------|
| `gov`                | `Addr` | Gov                |
| `total_amount`       | `u128` | Total amount       |
| `distribute_token`   | `Addr` | Distribute token   |
| `rules_total_amount` | `u128` | Rules total amount |
