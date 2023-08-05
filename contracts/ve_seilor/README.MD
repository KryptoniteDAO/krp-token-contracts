# veKPT

veKPT is escrowed KPT. It has the same value as KPT and is subject to the total supply of KPT. veKPT cannot be traded or
transferred but has voting rights and can share in protocol earnings. Mining rewards are the primary source of veKPT.
veKPT holders can convert their veKPT to KPT through a vesting process. Once the process is started, veKPT will be
linearly converted to KPT over a period of 30 days.

## VoteConfig

The VoteConfig contract is used to manage the voting parameters of the KPT protocol.

| Key            | Type      | Description                                                 |
|----------------|-----------|-------------------------------------------------------------|
| `max_supply`   | `u128`    | veKPT max supply                                            |
| `kpt_fund`     | `Addr`    | KPT FUND module contract address (Possess mint permissions) |
| `gov`          | `Addr`    | Address of contract owner that can update config            |
| `max_minted`   | `Uint128` | Maximum amount of veKPT that can be minted                  |
| `total_minted` | `Uint128` | Total amount of veKPT that has been minted                  |

## InstantiateMsg {.tabset}

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub cw20_init_msg: Cw20InstantiateMsg,

    pub max_supply: u128,
    // default msg.sender
    pub gov: Option<Addr>,
    pub max_minted: u128,
}
```

### JSON

```json
{
  "cw20_init_msg": {
    "name": "kpt dev",
    "symbol": "kpt",
    "decimals": 6,
    "initial_balances": []
  },
  "max_supply": "650000000000000"
}
```

| Key             | Type                 | Description                                                                                               |
|:----------------|:---------------------|:----------------------------------------------------------------------------------------------------------|
| `cw20_init_msg` | `Cw20InstantiateMsg` | The cw20 initialization message structure based on the cw20_base library                                  |
| `max_supply`    | `u128`               | KPT max supply                                                                                            |
| `gov`           | `Addr`*              | Address of contract owner that can update config. If not filled in, it is the initialization call address |
| `max_minted`    | `Uint128`            | Maximum amount of veKPT that can be minted                                                                |

* = optional

## ExecuteMsg

### UpdateConfig {.tabset}

Updates the configuration of the contract. Can only be issued by the owner.

### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        max_minted: Option<Uint128>,
        kpt_fund: Option<Addr>,
        gov: Option<Addr>,
    }
}
```

### JSON

```json
{
  "update_config": {
    "max_minted": "650000000000000",
    "kpt_fund": "sei1...",
    "gov": "sei1..."
  }
}
```

| Key          | Type       | Description                                                                                               |
|:-------------|:-----------|:----------------------------------------------------------------------------------------------------------|
| `max_minted` | `Uint128`* | Maximum amount of veKPT that can be minted                                                                |
| `kpt_fund`   | `Addr`*    | KPT FUND module contract address (Possess mint permissions)                                               |
| `gov`        | `Addr`*    | Address of contract owner that can update config. If not filled in, it is the initialization call address |

* = optional

### SetMinters {.tabset}

Updates the minters of the contract. Can only be issued by the owner.

### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    SetMinters {
        contracts: Vec<Addr>,
        is_minter: Vec<bool>
    }
}
```

### JSON

```json
{
  "set_minters": {
    "contracts": [
      "sei1..."
    ],
    "is_minter": [
      true
    ]
  }
}
```

| Key         | Type        | Description                            |
|:------------|:------------|:---------------------------------------|
| `contracts` | `Vec<Addr>` | List of contracts to be updated        |
| `is_minter` | `Vec<bool>` | List of corresponding mint permissions |

### Mint {.tabset}

Mints veKPT. Can only be issued by the mint permissions.

### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        recipient: String,
        amount: Uint128,
    }
}
```

### JSON

```json
{
  "mint": {
    "recipient": "sei1...",
    "amount": "650000000000000"
  }
}
```

| Key         | Type      | Description             |
|:------------|:----------|:------------------------|
| `recipient` | `String`  | Recipient address       |
| `amount`    | `Uint128` | Amount of veKPT to mint |

### Burn {.tabset}

Burns veKPT. Can only be issued by the mint permissions.

### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Burn {
        user: String,
        amount: Uint128,
    }
}
```

### JSON

```json
{
  "burn": {
    "user": "sei1...",
    "amount": "650000000000000"
  }
}
```

| Key      | Type      | Description    |
|:---------|:----------|:---------------|
| `user`   | `String`  | User address   |
| `amount` | `Uint128` | Amount to burn |

### UpdateMarketing {.tabset}

Only with the "marketing" extension. If authorized, updates marketing metadata. Setting None/null for any of these will
leave it unchanged. Setting Some("") will clear this field on the contract storage

### Rust

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

| Key           | Type     | Description         |
|:--------------|:---------|:--------------------|
| `project`     | `Option` | Project name        |
| `description` | `Option` | Project description |
| `marketing`   | `Option` | Marketing URL       |

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

| Key    | Type     | Description |
|:-------|:---------|:------------|
| `logo` | `String` | Logo URL    |

## QueryMsg

### VoteConfig {.tabset}

Returns the current configuration of the contract.

### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(VoteConfigResponse)]
    VoteConfig {},
}
```

### JSON

```json
{
  "vote_config": {}
}
```

### VoteConfigResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct VoteConfigResponse {
    pub max_supply: u128,
    pub kpt_fund: Addr,
    pub gov: Addr,
    pub max_minted: Uint128,
    pub total_minted: Uint128,
}
```

#### JSON

```json
{
  "max_supply": "650000000000000",
  "kpt_fund": "sei1...",
  "gov": "sei1...",
  "max_minted": "650000000000000",
  "total_minted": "650000000000000"
}
```

| Key            | Type      | Description                                                                                               |
|:---------------|:----------|:----------------------------------------------------------------------------------------------------------|
| `max_supply`   | `u128`    | KPT max supply                                                                                            |
| `kpt_fund`     | `Addr`    | KPT FUND module contract address (Possess mint permissions)                                               |
| `gov`          | `Addr`    | Address of contract owner that can update config. If not filled in, it is the initialization call address |
| `max_minted`   | `Uint128` | Maximum amount of veKPT that can be minted                                                                |
| `total_minted` | `Uint128` | Total amount of veKPT minted                                                                              |

### IsMinter {.tabset}

Returns whether the given address is a minter.

### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(IsMinterResponse)]
    IsMinter { address: String },
}
```

### JSON

```json
{
  "is_minter": {
    "address": "sei1..."
  }
}
```

### IsMinterResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct IsMinterResponse {
    pub is_minter: bool,
}
```

#### JSON

```json
{
  "is_minter": true
}
```

| Key         | Type   | Description |
|:------------|:-------|:------------|
| `is_minter` | `bool` | Is minter   |

### Checkpoints {.tabset}

Returns the checkpoints of the given address and points.

### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CheckpointResponse)]
    Checkpoints { account: Addr, pos: u32 },
}
```

### JSON

```json
{
  "checkpoints": {
    "account": "sei1...",
    "pos": 0
  }
}
```

| Key       | Type   | Description |
|:----------|:-------|:------------|
| `account` | `Addr` | Account     |
| `pos`     | `u32`  | Position    |

### CheckpointResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct CheckpointResponse {
    pub from_block: u64,
    pub votes: u128,
}
```

#### JSON

```json
{
  "from_block": 0,
  "votes": "650000000000000"
}
```

| Key          | Type   | Description |
|:-------------|:-------|:------------|
| `from_block` | `u64`  | From block  |
| `votes`      | `u128` | Votes       |

### NumCheckpoints {.tabset}

Returns the number of checkpoints for the given address.

### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NumCheckpointsResponse)]
    NumCheckpoints { account: Addr },
}
```

### JSON

```json
{
  "num_checkpoints": {
    "account": "sei1..."
  }
}
```

| Key       | Type   | Description |
|:----------|:-------|:------------|
| `account` | `Addr` | Account     |

### NumCheckpointsResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct NumCheckpointsResponse {
    pub num: usize,
}
```

#### JSON

```json
{
  "num": 1
}
```

| Key   | Type   | Description |
|:------|:-------|:------------|
| `num` | `u128` | Number      |

### GetVotes {.tabset}

Returns the amount of votes the given address has.

### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetVotesResponse)]
    GetVotes { account: Addr },
}
```

### JSON

```json
{
  "get_votes": {
    "account": "sei1..."
  }
}
```

| Key       | Type   | Description |
|:----------|:-------|:------------|
| `account` | `Addr` | Account     |

### GetVotesResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetVotesResponse {
    pub votes: u128,
}
```

#### JSON

```json
{
  "votes": "650000000000000"
}
```

| Key     | Type   | Description |
|:--------|:-------|:------------|
| `votes` | `u128` | Votes       |

### GetPastVotes {.tabset}

Returns the amount of votes the given address had at the given block.

### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetPastVotesResponse)]
    GetPastVotes { account: Addr, block_number: u64 },
}
```

### JSON

```json
{
  "get_past_votes": {
    "account": "sei1...",
    "block_number": 0
  }
}
```

| Key            | Type   | Description  |
|:---------------|:-------|:-------------|
| `account`      | `Addr` | Account      |
| `block_number` | `u64`  | Block number |

### GetPastVotesResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetPastVotesResponse {
    pub votes: u128,
}
```

#### JSON

```json
{
  "votes": "650000000000000"
}
```

| Key     | Type   | Description |
|:--------|:-------|:------------|
| `votes` | `u128` | Votes       |

### GetPastTotalSupply {.tabset}

Returns the amount of total supply the given address had at the given block.

### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetPastTotalSupplyResponse)]
    GetPastTotalSupply { block_number: u64 },
}
```

### JSON

```json
{
  "get_past_total_supply": {
    "block_number": 0
  }
}
```

| Key            | Type  | Description  |
|:---------------|:------|:-------------|
| `block_number` | `u64` | Block number |

### GetPastTotalSupplyResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GetPastTotalSupplyResponse {
    pub total_supply: u128,
}
```

#### JSON

```json
{
  "total_supply": "650000000000000"
}
```

| Key            | Type   | Description  |
|:---------------|:-------|:-------------|
| `total_supply` | `u128` | Total supply |

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

| Key       | Type     | Description |
|:----------|:---------|:------------|
| `address` | `String` | Address     |

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

| Key       | Type      | Description |
|:----------|:----------|:------------|
| `balance` | `Uint128` | Balance     |

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

| Key | Type | Description |
|:----|:-----|:------------|

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

| Key            | Type      | Description  |
|:---------------|:----------|:-------------|
| `name`         | `String`  | Name         |
| `symbol`       | `String`  | Symbol       |
| `decimals`     | `u8`      | Decimals     |
| `total_supply` | `Uint128` | Total supply |

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

| Key | Type | Description |
|:----|:-----|:------------|

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

| Key      | Type     | Description                                                                                                                                |
|:---------|:---------|:-------------------------------------------------------------------------------------------------------------------------------------------|
| `minter` | `Addr`   | Minter address                                                                                                                             |
| `cap`    | `Option` | cap is a hard cap on total supply that can be achieved by minting. Note that this refers to total_supply. If None, there is unlimited cap. |

### AllAccounts {.tabset}

Only with "enumerable" extension Returns all accounts that have balances. Supports pagination. Return type:
AllAccountsResponse.

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

| Key           | Type             | Description                                     |
|:--------------|:-----------------|:------------------------------------------------|
| `start_after` | `Option<String>` | The address to start after, used for pagination |
| `limit`       | `Option<u32>`    | The number of accounts to limit the query to    |

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

| Key | Type | Description |
|:----|:-----|:------------|

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

| Key           | Type             | Description                                                                       |
|:--------------|:-----------------|:----------------------------------------------------------------------------------|
| `project`     | `Option<String>` | A URL pointing to the project behind this token.                                  |
| `description` | `Option<String>` | A longer description of the token and it's utility. Designed for tooltips or such |
| `logo`        | `Option<String>` | A link to the logo, or a comment there is an on-chain logo stored                 |
| `marketing`   | `Option<Addr>`   | The address (if any) who can update this data structure                           |

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

| Key | Type | Description |
|:----|:-----|:------------|

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

| Key         | Type     | Description                |
|:------------|:---------|:---------------------------|
| `mime_type` | `String` | The mime type of the image |
| `data`      | `Binary` | The raw bytes of the image |
