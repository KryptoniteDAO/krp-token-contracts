use cosmwasm_std::{Binary, CosmosMsg, StdResult, Uint128, WasmMsg, to_json_binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Cw20ReceiveMsg should be de/serialized under `Receive()` variant in a ExecuteMsg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw20ReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}

impl Cw20ReceiveMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::Receive(self);
        to_json_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>>(self, contract_addr: T) -> StdResult<CosmosMsg> {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

// This is just a helper to properly serialize the above message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
enum ReceiverExecuteMsg {
    Receive(Cw20ReceiveMsg),
}
