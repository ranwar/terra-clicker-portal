use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    // Change `count` to whatever you want
    // pub count: i32,
    pub speed: i32,
    pub owner: Addr,
    // Here's the vector!
    pub scores: Vec<(Addr, u16)>,
}

// I've renamed this from `STATE` to `STORAGE` to avoid confusion lol
pub const STORAGE: Item<State> = Item::new("state");
