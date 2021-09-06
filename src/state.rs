use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub uust: cosmwasm_std::Uint128,
    pub owner: Addr,
    pub interest: f64,
    pub timestamp: std::time::SystemTime,
    pub interest_borrow: Option<f64>,
    pub borrowed: Option<cosmwasm_std::Uint128>,
}

pub const STATE: Item<State> = Item::new("state");
