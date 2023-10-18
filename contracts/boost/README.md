# veSEILOR Boost

veSEILOR holders receive a varied percentage of yield boost depending on the lock-up length. Details TBD.

## BoostConfig

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BoostConfig {
    pub gov: Addr,
    pub ve_seilor_lock_settings: Vec<VeSeilorLockSetting>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VeSeilorLockSetting {
    pub duration: Uint128,
    pub mining_boost: Uint128,
}
```

## InstantiateMsg  {.tabset}

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub ve_seilor_lock_settings: Vec<VeSeilorLockSetting>,
}
```

### JSON

```json
{
  "gov": "addr",
  "ve_seilor_lock_settings": [
    {
      "duration": "1000000000000000000",
      "mining_boost": "1000000000000000000"
    }
  ]
}
```


| Key                       | Type     | Description                             |
| ------------------------- | -------- | --------------------------------------- |
| `gov`                     | `string` | The address of the governance contract. |
| `ve_seilor_lock_settings` | `array`  | The list of veSEILOR lock settings.     |

## VeSeilorLockSetting {.tabset}

### Rust

```rust
#[cw_serde]
// Define a struct for the lock settings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VeSeilorLockSetting {
    pub duration: Uint128,
    pub mining_boost: Uint128,
}
```

### JSON

```json
{
  "duration": "1000000000000000000",
  "mining_boost": "1000000000000000000"
}
```


| Key            | Type     | Description               |
| -------------- | -------- | ------------------------- |
| `duration`     | `string` | The duration of the lock. |
| `mining_boost` | `string` | The boost percentage.     |

## ExecuteMsg

### AddLockSetting {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    AddLockSetting {
        duration: Uint128,
        mining_boost: Uint128,
    },
}
```

#### JSON

```json
{
  "add_lock_setting": {
    "duration": "1000000000000000000",
    "mining_boost": "1000000000000000000"
  }
}
```


| Key            | Type     | Description               |
| -------------- | -------- | ------------------------- |
| `duration`     | `string` | The duration of the lock. |
| `mining_boost` | `string` | The boost percentage.     |

### ChangeGov {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    ChangeGov {
        gov: Addr,
    },
}
```

#### JSON

```json
{
  "change_gov": {
    "gov": "addr"
  }
}
```


| Key   | Type     | Description                             |
| ----- | -------- | --------------------------------------- |
| `gov` | `string` | The address of the governance contract. |

### SetLockStatus {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    SetLockStatus {
        index: u32,
    },
}
```

#### JSON

```json
{
  "set_lock_status": {
    "index": 0
  }
}
```


| Key     | Type     | Description                              |
| ------- | -------- | ---------------------------------------- |
| `index` | `number` | The index of the lock setting to update. |

## QueryMsg

### GetUnlockTime {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetUnlockTimeResponse)]
    GetUnlockTime {
        user: Addr,
    },
}
```

#### JSON

```json
{
  "get_unlock_time": {
    "user": "addr"
  }
}
```


| Key    | Type     | Description              |
| ------ | -------- | ------------------------ |
| `user` | `string` | The address of the user. |

### GetUnlockTimeResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetUnlockTimeResponse {
    pub unlock_time: Uint128,
}
```

#### JSON

```json
{
  "unlock_time": "1000000000000000000"
}
```


| Key           | Type     | Description              |
| ------------- | -------- | ------------------------ |
| `unlock_time` | `string` | The unlock time of user. |

### GetUserLockStatus {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(LockStatusResponse)]
    GetUserLockStatus {
        user: Addr,
    },
}
```

#### JSON

```json
{
  "get_user_lock_status": {
    "user": "addr"
  }
}
```


| Key    | Type     | Description              |
| ------ | -------- | ------------------------ |
| `user` | `string` | The address of the user. |

### LockStatusResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct LockStatusResponse {
    pub unlock_time: Uint128,
    pub duration: Uint128,
    pub mining_boost: Uint128,
}
```

#### JSON

```json
{
  "unlock_time": "1000000000000000000",
  "duration": "1000000000000000000",
  "mining_boost": "1000000000000000000"
}
```


| Key            | Type     | Description               |
| -------------- | -------- | ------------------------- |
| `unlock_time`  | `string` | The unlock time of user.  |
| `duration`     | `string` | The duration of the lock. |
| `mining_boost` | `string` | The boost percentage.     |

### GetUserBoost {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetUserBoostResponse)]
    GetUserBoost {
        user: Addr,
        user_updated_at: Uint128,
        finish_at: Uint128,
    },
}
```

#### JSON

```json
{
  "get_user_boost": {
    "user": "sei1...",
    "user_updated_at": "1000000000000000000",
    "finish_at": "1000000000000000000"
  }
}
```


| Key               | Type     | Description                    |
| ----------------- | -------- | ------------------------------ |
| `user`            | `string` | The address of the user.       |
| `user_updated_at` | `string` | The last updated time of user. |
| `finish_at`       | `string` | The finish time of user.       |

### GetUserBoostResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetUserBoostResponse {
    pub user_boost: Uint128,
}
```

#### JSON

```json
{
  "user_boost": "1000000000000000000"
}
```


| Key          | Type     | Description           |
| ------------ | -------- | --------------------- |
| `user_boost` | `string` | The boost percentage. |

### GetBoostConfig {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetBoostConfigResponse)]
    GetBoostConfig {},
}
```

#### JSON

```json
{
  "get_boost_config": {}
}
```

### GetBoostConfigResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetBoostConfigResponse {
    pub gov: Addr,
    pub ve_seilor_lock_settings: Vec<VeSeilorLockSetting>,
}
```

#### JSON

```json
{
  "gov": "addr",
  "ve_seilor_lock_settings": [
    {
      "duration": "1000000000000000000",
      "mining_boost": "1000000000000000000"
    }
  ]
}
```


| Key                       | Type     | Description                             |
| ------------------------- | -------- | --------------------------------------- |
| `gov`                     | `string` | The address of the governance contract. |
| `ve_seilor_lock_settings` | `array`  | The array of the lock settings.         |
