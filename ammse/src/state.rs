use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
static ESCROWS: Item<Addr> = Item::new("escrows:");
static POOL: Item<Pool> = Item::new("pool");
static COLLATERALS: Item<Addr> = Item::new("collaterals:");

//pub static ASSETS: Map<Addr, Asset> = Map::new("assets");
//
//pub struct Asset {
//    pub owner: Addr,    // Address of the owner.
//    pub unit: f64,               // Amount/Units of the asset.
//    pub interest: f64,           // Calculated interest for this asset.
//}
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Escrow {
    pub owner: Addr,
    pub funds: Coin,  // amount of tokens stored
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pool {
    pub liquidity: Coin,  // total tokens in the pool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Collateral {
    pub owner: Addr,
    pub amount: Coin,
}

// Execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddToEscrow { amount: Coin },
    BorrowFromPool { amount: Coin },
    AddCollateral { amount: Coin },
    // ... you can add more actions if needed
}
