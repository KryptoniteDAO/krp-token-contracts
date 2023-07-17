# KPT Tokenomics

kryptonite is an open-source DAO (Decentralized Autonomous Organization) project. kryptonite is managed by people around
the world who hold its governance token, KPT. Through a governance system involving Executive Voting and Governance
Polling, KPT holders can influence the direction of the protocol. The kryptonite (KPT) Token is the native token
powering the kryptonite Protocol. Its utility comprises all core network functionalities, such as staking, governance,
mint, and liquidators rewards. KPT is an ERC-20 governance token with a maximum supply of 100,000,000. KPT holders
manage the kryptonite Protocol and the financial risks of eUSD to ensure its stability, transparency, and efficiency.
KPT voting weight is proportional to the amount of KPT a voter stakes in the voting contract. In other words, the more
KPT tokens locked in the contract, the greater the voter’s decision-making power.

## Config

| Key| Type| Description                                                       |
| :--- | :--- |:------------------------------------------------------------------|
| `max_supply` | `u128` | KPT max supply                                                    |
| `kpt_fund` | `Addr` | KPT FUND module contract address (Possess mint permissions)       |
| `gov` | `Addr` | Address of contract owner that can update config                  |
| `kpt_distribute` | `Addr` | KPT DISTRIBUTE module contract address (Possess mint permissions) |

## InstantiateMsg {.tabset}

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub cw20_init_msg: Cw20InstantiateMsg,
    pub max_supply: u128,
    // default msg.sender
    pub gov: Option<Addr>,
}
```

### JSON

```json
{
  "cw20_init_msg": {
    "name": "Kryptonite",
    "symbol": "KPT",
    "decimals": 18,
    "initial_balances": [],
    "mint": null,
    "marketing": null
  },
  "max_supply": "100000000000000000000000000",
  "gov": null
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `cw20_init_msg` | `Cw20InstantiateMsg` | The cw20 initialization message structure based on the cw20_base library |
| `max_supply` | `u128` | KPT max supply |
| `gov` | `Addr`* | Address of contract owner that can update config. If not filled in, it is the initialization call address |

* = optional

## ExecuteMsg

### UpdateConfig {.tabset}

Updates the configuration of the contract. Can only be issued by the owner.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        kpt_fund: Option<Addr>,
        gov: Option<Addr>,
        kpt_distribute: Option<Addr>,
    }
}
```

#### JSON

```json
{
  "update_config": {
    "kpt_fund": null,
    "gov": null,
    "kpt_distribute": null
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `kpt_fund`* | `Addr` | KPT FUND module contract address (Possess mint permissions) |
| `gov`* | `Addr` | Address of contract owner that can update config |
| `kpt_distribute`* | `Addr` | KPT token contract address (Possess mint permissions) |

* = optional

### Mint {.tabset}

Only with the "mintable" extension. If authorized, creates amount new tokens and adds to the recipient balance.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        recipient: String,
        amount: Uint128
    }
}
```

#### JSON

```json
{
  "mint": {
    "recipient": "sei...",
    "amount": "100000000000000000000000000"
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `recipient` | `String` | Recipient address |
| `amount` | `Uint128` | Amount to mint |

### Burn {.tabset}

Burn is a base message to destroy tokens forever

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Burn {
        user: String,
        amount: Uint128
    }
}
```

#### JSON

```json
{
  "burn": {
    "user": "sei...",
    "amount": "100000000000000000000000000"
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `user` | `String` | User address |
| `amount` | `Uint128` | Amount to burn |

### TransferOwner {.tabset}

Transfer is a base message to move tokens to another account without triggering actions

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    TransferOwner {
        recipient: String,
        amount: Uint128
    }
}
```

#### JSON

```json
{
  "transfer_owner": {
    "recipient": "sei...",
    "amount": "100000000000000000000000000"
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `recipient` | `String` | Recipient address |
| `amount` | `Uint128` | Amount to transfer |

### Send {.tabset}

Send is a base message to transfer tokens to a contract and trigger an action on the receiving contract.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary
    }
}
```

#### JSON

```json
{
  "send": {
    "contract": "sei...",
    "amount": "100000000000000000000000000",
    "msg": "eyJh..."
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `contract` | `String` | Contract address |
| `amount` | `Uint128` | Amount to send |
| `msg` | `Binary` | Message to send |

### IncreaseAllowance {.tabset}

Only with "approval" extension. Allows spender to access an additional amount tokens from the owner's (env.sender)
account. If expires is Some(), overwrites current allowance expiration with this one.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>
    }
}
```

#### JSON

```json
{
  "increase_allowance": {
    "spender": "sei...",
    "amount": "100000000000000000000000000",
    "expires": null
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `spender` | `String` | Spender address |
| `amount` | `Uint128` | Amount to increase |
| `expires`* | `Expiration` | Expiration time |

* = optional

### DecreaseAllowance {.tabset}

Only with "approval" extension. Lowers the spender's access of tokens from the owner's (env.sender) account by amount.
If expires is Some(), overwrites current allowance expiration with this one.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>
    }
}
```

#### JSON

```json
{
  "decrease_allowance": {
    "spender": "sei...",
    "amount": "100000000000000000000000000",
    "expires": null
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `spender` | `String` | Spender address |
| `amount` | `Uint128` | Amount to decrease |
| `expires`* | `Expiration` | Expiration time |

* = optional

### TransferFrom {.tabset}

Only with "approval" extension. Transfers amount tokens from owner -> recipient if env.sender has sufficient
pre-approval.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128
    }
}
```

#### JSON

```json
{
  "transfer_from": {
    "owner": "sei...",
    "recipient": "sei...",
    "amount": "100000000000000000000000000"
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `owner` | `String` | Owner address |
| `recipient` | `String` | Recipient address |
| `amount` | `Uint128` | Amount to transfer |

### SendFrom {.tabset}

Only with "approval" extension. Sends amount tokens from owner -> contract if env.sender has sufficient pre-approval.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary
    }
}
```

```json
{
  "send_from": {
    "owner": "sei...",
    "contract": "sei...",
    "amount": "100000000000000000000000000",
    "msg": "eyJh..."
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `owner` | `String` | Owner address |
| `contract` | `String` | Contract address |
| `amount` | `Uint128` | Amount to send |
| `msg` | `Binary` | Message to send |

### BurnFrom {.tabset}

Only with "approval" extension. Destroys tokens forever

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    BurnFrom {
        owner: String,
        amount: Uint128
    }
}
```

#### JSON

```json
{
  "burn_from": {
    "owner": "sei...",
    "amount": "100000000000000000000000000"
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `owner` | `String` | Owner address |
| `amount` | `Uint128` | Amount to burn |

### UpdateMinter {.tabset}

Only with the "mintable" extension. The current minter may set a new minter. Setting the minter to None will remove the
token's minter forever.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateMinter {
        minter: Option<String>
    }
}
```

#### JSON

```json
{
  "update_minter": {
    "minter": null
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `minter` | `Option` | Minter address |

* = optional

### UpdateMarketing {.tabset}

Only with the "marketing" extension. If authorized, updates marketing metadata. Setting None/null for any of these will
leave it unchanged. Setting Some("") will clear this field on the contract storage

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateMarketing {
        project: Option<String>,
        description: Option<String>,
        marketing: Option<String>,
    }
}
```

#### JSON

```json
{
  "update_marketing": {
    "project": null,
    "description": null,
    "marketing": null
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `project` | `Option` | Project name |
| `description` | `Option` | Project description |
| `marketing` | `Option` | Marketing URL |

* = optional

### UploadLogo {.tabset}

If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UploadLogo {
        logo: String
    }
}
```

#### JSON

```json
{
  "upload_logo": {
    "logo": "https://..."
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `logo` | `String` | Logo URL |

## QueryMsg {.tabset}

KptConfig

Gets the KPT contract configuration.

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    KptConfig {}
}
```

#### JSON

```json
{
  "kpt_config": {}
}
```

| Key| Type| Description|
| :--- | :--- | :--- |

### KptConfigResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct KptConfigResponse {
    pub max_supply: u128,
    pub kpt_fund: Addr,
    pub kpt_distribute: Addr,
    pub gov: Addr,
}
```

#### JSON

```json
{
  "max_supply": "1000000000000000000000000000",
  "kpt_fund": "sei...",
  "kpt_distribute": "sei...",
  "gov": "sei..."
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `max_supply` | `u128` | Maximum supply |
| `kpt_fund` | `Addr` | KPT fund address |
| `kpt_distribute` | `Addr` | KPT distribute address |
| `gov` | `Addr` | Governance address |

### Balance {.tabset}

Returns the current balance of the given address, 0 if unset. Return type: BalanceResponse.

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    Balance {
        address: String
    }
}
```

#### JSON

```json
{
  "balance": {
    "address": "sei..."
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `address` | `String` | Address |

### BalanceResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}
```

#### JSON

```json
{
  "balance": "100000000000000000000000000"
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `balance` | `Uint128` | Balance |

### TokenInfo {.tabset}

Returns metadata on the contract - name, decimals, supply, etc. Return type: TokenInfoResponse

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    TokenInfo {}
}
```

#### JSON

```json
{
  "token_info": {}
}
```

| Key| Type| Description|
| :--- | :--- | :--- |

### TokenInfoResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}
```

#### JSON

```json
{
  "name": "KPT",
  "symbol": "KPT",
  "decimals": 18,
  "total_supply": "100000000000000000000000000"
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `name` | `String` | Name |
| `symbol` | `String` | Symbol |
| `decimals` | `u8` | Decimals |
| `total_supply` | `Uint128` | Total supply |

### Allowance {.tabset}

Only with "allowance" extension. Returns how much spender can use from owner account, 0 if unset. Return type:
AllowanceResponse.

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    Allowance {
        owner: String,
        spender: String
    }
}
```

#### JSON

```json
{
  "allowance": {
    "owner": "sei...",
    "spender": "sei..."
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `owner` | `String` | Owner address |
| `spender` | `String` | Spender address |

### AllowanceResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct AllowanceResponse {
    pub allowance: Uint128,
    pub expires: Expiration,
}
```

#### JSON

```json
{
  "allowance": "100000000000000000000000000",
  "expires": {
    "at_height": "0",
    "at_time": "0",
    "never": {}
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `allowance` | `Uint128` | Allowance |
| `expires` | `Expiration` | Expiration |

### Expiration {.tabset}

Expiration represents a point in time when some event happens. It can compare with a BlockInfo and will return
is_expired() == true once the condition is hit (and for every block in the future)

#### Rust

```rust
#[cw_serde]
#[derive(Copy)]
pub enum Expiration {
    /// AtHeight will expire when `env.block.height` >= height
    AtHeight(u64),
    /// AtTime will expire when `env.block.time` >= time
    AtTime(Timestamp),
    /// Never will never expire. Used to express the empty variant
    Never {},
}
```

#### JSON

```json
{
  "at_height": "0",
  "at_time": "0",
  "never": {}
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `at_height` | `u64` | AtHeight will expire when `env.block.height` >= height |
| `at_time` | `Timestamp` | AtTime will expire when `env.block.time` >= time |
| `never` | `Never` | Never will never expire. Used to express the empty variant |

### Minter {.tabset}

Only with "mintable" extension. Returns who can mint and the hard cap on maximum tokens after minting. Return type:
MinterResponse.

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    Minter {}
}
```

#### JSON

```json
{
  "minter": {}
}
```

| Key| Type| Description|
| :--- | :--- | :--- |

### MinterResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct MinterResponse {
    pub minter: Addr,
    pub cap: Option<Uint128>,
}
```

#### JSON

```json
{
  "minter": "sei...",
  "cap": "100000000000000000000000000"
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `minter` | `Addr` | Minter address |
| `cap` | `Option` | cap is a hard cap on total supply that can be achieved by minting. Note that this refers to total_supply. If None, there is unlimited cap. |

### MarketingInfo {.tabset}

Only with "marketing" extension Returns more metadata on the contract to display in the client:
> description, logo, project url, etc. Return type: MarketingInfoResponse.

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    MarketingInfo {}
}
```

#### JSON

```json
{
  "marketing_info": {}
}
```

| Key| Type| Description|
| :--- | :--- | :--- |

### MarketingInfoResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct MarketingInfoResponse {
    /// A URL pointing to the project behind this token.
    pub project: Option<String>,
    /// A longer description of the token and it's utility. Designed for tooltips or such
    pub description: Option<String>,
    /// A link to the logo, or a comment there is an on-chain logo stored
    pub logo: Option<LogoInfo>,
    /// The address (if any) who can update this data structure
    pub marketing: Option<Addr>,
}
```

#### JSON

```json
{
  "project": "https://...",
  "description": "Ku...",
  "logo": {
    "url": "logo",
    "embedded": "iVBORw0KGgoAAAANSUhEUgAA..."
  },
  "marketing": "sei..."
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `project` | `Option<String>` | A URL pointing to the project behind this token. |
| `description` | `Option<String>` | A longer description of the token and it's utility. Designed for tooltips or such |
| `logo` | `Option<String>` | A link to the logo, or a comment there is an on-chain logo stored |
| `marketing` | `Option<Addr>` | The address (if any) who can update this data structure |

### DownloadLogo {.tabset}

Only with "marketing" extension Downloads the embedded logo data (if stored on chain). Errors if no logo data stored for
this contract. Return type: DownloadLogoResponse.

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    DownloadLogo {}
}
```

#### JSON

```json
{
  "download_logo": {}
}
```

| Key| Type| Description|
| :--- | :--- | :--- |

### DownloadLogoResponse {.tabset}

When we download an embedded logo, we get this response type. We expect a SPA to be able to accept this info and display
it.

#### Rust

```rust
#[cw_serde]
pub struct DownloadLogoResponse {
    /// The mime type of the image
    pub mime_type: String,
    /// The raw bytes of the image
    pub data: Binary,
}
```

#### JSON

```json
{
  "mime_type": "image/png",
  "data": "iVBORw0KGgoAAAANSUhEUgAA..."
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `mime_type` | `String` | The mime type of the image |
| `data` | `Binary` | The raw bytes of the image |

### AllAllowances {.tabset}

Only with "enumerable" extension (and "allowances") Returns all allowances this owner has approved. Supports pagination.
Return type: AllAllowancesResponse.

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    }
}
```

#### JSON

```json
{
  "all_allowances": {
    "owner": "sei...",
    "start_after": "sei...",
    "limit": 10
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `owner` | `String` | The owner of the allowances |
| `start_after` | `Option<String>` | The address to start after, used for pagination |
| `limit` | `Option<u32>` | The number of allowances to limit the query to |

### AllAllowancesResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct AllAllowancesResponse {
    pub allowances: Vec<AllowanceInfo>,
}
```

#### JSON

```json
{
  "allowances": [
    {
      "spender": "sei...",
      "allowance": "100000000000000000000000000",
      "expires": {
        "at_height": "0",
        "at_time": "0",
        "never": {}
      }
    }
  ]
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `allowances` | `Vec<AllowanceInfo>` | The list of allowances |

### AllowanceInfo {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct AllowanceInfo {
    pub spender: Addr,
    pub allowance: Uint128,
    pub expires: Expiration,
}
```

#### JSON

```json
{
  "spender": "sei...",
  "allowance": "100000000000000000000000000",
  "expires": {
    "at_height": "0",
    "at_time": "0",
    "never": {}
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `spender` | `Addr` | The address of the spender |
| `allowance` | `Uint128` | The amount of tokens the spender is allowed to spend |
| `expires` | `Expiration` | When the allowance expires |

### AllAccounts {.tabset}

Only with "enumerable" extension Returns all accounts that have balances. Supports pagination. Return type: AllAccountsResponse.

#### Rust

```rust
#[cw_serde]
pub enum QueryMsg {
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    }
}
```

#### JSON

```json
{
  "all_accounts": {
    "start_after": "sei...",
    "limit": 10
  }
}
```

| Key| Type| Description|
| :--- | :--- | :--- |
| `start_after` | `Option<String>` | The address to start after, used for pagination |
| `limit` | `Option<u32>` | The number of accounts to limit the query to |



