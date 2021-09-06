use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub uust: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Borrow {},
    //Increment {},
    //Reset { count: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    //GetCount {},
    CollateralBalance {},
    BorrowBalance {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralBalance {
    pub balance: cosmwasm_std::Uint128,
    pub timestamp: SystemTime,
    pub interest: f64,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BorrowBalance {
    pub balance: cosmwasm_std::Uint128,
    pub timestamp: SystemTime,
    pub interest: Option<f64>,
}
